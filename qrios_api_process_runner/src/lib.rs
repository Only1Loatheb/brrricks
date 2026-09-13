mod session_store;

use crate::session_store::{
  create_session_context_batch, create_session_context_table, delete_session_context_batch,
  get_session_step_context_batch, pop_session_step_context_for_back_navigation,
  update_session_step_context_for_back_navigation_batch,
  update_session_step_context_for_next_interaction_with_same_form_batch,
};
use async_trait::async_trait;
use qrios_api_axum_server::apis::ErrorHandler;
use qrios_api_axum_server::apis::developers_app_endpoints::{
  PostUssdsessioneventAbortResponse, PostUssdsessioneventCloseResponse, PostUssdsessioneventContinueResponse,
  PostUssdsessioneventNewResponse,
};
use qrios_api_axum_server::models::{
  AbortSession, CloseSession, ContinueSession, InfoView, InputView, PostUssdsessioneventAbortHeaderParams,
  PostUssdsessioneventCloseHeaderParams, PostUssdsessioneventContinueHeaderParams, PostUssdsessioneventNewHeaderParams,
  ShowView, UssdAction, UssdActionResult, UssdSessionCommand, UssdSessionEventNewSession,
  UssdSessionEventNewSessionSessionInput, UssdView,
};
use qrios_api_process_entry::{DialedSessionEntryParam, Msisdn, Operator, ShortcodeString};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use type_process_builder::back_navigation::create_back_token;
use type_process_builder::builder::{
  CurrentRunYieldedAt, FinalizedProcess, FormContext, MaybeFormContext, ParamList, PreviousRunYieldedAt, RunOutcome,
  RunnableProcess, SessionContext, StepIndex,
};
use type_process_builder::step::{BackToken, ProcessMessages};
use type_process_builder::{HList, hlist};
use uuid::Uuid;

pub struct Message(pub String);

pub struct Messages;
impl ProcessMessages for Messages {
  type FormMessage = Message;
  type FinalMessage = Message;
}

type EntryConsumes = HList!(DialedSessionEntryParam);

pub enum SessionBatchCommand {
  Create {
    id: Uuid,
    visited_form_step: i32,
    has_back: bool,
    form_context: MaybeFormContext,
    session_context: SessionContext,
    tx: oneshot::Sender<Result<(), sqlx::Error>>,
  },
  Get {
    id: Uuid,
    visited_form_step: i32,
    tx: oneshot::Sender<Result<(MaybeFormContext, SessionContext, bool), sqlx::Error>>,
  },
  Delete {
    id: Uuid,
    tx: oneshot::Sender<Result<u64, sqlx::Error>>,
  },
  UpdateNextInteraction {
    id: Uuid,
    visited_form_step: i32,
    form_context: Vec<u8>,
    tx: oneshot::Sender<Result<(), sqlx::Error>>,
  },
  UpdateBackNavigation {
    id: Uuid,
    visited_form_step: i32,
    form_context: MaybeFormContext,
    session_context: SessionContext,
    tx: oneshot::Sender<Result<(), sqlx::Error>>,
  },
}

fn spawn_session_batch_worker<Process: FinalizedProcess + 'static>(
  pool: PgPool,
  process: Arc<RunnableProcess<Process>>,
  mut rx: mpsc::Receiver<SessionBatchCommand>,
) {
  tokio::spawn(async move {
    let max_batch_size = 64;
    let batch_timeout = std::time::Duration::from_millis(5);

    loop {
      let Some(first_cmd) = rx.recv().await else { break };

      let mut commands = Vec::with_capacity(max_batch_size);
      commands.push(first_cmd);

      let deadline = tokio::time::sleep(batch_timeout);
      tokio::pin!(deadline);

      while commands.len() < max_batch_size {
        tokio::select! {
          cmd = rx.recv() => {
            match cmd {
              Some(c) => commands.push(c),
              None => break,
            }
          }
          () = &mut deadline => {
            break;
          }
        }
      }

      process_session_command_batch(&pool, &process, commands).await;
    }
  });
}

