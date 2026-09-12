mod session_store;

use crate::session_store::{
  GetSessionContextQuery, build_get_session_context_query, create_session_context, create_session_context_table,
  delete_session_context, get_session_context, increment_failed_input_validation_attempts, update_session_context,
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
use type_process_builder::back_navigation::create_back_token;
use type_process_builder::builder::{
  FinalizedProcess, FormContext, ParamList, PreviousRunYieldedAt, RunOutcome, RunnableProcess, StepIndex,
};
use type_process_builder::step::{BackToken, ProcessMessages};
use type_process_builder::{HList, hlist};

pub struct Message(pub String);

pub struct Messages;
impl ProcessMessages for Messages {
  type FormMessage = Message;
  type FinalMessage = Message;
}

type EntryConsumes = HList!(DialedSessionEntryParam);

pub struct QriosUssdApiService<Process: FinalizedProcess<Messages = Messages, EntryConsumes = EntryConsumes>> {
  process: RunnableProcess<Process>,
  pool: PgPool,
  get_session_context_query: GetSessionContextQuery,
}

impl<Process: FinalizedProcess<Messages = Messages, EntryConsumes = EntryConsumes>> QriosUssdApiService<Process> {
  pub async fn new(process: RunnableProcess<Process>, pool: PgPool) -> Result<Self, sqlx::Error> {
    create_session_context_table(&pool, &process).await?;
    let get_session_context_query = build_get_session_context_query(&process);
    Ok(QriosUssdApiService { process, pool, get_session_context_query })
  }
}

impl<Process: FinalizedProcess<Messages = Messages, EntryConsumes = EntryConsumes>> ErrorHandler<()>
  for QriosUssdApiService<Process>
{
}

#[allow(unused_variables)]
#[async_trait]
impl<Process: FinalizedProcess<Messages = Messages, EntryConsumes = EntryConsumes> + Sync>
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
    delete_session_context(&self.pool, &self.process, session_id).await.map_err(|_| ())?;
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
    delete_session_context(&self.pool, &self.process, session_id).await.map_err(|_| ())?;
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
    let (form_context, mut visited_form_steps, active_session_context_from_db) =
      get_session_context(&self.pool, &self.get_session_context_query, session_id).await.map_err(|_| ())?;

    let (previous_step, active_session_context) =
      visited_form_steps.last().cloned().unwrap_or((StepIndex::MIN, active_session_context_from_db));
    let previous_run_yielded_at = PreviousRunYieldedAt(previous_step);
    let back_token = (visited_form_steps.len() > 1).then(create_back_token);

    let mut run_result = self
      .process
      .resume_run(active_session_context, previous_run_yielded_at, user_input.clone(), form_context, back_token)
      .await;

    let mut was_backed = false;
    if let Ok(RunOutcome::Back) = run_result {
      was_backed = true;
      visited_form_steps.pop();
      let (target_step_index, target_session_context) =
        visited_form_steps.last().cloned().expect("Cannot go back further");
      let back_token = (visited_form_steps.len() > 1).then(create_back_token);
      run_result = self
        .process
        .resume_run(
          target_session_context,
          PreviousRunYieldedAt(target_step_index),
          String::new(),
          None::<FormContext>,
          back_token,
        )
        .await;
    }

    match run_result {
      Ok(RunOutcome::Yield(message, session_context, current_run_yielded_at, form_context)) => {
        if was_backed {
          if let Some(last) = visited_form_steps.last_mut() {
            *last = (current_run_yielded_at.0, session_context.clone());
          }
        } else {
          visited_form_steps.push((current_run_yielded_at.0, session_context.clone()));
        }
        update_session_context(
          &self.pool,
          &self.process,
          session_id,
          Some(form_context),
          &visited_form_steps,
          &session_context,
        )
        .await
        .map_err(|_| ())?;
        Ok(UssdView::InputView(InputView { message: message.0, r_type: "InputView".into() }))
      },
      Ok(RunOutcome::RetryUserInput(message, form_context)) => {
        increment_failed_input_validation_attempts(&self.pool, &self.process, session_id, form_context)
          .await
          .map_err(|_| ())?;
        Ok(UssdView::InputView(InputView { message: message.0, r_type: "InputView".into() }))
      },
      Ok(RunOutcome::Finish(message)) => {
        delete_session_context(&self.pool, &self.process, session_id).await.map_err(|_| ())?;
        Ok(UssdView::InfoView(InfoView { message: message.0, r_type: "InfoView".into() }))
      },
      Ok(RunOutcome::Back) => Err(()),
      Err(e) => {
        tracing::error!("Resume session failed: {:?}", e);
        delete_session_context(&self.pool, &self.process, session_id).await.map_err(|_| ())?;
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
        let visited_form_steps = vec![(current_run_yielded_at.0, session_context.clone())];
        create_session_context(
          &self.pool,
          &self.process,
          session_id,
          &visited_form_steps,
          Some(form_context),
          &session_context,
        )
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
