mod session_store;

use crate::session_store::{
  GetSessionContextQuery, build_get_session_context_query, create_session_context_batch, create_session_context_table,
  delete_session_context_batch, get_session_context_batch, increment_failed_input_validation_attempts_batch,
  update_session_context_batch,
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
use std::ops::Not;
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

use std::sync::Arc;

pub enum SessionBatchCommand {
  Create {
    id: Uuid,
    current_run_yielded_at: CurrentRunYieldedAt,
    form_context: MaybeFormContext,
    session_context: SessionContext,
    tx: oneshot::Sender<Result<(), sqlx::Error>>,
  },
  Get {
    id: Uuid,
    tx: oneshot::Sender<Result<(MaybeFormContext, Vec<i32>, SessionContext), sqlx::Error>>,
  },
  Update {
    id: Uuid,
    form_context: MaybeFormContext,
    visited_form_steps: Vec<i32>,
    session_context: SessionContext,
    tx: oneshot::Sender<Result<(), sqlx::Error>>,
  },
  Delete {
    id: Uuid,
    tx: oneshot::Sender<Result<u64, sqlx::Error>>,
  },
  IncrementValidation {
    id: Uuid,
    form_context: Vec<u8>,
    tx: oneshot::Sender<Result<u64, sqlx::Error>>,
  },
}

fn spawn_session_batch_worker<Process: FinalizedProcess + 'static>(
  pool: PgPool,
  process: Arc<RunnableProcess<Process>>,
  get_query: GetSessionContextQuery,
  mut rx: mpsc::Receiver<SessionBatchCommand>,
) {
  tokio::spawn(async move {
    let max_batch_size = 64;
    let batch_timeout = std::time::Duration::from_millis(5);

    loop {
      let first_cmd = match rx.recv().await {
        Some(cmd) => cmd,
        None => break,
      };

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

      process_session_command_batch(&pool, &process, &get_query, commands).await;
    }
  });
}