#[allow(clippy::too_many_lines, clippy::manual_let_else)]
async fn process_session_command_batch<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  commands: Vec<SessionBatchCommand>,
) {
  let mut creates = Vec::new();
  let mut gets = Vec::new();
  let mut deletes = Vec::new();
  let mut update_nexts = Vec::new();
  let mut update_backs = Vec::new();

  for cmd in commands {
    match cmd {
      SessionBatchCommand::Create { id, visited_form_step, has_back, form_context, session_context, tx } => {
        creates.push((id, visited_form_step, has_back, form_context, session_context, tx));
      },
      SessionBatchCommand::Get { id, visited_form_step, tx } => {
        gets.push((id, visited_form_step, tx));
      },
      SessionBatchCommand::Delete { id, tx } => {
        deletes.push((id, tx));
      },
      SessionBatchCommand::UpdateNextInteraction { id, visited_form_step, form_context, tx } => {
        update_nexts.push((id, visited_form_step, form_context, tx));
      },
      SessionBatchCommand::UpdateBackNavigation { id, visited_form_step, form_context, session_context, tx } => {
        update_backs.push((id, visited_form_step, form_context, session_context, tx));
      },
    }
  }

  if !creates.is_empty() {
    let mut ids = Vec::with_capacity(creates.len());
    let mut steps = Vec::with_capacity(creates.len());
    let mut backs = Vec::with_capacity(creates.len());
    let mut form_contexts = Vec::with_capacity(creates.len());
    let mut session_contexts = Vec::with_capacity(creates.len());
    let mut txs = Vec::with_capacity(creates.len());

    for (id, step, back, form_ctx, session_ctx, tx) in creates {
      ids.push(id);
      steps.push(step);
      backs.push(back);
      form_contexts.push(form_ctx);
      session_contexts.push(session_ctx);
      txs.push(tx);
    }

    let res =
      create_session_context_batch(pool, process, &ids, &steps, &backs, &form_contexts, &session_contexts).await;
    match res {
      Ok(()) => {
        for tx in txs {
          let _ = tx.send(Ok(()));
        }
      },
      Err(_) => {
        for tx in txs {
          let _ = tx.send(Err(sqlx::Error::PoolClosed));
        }
      },
    }
  }

  if !gets.is_empty() {
    let mut ids = Vec::with_capacity(gets.len());
    let mut steps = Vec::with_capacity(gets.len());
    let mut tx_map = HashMap::new();

    for (id, step, tx) in gets {
      ids.push(id);
      steps.push(step);
      tx_map.insert((id, step), tx);
    }

    let res = get_session_step_context_batch(pool, process, &ids, &steps).await;
    match res {
      Ok(results) => {
        for (id, step, form_ctx, session_ctx, has_back) in results {
          if let Some(tx) = tx_map.remove(&(id, step)) {
            let _ = tx.send(Ok((form_ctx, session_ctx, has_back)));
          }
        }
        for (_key, tx) in tx_map {
          let _ = tx.send(Err(sqlx::Error::RowNotFound));
        }
      },
      Err(_) => {
        for (_key, tx) in tx_map {
          let _ = tx.send(Err(sqlx::Error::PoolClosed));
        }
      },
    }
  }

  if !deletes.is_empty() {
    let mut ids = Vec::with_capacity(deletes.len());
    let mut txs = Vec::with_capacity(deletes.len());

    for (id, tx) in deletes {
      ids.push(id);
      txs.push(tx);
    }

    let res = delete_session_context_batch(pool, process, &ids).await;
    match res {
      Ok(affected) => {
        for tx in txs {
          let _ = tx.send(Ok(affected));
        }
      },
      Err(_) => {
        for tx in txs {
          let _ = tx.send(Err(sqlx::Error::PoolClosed));
        }
      },
    }
  }

  if !update_nexts.is_empty() {
    let mut ids = Vec::with_capacity(update_nexts.len());
    let mut steps = Vec::with_capacity(update_nexts.len());
    let mut form_contexts = Vec::with_capacity(update_nexts.len());
    let mut txs = Vec::with_capacity(update_nexts.len());

    for (id, step, form_ctx, tx) in update_nexts {
      ids.push(id);
      steps.push(step);
      form_contexts.push(form_ctx);
      txs.push(tx);
    }

    let res = update_session_step_context_for_next_interaction_with_same_form_batch(
      pool,
      process,
      &ids,
      &steps,
      &form_contexts,
    )
    .await;
    match res {
      Ok(()) => {
        for tx in txs {
          let _ = tx.send(Ok(()));
        }
      },
      Err(_) => {
        for tx in txs {
          let _ = tx.send(Err(sqlx::Error::PoolClosed));
        }
      },
    }
  }

  if !update_backs.is_empty() {
    let mut ids = Vec::with_capacity(update_backs.len());
    let mut steps = Vec::with_capacity(update_backs.len());
    let mut form_contexts = Vec::with_capacity(update_backs.len());
    let mut session_contexts = Vec::with_capacity(update_backs.len());
    let mut txs = Vec::with_capacity(update_backs.len());

    for (id, step, form_ctx, session_ctx, tx) in update_backs {
      ids.push(id);
      steps.push(step);
      form_contexts.push(form_ctx);
      session_contexts.push(session_ctx);
      txs.push(tx);
    }

    let res = update_session_step_context_for_back_navigation_batch(
      pool,
      process,
      &ids,
      &steps,
      &form_contexts,
      &session_contexts,
    )
    .await;
    match res {
      Ok(()) => {
        for tx in txs {
          let _ = tx.send(Ok(()));
        }
      },
      Err(_) => {
        for tx in txs {
          let _ = tx.send(Err(sqlx::Error::PoolClosed));
        }
      },
    }
  }
}

pub struct QriosUssdApiService<Process: FinalizedProcess<Messages = Messages, EntryConsumes = EntryConsumes>> {
  process: Arc<RunnableProcess<Process>>,
  pool: PgPool,
  batch_tx: mpsc::Sender<SessionBatchCommand>,
}

impl<Process: FinalizedProcess<Messages = Messages, EntryConsumes = EntryConsumes> + 'static>
  QriosUssdApiService<Process>
{
  pub async fn new(process: RunnableProcess<Process>, pool: PgPool) -> Result<Self, sqlx::Error> {
    create_session_context_table(&pool, &process).await?;
    let process = Arc::new(process);
    let (batch_tx, batch_rx) = mpsc::channel::<SessionBatchCommand>(1024);
    spawn_session_batch_worker(pool.clone(), process.clone(), batch_rx);
    Ok(QriosUssdApiService { process, pool, batch_tx })
  }

  async fn db_create_session_context(
    &self,
    id: Uuid,
    current_run_yielded_at: CurrentRunYieldedAt,
    has_back: bool,
    form_context: MaybeFormContext,
    session_context: SessionContext,
  ) -> Result<(), sqlx::Error> {
    let (tx, rx) = oneshot::channel();
    self
      .batch_tx
      .send(SessionBatchCommand::Create {
        id,
        visited_form_step: current_run_yielded_at.0,
        has_back,
        form_context,
        session_context,
        tx,
      })
      .await
      .map_err(|_| sqlx::Error::PoolClosed)?;
    rx.await.map_err(|_| sqlx::Error::PoolClosed)?
  }

  async fn db_get_session_step_context(
    &self,
    session_id: Uuid,
    visited_form_step: i32,
  ) -> Result<(MaybeFormContext, SessionContext, bool), sqlx::Error> {
    let (tx, rx) = oneshot::channel();
    self
      .batch_tx
      .send(SessionBatchCommand::Get { id: session_id, visited_form_step, tx })
      .await
      .map_err(|_| sqlx::Error::PoolClosed)?;
    rx.await.map_err(|_| sqlx::Error::PoolClosed)?
  }

  async fn db_delete_session_context(&self, id: Uuid) -> Result<u64, sqlx::Error> {
    let (tx, rx) = oneshot::channel();
    self.batch_tx.send(SessionBatchCommand::Delete { id, tx }).await.map_err(|_| sqlx::Error::PoolClosed)?;
    rx.await.map_err(|_| sqlx::Error::PoolClosed)?
  }

  async fn db_update_session_step_context_for_next_interaction_with_same_form(
    &self,
    id: Uuid,
    visited_form_step: i32,
    form_context: Vec<u8>,
  ) -> Result<(), sqlx::Error> {
    let (tx, rx) = oneshot::channel();
    self
      .batch_tx
      .send(SessionBatchCommand::UpdateNextInteraction { id, visited_form_step, form_context, tx })
      .await
      .map_err(|_| sqlx::Error::PoolClosed)?;
    rx.await.map_err(|_| sqlx::Error::PoolClosed)?
  }

  async fn db_update_session_step_context_for_back_navigation(
    &self,
    id: Uuid,
    visited_form_step: i32,
    form_context: MaybeFormContext,
    session_context: SessionContext,
  ) -> Result<(), sqlx::Error> {
    let (tx, rx) = oneshot::channel();
    self
      .batch_tx
      .send(SessionBatchCommand::UpdateBackNavigation { id, visited_form_step, form_context, session_context, tx })
      .await
      .map_err(|_| sqlx::Error::PoolClosed)?;
    rx.await.map_err(|_| sqlx::Error::PoolClosed)?
  }
}

impl<Process: FinalizedProcess<Messages = Messages, EntryConsumes = EntryConsumes> + 'static> ErrorHandler<()>
  for QriosUssdApiService<Process>
{
}