async fn process_session_command_batch<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  get_query: &GetSessionContextQuery,
  commands: Vec<SessionBatchCommand>,
) {
  let mut creates = Vec::new();
  let mut gets = Vec::new();
  let mut updates = Vec::new();
  let mut deletes = Vec::new();
  let mut increments = Vec::new();

  for cmd in commands {
    match cmd {
      SessionBatchCommand::Create { id, current_run_yielded_at, form_context, session_context, tx } => {
        creates.push((id, current_run_yielded_at, form_context, session_context, tx));
      },
      SessionBatchCommand::Get { id, tx } => {
        gets.push((id, tx));
      },
      SessionBatchCommand::Update { id, form_context, visited_form_steps, session_context, tx } => {
        updates.push((id, form_context, visited_form_steps, session_context, tx));
      },
      SessionBatchCommand::Delete { id, tx } => {
        deletes.push((id, tx));
      },
      SessionBatchCommand::IncrementValidation { id, form_context, tx } => {
        increments.push((id, form_context, tx));
      },
    }
  }

  if !creates.is_empty() {
    let mut ids = Vec::with_capacity(creates.len());
    let mut yielded_ats = Vec::with_capacity(creates.len());
    let mut form_contexts = Vec::with_capacity(creates.len());
    let mut session_contexts = Vec::with_capacity(creates.len());
    let mut txs = Vec::with_capacity(creates.len());

    for (id, yielded_at, form_ctx, session_ctx, tx) in creates {
      ids.push(id);
      yielded_ats.push(yielded_at);
      form_contexts.push(form_ctx);
      session_contexts.push(session_ctx);
      txs.push(tx);
    }

    let res = create_session_context_batch(pool, process, &ids, &yielded_ats, &form_contexts, &session_contexts).await;
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
    let mut tx_map = HashMap::new();

    for (id, tx) in gets {
      ids.push(id);
      tx_map.insert(id, tx);
    }

    let res = get_session_context_batch(pool, get_query, &ids).await;
    match res {
      Ok(results) => {
        for (id, form_ctx, visited_steps, session_ctx) in results {
          if let Some(tx) = tx_map.remove(&id) {
            let _ = tx.send(Ok((form_ctx, visited_steps, session_ctx)));
          }
        }
        for (_id, tx) in tx_map {
          let _ = tx.send(Err(sqlx::Error::RowNotFound));
        }
      },
      Err(_) => {
        for (_id, tx) in tx_map {
          let _ = tx.send(Err(sqlx::Error::PoolClosed));
        }
      },
    }
  }

  if !updates.is_empty() {
    let mut ids = Vec::with_capacity(updates.len());
    let mut form_contexts = Vec::with_capacity(updates.len());
    let mut visited_steps_list = Vec::with_capacity(updates.len());
    let mut session_contexts = Vec::with_capacity(updates.len());
    let mut txs = Vec::with_capacity(updates.len());

    for (id, form_ctx, visited_steps, session_ctx, tx) in updates {
      ids.push(id);
      form_contexts.push(form_ctx);
      visited_steps_list.push(visited_steps);
      session_contexts.push(session_ctx);
      txs.push(tx);
    }

    let res =
      update_session_context_batch(pool, process, &ids, &form_contexts, &visited_steps_list, &session_contexts).await;
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

  if !increments.is_empty() {
    let mut ids = Vec::with_capacity(increments.len());
    let mut form_contexts = Vec::with_capacity(increments.len());
    let mut txs = Vec::with_capacity(increments.len());

    for (id, form_ctx, tx) in increments {
      ids.push(id);
      form_contexts.push(form_ctx);
      txs.push(tx);
    }

    let res = increment_failed_input_validation_attempts_batch(pool, process, &ids, &form_contexts).await;
    match res {
      Ok(qr) => {
        for tx in txs {
          let _ = tx.send(Ok(qr.rows_affected()));
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
  get_session_context_query: GetSessionContextQuery,
  batch_tx: mpsc::Sender<SessionBatchCommand>,
}

impl<Process: FinalizedProcess<Messages = Messages, EntryConsumes = EntryConsumes> + 'static>
  QriosUssdApiService<Process>
{
  pub async fn new(process: RunnableProcess<Process>, pool: PgPool) -> Result<Self, sqlx::Error> {
    create_session_context_table(&pool, &process).await?;
    let get_session_context_query = build_get_session_context_query(&process);
    let process = Arc::new(process);
    let (batch_tx, batch_rx) = mpsc::channel::<SessionBatchCommand>(1024);
    spawn_session_batch_worker(pool.clone(), process.clone(), get_session_context_query.clone(), batch_rx);
    Ok(QriosUssdApiService { process, pool, get_session_context_query, batch_tx })
  }

  async fn db_create_session_context(
    &self,
    id: Uuid,
    current_run_yielded_at: CurrentRunYieldedAt,
    form_context: MaybeFormContext,
    session_context: SessionContext,
  ) -> Result<(), sqlx::Error> {
    let (tx, rx) = oneshot::channel();
    self
      .batch_tx
      .send(SessionBatchCommand::Create { id, current_run_yielded_at, form_context, session_context, tx })
      .await
      .map_err(|_| sqlx::Error::PoolClosed)?;
    rx.await.map_err(|_| sqlx::Error::PoolClosed)?
  }

  async fn db_get_session_context(
    &self,
    session_id: Uuid,
  ) -> Result<(MaybeFormContext, Vec<i32>, SessionContext), sqlx::Error> {
    let (tx, rx) = oneshot::channel();
    self.batch_tx.send(SessionBatchCommand::Get { id: session_id, tx }).await.map_err(|_| sqlx::Error::PoolClosed)?;
    rx.await.map_err(|_| sqlx::Error::PoolClosed)?
  }

  async fn db_update_session_context(
    &self,
    id: Uuid,
    form_context: MaybeFormContext,
    visited_form_steps: Vec<i32>,
    session_context: SessionContext,
  ) -> Result<(), sqlx::Error> {
    let (tx, rx) = oneshot::channel();
    self
      .batch_tx
      .send(SessionBatchCommand::Update { id, form_context, visited_form_steps, session_context, tx })
      .await
      .map_err(|_| sqlx::Error::PoolClosed)?;
    rx.await.map_err(|_| sqlx::Error::PoolClosed)?
  }

  async fn db_delete_session_context(&self, id: Uuid) -> Result<u64, sqlx::Error> {
    let (tx, rx) = oneshot::channel();
    self.batch_tx.send(SessionBatchCommand::Delete { id, tx }).await.map_err(|_| sqlx::Error::PoolClosed)?;
    rx.await.map_err(|_| sqlx::Error::PoolClosed)?
  }

  async fn db_increment_failed_input_validation_attempts(
    &self,
    id: Uuid,
    form_context: Vec<u8>,
  ) -> Result<u64, sqlx::Error> {
    let (tx, rx) = oneshot::channel();
    self
      .batch_tx
      .send(SessionBatchCommand::IncrementValidation { id, form_context, tx })
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
  /// I guess we could delete by [`AbortSession`] `session_id`
  async fn post_ussdsessionevent_abort(
    &self,
    method: &http::method::Method,
    host: &headers::Host,
    cookies: &axum_extra::extract::cookie::CookieJar,
    header_params: &PostUssdsessioneventAbortHeaderParams,
    body: &AbortSession,
  ) -> Result<PostUssdsessioneventAbortResponse, ()> {
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
    let session_id = uuid::Uuid::parse_str(&body.context_data).map_err(|_| ())?;
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
    let user_input = match body.result.clone() {
      UssdActionResult::EmbeddedProcessResult(_) => todo!(),
      UssdActionResult::InputResult(input_result) => input_result.value,
      UssdActionResult::MerchantPaymentResult(_) => todo!(),
      UssdActionResult::ReturnFromRedirectResult(_) => todo!(),
    };
    let session_id = uuid::Uuid::parse_str(&body.context_data).map_err(|_| ())?;
    let (form_context, mut visited_form_steps, session_context) =
      self.db_get_session_context(session_id).await.map_err(|_| ())?;
    let previous_run_yielded_at = PreviousRunYieldedAt(*visited_form_steps.last().ok_or(())?);

    let mut run_result = {
      let back_token = visited_form_steps.is_empty().not().then(create_back_token);
      self
        .process
        .resume_run(session_context.clone(), previous_run_yielded_at, user_input, form_context, back_token)
        .await
    };

    let mut is_back = false;
    if let Ok(RunOutcome::Back) = run_result {
      is_back = true;
      visited_form_steps.pop();
      let target_step_index = *visited_form_steps.last().ok_or(())?;
      let back_token = if visited_form_steps.len() > 1 { Some(create_back_token()) } else { None };
      run_result = self
        .process
        .resume_run(
          session_context.clone(),
          PreviousRunYieldedAt(target_step_index),
          String::new(),
          None::<FormContext>,
          back_token,
        )
        .await;
    }

    match run_result {
      Ok(RunOutcome::Yield(message, session_context, current_run_yielded_at, form_context)) => {
        if is_back.not() {
          visited_form_steps.push(current_run_yielded_at.0);
        }
        self
          .db_update_session_context(session_id, Some(form_context), visited_form_steps, session_context)
          .await
          .map_err(|_| ())?;
        Ok(UssdView::InputView(InputView { message: message.0, r_type: "InputView".into() }))
      },
      Ok(RunOutcome::RetryUserInput(message, form_context)) => {
        self.db_increment_failed_input_validation_attempts(session_id, form_context).await.map_err(|_| ())?;
        Ok(UssdView::InputView(InputView { message: message.0, r_type: "InputView".into() }))
      },
      Ok(RunOutcome::Finish(message)) => {
        self.db_delete_session_context(session_id).await.map_err(|_| ())?;
        Ok(UssdView::InfoView(InfoView { message: message.0, r_type: "InfoView".into() }))
      },
      Ok(RunOutcome::Back) => Err(()),
      Err(e) => {
        tracing::error!("Resume session failed: {:?}", e);
        self.db_delete_session_context(session_id).await.map_err(|_| ())?;
        Err(())
      },
    }
    .map(|ussd_view| {
      PostUssdsessioneventContinueResponse::Status200_SessionContinuationHasBeenSuccessfullyHandledByTheDeveloper(
        UssdSessionCommand {
          action: UssdAction::ShowView(ShowView { r_type: "ShowView".into(), view: ussd_view }),
          context_data: session_id.to_string(),
          session_tag: None,
        },
      )
    })
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
    let shortcode_string = match body.input.clone() {
      UssdSessionEventNewSessionSessionInput::Dial(x) => x.shortcode_string,
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
      ShortcodeString(shortcode_string)
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
        self
          .db_create_session_context(session_id, current_run_yielded_at, Some(form_context), session_context)
          .await
          .map_err(|_| ())?;
        Ok((session_id, UssdView::InputView(InputView { message: message.0, r_type: "InputView".into() })))
      },
      Ok(RunOutcome::RetryUserInput(..)) => {
        unreachable!("We haven't prompted user for input yet")
      },
      Ok(RunOutcome::Finish(message)) => {
        Ok((uuid::Uuid::nil(), UssdView::InfoView(InfoView { message: message.0, r_type: "InfoView".into() })))
      },
      Ok(RunOutcome::Back) => Err(()),
      Err(e) => {
        tracing::error!("New session failed: {:?}", e);
        Err(())
      },
    }
    .map(|(session_id, ussd_view)| {
      PostUssdsessioneventNewResponse::Status200_SessionStartHasBeenSuccessfullyHandledByTheDeveloper(
        UssdSessionCommand {
          action: UssdAction::ShowView(ShowView { r_type: "ShowView".into(), view: ussd_view }),
          context_data: session_id.to_string(),
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
          session_id: "test_session_1".into(),
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
          session_id: "test_session_1".into(),
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
          session_id: "test_session_1".into(),
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
          session_id: "test_session_1".into(),
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
}