#[allow(unused_variables)]
#[async_trait]
impl<Process: FinalizedProcess<Messages = Messages, EntryConsumes = EntryConsumes> + Sync + 'static>
  qrios_api_axum_server::apis::developers_app_endpoints::DevelopersAppEndpoints for QriosUssdApiService<Process>
{
  async fn post_ussdsessionevent_abort(
    &self,
    method: &http::method::Method,
    host: &headers::Host,
    cookies: &axum_extra::extract::cookie::CookieJar,
    header_params: &PostUssdsessioneventAbortHeaderParams,
    body: &AbortSession,
  ) -> Result<PostUssdsessioneventAbortResponse, ()> {
    let session_id = uuid::Uuid::parse_str(&body.session_id).map_err(|_| ())?;
    self.db_delete_session_context(session_id).await.map_err(|_| ())?;
    Ok(PostUssdsessioneventAbortResponse::Status200_TheAbortingOfTheSessionHasBeenSuccessfullyHandledByTheDeveloper)
  }

  async fn post_ussdsessionevent_close(
    &self,
    method: &http::method::Method,
    host: &headers::Host,
    cookies: &axum_extra::extract::cookie::CookieJar,
    header_params: &PostUssdsessioneventCloseHeaderParams,
    body: &CloseSession,
  ) -> Result<PostUssdsessioneventCloseResponse, ()> {
    let session_id = uuid::Uuid::parse_str(&body.session_id).map_err(|_| ())?;
    self.db_delete_session_context(session_id).await.map_err(|_| ())?;
    Ok(PostUssdsessioneventCloseResponse::Status200_SessionEndHasBeenSuccessfullyHandledByTheDeveloper)
  }

  async fn post_ussdsessionevent_continue(
    &self,
    method: &http::method::Method,
    host: &headers::Host,
    cookies: &axum_extra::extract::cookie::CookieJar,
    header_params: &PostUssdsessioneventContinueHeaderParams,
    body: &ContinueSession,
  ) -> Result<PostUssdsessioneventContinueResponse, ()> {
    let user_input = match body.result {
      UssdActionResult::EmbeddedProcessResult(_) => todo!(),
      UssdActionResult::InputResult(ref input_result) => &input_result.value,
      UssdActionResult::MerchantPaymentResult(_) => todo!(),
      UssdActionResult::ReturnFromRedirectResult(_) => todo!(),
    };
    let session_id = uuid::Uuid::parse_str(&body.session_id).map_err(|_| ())?;
    let visited_form_step: i32 = body.context_data.parse().map_err(|_| ())?;
    let (form_context, session_context, has_back) =
      self.db_get_session_step_context(session_id, visited_form_step).await.map_err(|_| ())?;
    let previous_run_yielded_at = PreviousRunYieldedAt(visited_form_step);

    let mut run_result = {
      let back_token = has_back.then(create_back_token);
      self
        .process
        .resume_run(session_context, previous_run_yielded_at, user_input.clone(), form_context, back_token)
        .await
    };

    let mut is_back = false;
    if let Ok(RunOutcome::Back) = run_result {
      is_back = true;
      let (_target_form_context, target_step, target_session_context, has_back_target) =
        pop_session_step_context_for_back_navigation(&self.pool, &self.process, session_id, visited_form_step)
          .await
          .map_err(|_| ())?;
      let back_token = has_back_target.then(create_back_token);
      run_result = self
        .process
        .resume_run(
          target_session_context,
          PreviousRunYieldedAt(target_step),
          String::new(),
          None::<FormContext>,
          back_token,
        )
        .await;
    }

    let (ussd_view, yielded_step) = match run_result {
      Ok(RunOutcome::Yield(message, session_context, current_run_yielded_at, form_context)) => {
        let yielded_step = current_run_yielded_at.0;
        if is_back {
          self
            .db_update_session_step_context_for_back_navigation(
              session_id,
              yielded_step,
              Some(form_context),
              session_context,
            )
            .await
            .map_err(|_| ())?;
        } else {
          self
            .db_create_session_context(session_id, current_run_yielded_at, true, Some(form_context), session_context)
            .await
            .map_err(|_| ())?;
        }
        (UssdView::InputView(InputView { message: message.0, r_type: "InputView".into() }), yielded_step)
      },
      Ok(RunOutcome::RetryUserInput(message, form_context)) => {
        self
          .db_update_session_step_context_for_next_interaction_with_same_form(
            session_id,
            visited_form_step,
            form_context,
          )
          .await
          .map_err(|_| ())?;
        (UssdView::InputView(InputView { message: message.0, r_type: "InputView".into() }), visited_form_step)
      },
      Ok(RunOutcome::Finish(message)) => {
        self.db_delete_session_context(session_id).await.map_err(|_| ())?;
        (UssdView::InfoView(InfoView { message: message.0, r_type: "InfoView".into() }), 0)
      },
      Ok(RunOutcome::Back) => return Err(()),
      Err(e) => {
        tracing::error!("Resume session failed: {:?}", e);
        self.db_delete_session_context(session_id).await.map_err(|_| ())?;
        return Err(());
      },
    };

    Ok(PostUssdsessioneventContinueResponse::Status200_SessionContinuationHasBeenSuccessfullyHandledByTheDeveloper(
      UssdSessionCommand {
        action: UssdAction::ShowView(ShowView { r_type: "ShowView".into(), view: ussd_view }),
        context_data: yielded_step.to_string(),
        session_tag: None,
      },
    ))
  }

  async fn post_ussdsessionevent_new(
    &self,
    method: &http::method::Method,
    host: &headers::Host,
    cookies: &axum_extra::extract::cookie::CookieJar,
    header_params: &PostUssdsessioneventNewHeaderParams,
    body: &UssdSessionEventNewSession,
  ) -> Result<PostUssdsessioneventNewResponse, ()> {
    let session_id = uuid::Uuid::parse_str(&body.session_id).unwrap_or_else(|_| uuid::Uuid::new_v4());
    let shortcode_string = match body.input {
      UssdSessionEventNewSessionSessionInput::Dial(ref x) => &x.shortcode_string,
      UssdSessionEventNewSessionSessionInput::Push(_) => todo!(),
      UssdSessionEventNewSessionSessionInput::Redirect(_) => todo!(),
    };
    let operator = match &*body.operator {
      "mtn" => Operator::mtn,
      "airtel" => Operator::airtel,
      "glo" => Operator::glo,
      "etisalat" => Operator::etisalat,
      _ => Err(())?,
    };
    let entry_consumes: EntryConsumes = hlist!(DialedSessionEntryParam(
      Msisdn::from_string(&body.msisdn).ok_or(())?,
      operator,
      ShortcodeString(shortcode_string.clone())
    ));
    let run_result = self
      .process
      .resume_run(
        entry_consumes.serialize_param_list().map_err(|_| ())?,
        PreviousRunYieldedAt(StepIndex::MIN),
        String::new(),
        None::<FormContext>,
        None::<BackToken>,
      )
      .await;
    match run_result {
      Ok(RunOutcome::Yield(message, session_context, current_run_yielded_at, form_context)) => {
        let yielded_step = current_run_yielded_at.0;
        self
          .db_create_session_context(session_id, current_run_yielded_at, false, Some(form_context), session_context)
          .await
          .map_err(|_| ())?;
        Ok((yielded_step, UssdView::InputView(InputView { message: message.0, r_type: "InputView".into() })))
      },
      Ok(RunOutcome::RetryUserInput(..)) => {
        unreachable!("We haven't prompted user for input yet")
      },
      Ok(RunOutcome::Finish(message)) => {
        Ok((0, UssdView::InfoView(InfoView { message: message.0, r_type: "InfoView".into() })))
      },
      Ok(RunOutcome::Back) => Err(()),
      Err(e) => {
        tracing::error!("New session failed: {:?}", e);
        Err(())
      },
    }
    .map(|(yielded_step, ussd_view)| {
      PostUssdsessioneventNewResponse::Status200_SessionStartHasBeenSuccessfullyHandledByTheDeveloper(
        UssdSessionCommand {
          action: UssdAction::ShowView(ShowView { r_type: "ShowView".into(), view: ussd_view }),
          context_data: yielded_step.to_string(),
          session_tag: None,
        },
      )
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::{Message, Messages};
  use qrios_api_process_entry::DialedSessionEntryParam;
  use serde::{Deserialize, Serialize};
  use type_process_builder::builder::*;
  use type_process_builder::step::Final;
  use type_process_builder::step::*;
  use type_process_builder::{Coprod, HList, HNil, ToRef, hlist};
  use typenum::{U1, U2, U3, U4};

  #[allow(clippy::too_many_lines)]
  #[allow(clippy::unused_async_trait_impl)]
  #[tokio::test]
  async fn session_store_test() {
    use crate::QriosUssdApiService;
    use qrios_api_reqwest_client::Client;
    use std::sync::Arc;
    use tokio::net::TcpListener;

    #[derive(Deserialize, Serialize)]
    struct FormOutput;

    #[derive(Deserialize, Serialize)]
    struct OperationOutput;

    #[derive(Deserialize, Serialize)]
    struct SplitCase1Output;

    #[derive(Deserialize, Serialize)]
    struct SplitCase2Output;

    type_process_builder::impl_param_value! {
      FormOutput => U1,
      OperationOutput => U2,
      SplitCase1Output => U3,
      SplitCase2Output => U4,
    }

    struct ProduceParamOperation;
    impl Operation for ProduceParamOperation {
      type Consumes = HNil;
      type Produces = HList![OperationOutput];
      type FinalMessage = Message;

      async fn handle(
        &self,
        _consumes: <Self::Consumes as ToRef<'_>>::Ref,
      ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
        Ok(OperationOutcome::Successful(hlist!(OperationOutput)))
      }
    }

    struct AskForInputTwiceForm;
    impl Form for AskForInputTwiceForm {
      type CreateFormConsumes = HNil;
      type ValidateInputConsumes = HNil;
      type Produces = HList![FormOutput];
      type Context = u16;
      type Messages = Messages;

      async fn create_form(
        &self,
        _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
        _back_token: Option<BackToken>,
      ) -> anyhow::Result<FormWithContext<Message, Self::Context>> {
        Ok(FormWithContext(Message("This will be discarded".into()), 0))
      }

      async fn handle_input(
        &self,
        _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
        _user_input: String,
        failed: Self::Context,
        _back_token: Option<BackToken>,
      ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
        match failed {
          0 => Ok(InputValidation::Retry(Message("This will be accepted".into()), failed + 1)),
          _ => Ok(InputValidation::Successful(hlist![FormOutput])),
        }
      }
    }

    struct ConsumeCase1Final;
    impl Final for ConsumeCase1Final {
      type Consumes = HList![SplitCase1Output];
      type FinalMessage = Message;

      async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Message> {
        Ok(Message("Empty good bye".into()))
      }
    }

    struct ConsumeCase2Final;
    impl Final for ConsumeCase2Final {
      type Consumes = HList![SplitCase2Output];
      type FinalMessage = Message;

      async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Message> {
        Ok(Message("Empty good bye".into()))
      }
    }

    pub struct Case1;
    pub struct Case2;
    struct TestFormSplitter;
    impl FormSplitter for TestFormSplitter {
      type CreateFormConsumes = HNil;
      type ValidateInputConsumes = HNil;
      type Produces = Coprod![(Case1, HList![SplitCase1Output]), (Case2, HList![SplitCase2Output])];
      type Context = u16;
      type Messages = Messages;

      async fn create_form(
        &self,
        _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
        _back_token: Option<BackToken>,
      ) -> anyhow::Result<FormWithContext<Message, Self::Context>> {
        Ok(FormWithContext(Message("choose case".into()), 0))
      }

      async fn handle_input(
        &self,
        _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
        user_input: String,
        failed: u16,
        _back_token: Option<BackToken>,
      ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
        match (user_input.as_str(), failed) {
          ("retry", 0) => Ok(InputValidation::Retry(Message("retry again".into()), failed + 1)),
          ("finish", _) => Ok(InputValidation::Finish(Message("finished early".into()))),
          ("1", _) => Ok(InputValidation::Successful(Self::Produces::inject((Case1, hlist![SplitCase1Output])))),
          _ => Ok(InputValidation::Successful(Self::Produces::inject((Case2, hlist![SplitCase2Output])))),
        }
      }
    }

    let process = entry::<HList![DialedSessionEntryParam], Messages>()
      .show(AskForInputTwiceForm)
      .then(ProduceParamOperation)
      .show_split(TestFormSplitter)
      .case_end(Case1, |x| x.end(ConsumeCase1Final))
      .case_end(Case2, |x| x.end(ConsumeCase2Final))
      .build("test_process", 1);

    let node = {
      use testcontainers::runners::AsyncRunner;
      use testcontainers_modules::postgres::Postgres;
      Postgres::default().start().await.unwrap()
    };
    let _ = tracing_subscriber::fmt::try_init();
    let service = {
      use sqlx::PgPool;
      let pool = {
        let port = node.get_host_port_ipv4(5432).await.unwrap();
        let connection_string = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
        PgPool::connect(&connection_string).await.unwrap()
      };
      QriosUssdApiService::new(process, pool).await.expect("Failed to create service")
    };
    let app = qrios_api_axum_server::server::new(Arc::new(service));
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind random port");
    let addr = listener.local_addr().expect("Failed to get server local address");
    let server = tokio::spawn(async move {
      axum::serve(listener, app).await.expect("Failed to start server");
    });

    let client = Client::new(format!("http://{addr}").as_str());
    let test_session_id = uuid::Uuid::new_v4().to_string();

    let resp = client
      .post_ussdsessionevent_new(
        None,
        &qrios_api_reqwest_client::types::UssdSessionEventNewSession {
          app_id: "test_app".into(),
          client_id: "test_client".into(),
          input: qrios_api_reqwest_client::types::UssdSessionEventNewSessionSessionInput::Dial(
            qrios_api_reqwest_client::types::Dial {
              type_: qrios_api_reqwest_client::types::DialType::Dial,
              shortcode_string: "*123#".to_string(),
            },
          ),
          msisdn: "2341234567890".into(),
          operator: qrios_api_reqwest_client::types::UssdSessionEventNewSessionOperator::Mtn,
          session_id: test_session_id.clone(),
        },
      )
      .await
      .expect("Failed to get a response from post_ussdsessionevent_new");

    match &resp.action {
      qrios_api_reqwest_client::types::UssdAction::ShowView(qrios_api_reqwest_client::types::ShowView {
        view:
          qrios_api_reqwest_client::types::UssdView::InputView(qrios_api_reqwest_client::types::InputView {
            message, ..
          }),
        ..
      }) => {
        assert_eq!(message, "This will be discarded");
      },
      _ => panic!("Expected InputView, got {:?}", resp.action),
    }

    let resp = client
      .post_ussdsessionevent_continue(
        None,
        &qrios_api_reqwest_client::types::ContinueSession {
          app_id: "test_app".into(),
          client_id: "test_client".into(),
          context_data: resp.context_data.clone(),
          result: qrios_api_reqwest_client::types::UssdActionResult::InputResult(
            qrios_api_reqwest_client::types::InputResult {
              type_: qrios_api_reqwest_client::types::InputResultType::InputResult,
              value: "some input".into(),
            },
          ),
          session_id: test_session_id.clone(),
        },
      )
      .await
      .expect("Failed to get a response from post_ussdsessionevent_continue (1)");

    match &resp.action {
      qrios_api_reqwest_client::types::UssdAction::ShowView(qrios_api_reqwest_client::types::ShowView {
        view:
          qrios_api_reqwest_client::types::UssdView::InputView(qrios_api_reqwest_client::types::InputView {
            message, ..
          }),
        ..
      }) => {
        assert_eq!(message, "This will be accepted");
      },
      _ => panic!("Expected InputView (Retry), got {:?}", resp.action),
    }

    let resp = client
      .post_ussdsessionevent_continue(
        None,
        &qrios_api_reqwest_client::types::ContinueSession {
          app_id: "test_app".into(),
          client_id: "test_client".into(),
          context_data: resp.context_data.clone(),
          result: qrios_api_reqwest_client::types::UssdActionResult::InputResult(
            qrios_api_reqwest_client::types::InputResult {
              type_: qrios_api_reqwest_client::types::InputResultType::InputResult,
              value: "some input 2".into(),
            },
          ),
          session_id: test_session_id.clone(),
        },
      )
      .await
      .expect("Failed to get a response from post_ussdsessionevent_continue (2)");

    match &resp.action {
      qrios_api_reqwest_client::types::UssdAction::ShowView(qrios_api_reqwest_client::types::ShowView {
        view:
          qrios_api_reqwest_client::types::UssdView::InputView(qrios_api_reqwest_client::types::InputView {
            message, ..
          }),
        ..
      }) => {
        assert_eq!(message, "choose case");
      },
      _ => panic!("Expected InputView (FinishAfterInput), got {:?}", resp.action),
    }

    let resp = client
      .post_ussdsessionevent_continue(
        None,
        &qrios_api_reqwest_client::types::ContinueSession {
          app_id: "test_app".into(),
          client_id: "test_client".into(),
          context_data: resp.context_data.clone(),
          result: qrios_api_reqwest_client::types::UssdActionResult::InputResult(
            qrios_api_reqwest_client::types::InputResult {
              type_: qrios_api_reqwest_client::types::InputResultType::InputResult,
              value: "final input".into(),
            },
          ),
          session_id: test_session_id.clone(),
        },
      )
      .await
      .expect("Failed to get a response from post_ussdsessionevent_continue (3)");

    match &resp.action {
      qrios_api_reqwest_client::types::UssdAction::ShowView(qrios_api_reqwest_client::types::ShowView {
        view:
          qrios_api_reqwest_client::types::UssdView::InfoView(qrios_api_reqwest_client::types::InfoView {
            message, ..
          }),
        ..
      }) => {
        assert_eq!(message, "Empty good bye");
      },
      _ => panic!("Expected InfoView (Finish), got {:?}", resp.action),
    }

    server.abort();
  }

  #[allow(clippy::too_many_lines)]
  #[allow(clippy::unused_async_trait_impl)]
  #[tokio::test]
  async fn test_back_into_branch_after_implicit_merge_purge_postgres() {
    use crate::QriosUssdApiService;
    use qrios_api_reqwest_client::Client;
    use std::sync::Arc;
    use tokio::net::TcpListener;
    #[allow(dead_code)]
    #[derive(serde::Deserialize, serde::Serialize)]
    struct EntryParam(pub u64);
    #[derive(serde::Deserialize, serde::Serialize)]
    struct CaseOptionParam(pub u8);
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Branch1Param(pub u64);
    #[derive(serde::Deserialize, serde::Serialize)]
    struct SharedParam(pub u64);

    type_process_builder::impl_param_value! {
      EntryParam => typenum::U0,
      CaseOptionParam => typenum::U1,
      Branch1Param => typenum::U2,
      SharedParam => typenum::U3,
    }

    struct ChooseCaseForm;
    impl Form for ChooseCaseForm {
      type CreateFormConsumes = HNil;
      type ValidateInputConsumes = HNil;
      type Produces = HList![CaseOptionParam];
      type Context = ();
      type Messages = Messages;

      async fn create_form(
        &self,
        _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
        _back_token: Option<BackToken>,
      ) -> anyhow::Result<FormWithContext<Message, ()>> {
        Ok(FormWithContext(Message("Choose a case".into()), ()))
      }

      async fn handle_input(
        &self,
        _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
        input: String,
        _context: (),
        _back_token: Option<BackToken>,
      ) -> anyhow::Result<InputValidation<Self::Produces, Messages, ()>> {
        let option = input.parse::<u8>().unwrap_or(1);
        Ok(InputValidation::Successful(hlist!(CaseOptionParam(option))))
      }
    }

    pub struct Case1;
    pub struct Case2;

    struct SelectCase;
    impl Splitter for SelectCase {
      type Consumes = HList![CaseOptionParam];
      type Produces = Coprod![(Case1, HNil), (Case2, HNil)];

      async fn handle(&self, consumes: <Self::Consumes as ToRef<'_>>::Ref) -> anyhow::Result<Self::Produces> {
        Ok(match consumes.head.0 {
          1 => Self::Produces::inject((Case1, HNil)),
          _ => Self::Produces::inject((Case2, HNil)),
        })
      }
    }

    struct ProduceBranch1Data;
    impl Operation for ProduceBranch1Data {
      type Consumes = HNil;
      type Produces = HList![Branch1Param, SharedParam];
      type FinalMessage = Message;

      async fn handle(
        &self,
        _consumes: <Self::Consumes as ToRef<'_>>::Ref,
      ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
        Ok(OperationOutcome::Successful(hlist!(Branch1Param(0x1111), SharedParam(0x9999))))
      }
    }

    struct ProduceBranch2Data;
    impl Operation for ProduceBranch2Data {
      type Consumes = HNil;
      type Produces = HList![SharedParam];
      type FinalMessage = Message;

      async fn handle(
        &self,
        _consumes: <Self::Consumes as ToRef<'_>>::Ref,
      ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
        Ok(OperationOutcome::Successful(hlist!(SharedParam(0x9999))))
      }
    }

    struct Branch1Form;
    impl Form for Branch1Form {
      type CreateFormConsumes = HList![Branch1Param];
      type ValidateInputConsumes = HNil;
      type Produces = HNil;
      type Context = ();
      type Messages = Messages;

      async fn create_form(
        &self,
        consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
        _back_token: Option<BackToken>,
      ) -> anyhow::Result<FormWithContext<Message, ()>> {
        Ok(FormWithContext(Message(format!("Branch 1: {:#X}", consumes.head.0)), ()))
      }

      async fn handle_input(
        &self,
        _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
        input: String,
        _context: (),
        back_token: Option<BackToken>,
      ) -> anyhow::Result<InputValidation<HNil, Messages, ()>> {
        if input == "0"
          && let Some(t) = back_token
        {
          Ok(InputValidation::Back(t))
        } else {
          Ok(InputValidation::Successful(HNil))
        }
      }
    }

    struct PostMergeForm;
    impl Form for PostMergeForm {
      type CreateFormConsumes = HList![SharedParam];
      type ValidateInputConsumes = HNil;
      type Produces = HNil;
      type Context = ();
      type Messages = Messages;

      async fn create_form(
        &self,
        consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
        _back_token: Option<BackToken>,
      ) -> anyhow::Result<FormWithContext<Message, ()>> {
        Ok(FormWithContext(Message(format!("Post merge: {:#X}", consumes.head.0)), ()))
      }

      async fn handle_input(
        &self,
        _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
        input: String,
        _context: (),
        back_token: Option<BackToken>,
      ) -> anyhow::Result<InputValidation<HNil, Messages, ()>> {
        if input == "0"
          && let Some(t) = back_token
        {
          Ok(InputValidation::Back(t))
        } else {
          Ok(InputValidation::Successful(HNil))
        }
      }
    }

    struct FinalStep;
    impl Final for FinalStep {
      type Consumes = HNil;
      type FinalMessage = Message;

      async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Message> {
        Ok(Message("Done".into()))
      }
    }

    let process = entry::<HList![DialedSessionEntryParam], Messages>()
      .show(ChooseCaseForm)
      .split(SelectCase)
      .case_via(Case1, |x| x.then(ProduceBranch1Data).show(Branch1Form))
      .case_via(Case2, |x| x.then(ProduceBranch2Data))
      .show(PostMergeForm)
      .end(FinalStep)
      .build("split_back_purge_process", 1);

    let node = {
      use testcontainers::runners::AsyncRunner;
      use testcontainers_modules::postgres::Postgres;
      Postgres::default().start().await.unwrap()
    };
    let service = {
      use sqlx::PgPool;
      let pool = {
        let port = node.get_host_port_ipv4(5432).await.unwrap();
        let connection_string = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
        PgPool::connect(&connection_string).await.unwrap()
      };
      QriosUssdApiService::new(process, pool).await.expect("Failed to create service")
    };
    let app = qrios_api_axum_server::server::new(Arc::new(service));
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind random port");
    let addr = listener.local_addr().expect("Failed to get server local address");
    let server = tokio::spawn(async move {
      axum::serve(listener, app).await.expect("Failed to start server");
    });

    let client = Client::new(format!("http://{addr}").as_str());
    let test_session_id = uuid::Uuid::new_v4().to_string();

    // 1. New Session -> Choose Case
    let resp = client
      .post_ussdsessionevent_new(
        None,
        &qrios_api_reqwest_client::types::UssdSessionEventNewSession {
          app_id: "test_app".into(),
          client_id: "test_client".into(),
          input: qrios_api_reqwest_client::types::UssdSessionEventNewSessionSessionInput::Dial(
            qrios_api_reqwest_client::types::Dial {
              type_: qrios_api_reqwest_client::types::DialType::Dial,
              shortcode_string: "*123#".to_string(),
            },
          ),
          msisdn: "2341234567890".into(),
          operator: qrios_api_reqwest_client::types::UssdSessionEventNewSessionOperator::Mtn,
          session_id: test_session_id.clone(),
        },
      )
      .await
      .expect("Failed to start new session");

    assert!(matches!(
      &resp.action,
      qrios_api_reqwest_client::types::UssdAction::ShowView(qrios_api_reqwest_client::types::ShowView {
        view: qrios_api_reqwest_client::types::UssdView::InputView(qrios_api_reqwest_client::types::InputView { message, .. }),
        ..
      }) if message == "Choose a case"
    ));

    // 2. Select Case 1 -> Branch 1 Form
    let resp = client
      .post_ussdsessionevent_continue(
        None,
        &qrios_api_reqwest_client::types::ContinueSession {
          app_id: "test_app".into(),
          client_id: "test_client".into(),
          context_data: resp.context_data.clone(),
          result: qrios_api_reqwest_client::types::UssdActionResult::InputResult(
            qrios_api_reqwest_client::types::InputResult {
              type_: qrios_api_reqwest_client::types::InputResultType::InputResult,
              value: "1".into(),
            },
          ),
          session_id: test_session_id.clone(),
        },
      )
      .await
      .expect("Failed continue 1");

    assert!(matches!(
      &resp.action,
      qrios_api_reqwest_client::types::UssdAction::ShowView(qrios_api_reqwest_client::types::ShowView {
        view: qrios_api_reqwest_client::types::UssdView::InputView(qrios_api_reqwest_client::types::InputView { message, .. }),
        ..
      }) if message == "Branch 1: 0x1111"
    ));

    // 3. Continue from Branch 1 Form -> Post Merge Form
    let resp = client
      .post_ussdsessionevent_continue(
        None,
        &qrios_api_reqwest_client::types::ContinueSession {
          app_id: "test_app".into(),
          client_id: "test_client".into(),
          context_data: resp.context_data.clone(),
          result: qrios_api_reqwest_client::types::UssdActionResult::InputResult(
            qrios_api_reqwest_client::types::InputResult {
              type_: qrios_api_reqwest_client::types::InputResultType::InputResult,
              value: "next".into(),
            },
          ),
          session_id: test_session_id.clone(),
        },
      )
      .await
      .expect("Failed continue 2");

    assert!(matches!(
      &resp.action,
      qrios_api_reqwest_client::types::UssdAction::ShowView(qrios_api_reqwest_client::types::ShowView {
        view: qrios_api_reqwest_client::types::UssdView::InputView(qrios_api_reqwest_client::types::InputView { message, .. }),
        ..
      }) if message == "Post merge: 0x9999"
    ));

    // 4. Send "0" (Back) at Post Merge Form -> should navigate back to Branch 1 Form with restored session_context!
    let resp = client
      .post_ussdsessionevent_continue(
        None,
        &qrios_api_reqwest_client::types::ContinueSession {
          app_id: "test_app".into(),
          client_id: "test_client".into(),
          context_data: resp.context_data.clone(),
          result: qrios_api_reqwest_client::types::UssdActionResult::InputResult(
            qrios_api_reqwest_client::types::InputResult {
              type_: qrios_api_reqwest_client::types::InputResultType::InputResult,
              value: "0".into(),
            },
          ),
          session_id: test_session_id.clone(),
        },
      )
      .await
      .expect("Failed continue back");

    assert!(matches!(
      &resp.action,
      qrios_api_reqwest_client::types::UssdAction::ShowView(qrios_api_reqwest_client::types::ShowView {
        view: qrios_api_reqwest_client::types::UssdView::InputView(qrios_api_reqwest_client::types::InputView { message, .. }),
        ..
      }) if message == "Branch 1: 0x1111"
    ));

    server.abort();
  }
}
