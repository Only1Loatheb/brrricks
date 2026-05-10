mod session_store;

use crate::session_store::*;
use async_trait::async_trait;
use qrios_api_axum_server::apis::ErrorHandler;
use qrios_api_axum_server::apis::developers_app_endpoints::{
  PostUssdsessioneventAbortResponse, PostUssdsessioneventCloseResponse, PostUssdsessioneventContinueResponse,
  PostUssdsessioneventNewResponse,
};
use qrios_api_axum_server::models::*;
use serde_value::Value;
use sqlx::PgPool;
use std::collections::HashSet;
use std::ops::Not;
use type_process_builder::builder::{
  FinalizedProcess, ParamUID, PreviousRunYieldedAt, RunOutcome, RunnableProcess, StepIndex,
};
use type_process_builder::step::{FailedInputValidationAttempts, ProcessMessages};

pub struct Message(pub String);

pub struct Messages;
impl ProcessMessages for Messages {
  type FormMessage = Message;
  type FinalMessage = Message;
}

pub struct QriosUssdApiService<Process: FinalizedProcess<Messages = Messages>> {
  process: RunnableProcess<Process>,
  pool: PgPool,
  ordered_all_unique_param_uids: Vec<ParamUID>,
  get_session_context_query: GetSessionContextQuery,
}

impl<Process: FinalizedProcess<Messages = Messages>> QriosUssdApiService<Process> {
  pub async fn new(process: RunnableProcess<Process>, pool: PgPool) -> Result<Self, sqlx::Error> {
    let ordered_all_unique_param_uids = process.ordered_all_unique_param_uids();
    create_session_context_table(&pool, &process, &ordered_all_unique_param_uids).await?;
    let get_session_context_query = build_get_session_context_query(&process, &ordered_all_unique_param_uids);
    Ok(QriosUssdApiService { process, pool, ordered_all_unique_param_uids, get_session_context_query })
  }
}

impl<Process: FinalizedProcess<Messages = Messages>> ErrorHandler<()> for QriosUssdApiService<Process> {}

#[allow(unused_variables)]
#[async_trait]
impl<Process: FinalizedProcess<Messages = Messages> + Sync>
  qrios_api_axum_server::apis::developers_app_endpoints::DevelopersAppEndpoints for QriosUssdApiService<Process>
{
  /// I guess we could delete by [AbortSession] session_id
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
    let session_id = body.context_data.parse::<i64>().map_err(|_| ())?;
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
    let user_input = match body.result.clone() {
      UssdActionResult::EmbeddedProcessResult(_) => todo!(),
      UssdActionResult::InputResult(input_result) => input_result.value,
      UssdActionResult::MerchantPaymentResult(_) => todo!(),
      UssdActionResult::ReturnFromRedirectResult(_) => todo!(),
    };
    let session_id = body.context_data.parse::<i64>().map_err(|_| ())?;
    let (previous_run_yielded_at, failed_input_validation_attempts, session_context) =
      get_session_context(&self.pool, &self.get_session_context_query, session_id, &self.ordered_all_unique_param_uids)
        .await
        .map_err(|_| ())?;
    let already_stored_params = session_context.iter().map(|x| x.0).collect::<HashSet<_>>();
    let run_result = self
      .process
      .resume_run(session_context, previous_run_yielded_at, user_input, failed_input_validation_attempts)
      .await;
    match run_result {
      Ok(RunOutcome::Yield(message, session_context, current_run_yielded_at)) => {
        let session_context_param_ids = session_context.iter().map(|x| x.0).collect::<HashSet<_>>();
        let params_to_store =
          session_context.into_iter().filter(|x| already_stored_params.contains(&x.0).not()).collect::<Vec<_>>();
        let params_to_remove =
          already_stored_params.into_iter().filter(|x| session_context_param_ids.contains(x).not()).collect::<Vec<_>>();
        let id = update_session_context(
          &self.pool,
          &self.process,
          session_id,
          current_run_yielded_at,
          FailedInputValidationAttempts(0),
          params_to_store,
          params_to_remove,
        )
        .await
        .map_err(|_| ())?;
        Ok(UssdView::InputView(InputView { message: message.0, r_type: "InputView".into() }))
      },
      Ok(RunOutcome::RetryUserInput(message)) => {
        increment_failed_input_validation_attempts(&self.pool, &self.process, session_id).await.map_err(|_| ())?;
        Ok(UssdView::InputView(InputView { message: message.0, r_type: "InputView".into() }))
      },
      Ok(RunOutcome::Finish(message)) => {
        delete_session_context(&self.pool, &self.process, session_id).await.map_err(|_| ())?;
        Ok(UssdView::InfoView(InfoView { message: message.0, r_type: "InfoView".into() }))
      },
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
    let shortcode_string = match body.input.clone() {
      UssdSessionEventNewSessionSessionInput::Dial(x) => x.shortcode_string,
      UssdSessionEventNewSessionSessionInput::Push(_) => todo!(),
      UssdSessionEventNewSessionSessionInput::Redirect(_) => todo!(),
    };
    let init_session_context = vec![(0, Value::String(body.msisdn.clone())), (1, Value::String(body.operator.clone()))];
    let run_result = self
      .process
      .resume_run(
        init_session_context,
        PreviousRunYieldedAt(StepIndex::MIN),
        shortcode_string,
        FailedInputValidationAttempts(0),
      )
      .await;
    match run_result {
      Ok(RunOutcome::Yield(message, session_context, current_run_yielded_at)) => {
        let id = create_session_context(
          &self.pool,
          &self.process,
          current_run_yielded_at,
          FailedInputValidationAttempts(0),
          &session_context,
        )
        .await
        .map_err(|_| ())?;
        Ok((id, UssdView::InputView(InputView { message: message.0, r_type: "InputView".into() })))
      },
      Ok(RunOutcome::RetryUserInput(message)) => {
        unreachable!("We haven't prompted user for input yet")
      },
      Ok(RunOutcome::Finish(message)) => {
        Ok((i64::MAX, UssdView::InfoView(InfoView { message: message.0, r_type: "InfoView".into() })))
      },
      Err(e) => {
        tracing::error!("New session failed: {:?}", e);
        Err(())
      },
    }
    .map(|(id, ussd_view)| {
      PostUssdsessioneventNewResponse::Status200_SessionStartHasBeenSuccessfullyHandledByTheDeveloper(
        UssdSessionCommand {
          action: UssdAction::ShowView(ShowView { r_type: "ShowView".into(), view: ussd_view }),
          context_data: id.to_string(),
          session_tag: None,
        },
      )
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::{Message, Messages};
  use frunk_core::hlist::HNil;
  use frunk_core::{Coprod, HList, hlist};
  use qrios_api_process_entry::DialedSessionEntry;
  use serde::{Deserialize, Serialize};
  use type_process_builder::builder::*;
  use type_process_builder::param_list::ParamValue;
  use type_process_builder::step::Final;
  use type_process_builder::step::*;
  use typenum::*;

  #[tokio::test]
  async fn session_store_test() {
    use crate::QriosUssdApiService;
    use qrios_api_reqwest_client::Client;
    use sqlx::PgPool;
    use std::sync::Arc;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;
    use tokio::net::TcpListener;
    let _ = tracing_subscriber::fmt::try_init();

    let node = Postgres::default().start().await.unwrap();
    let port = node.get_host_port_ipv4(5432).await.unwrap();
    let connection_string = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
    let pool = PgPool::connect(&connection_string).await.unwrap();

    sqlx::query("CREATE SCHEMA session_store").execute(&pool).await.expect("Failed to create schema");

    #[derive(Clone, Deserialize, Serialize)]
    struct FormOutput;
    impl ParamValue for FormOutput {
      type UID = U1;
    }

    #[derive(Clone, Deserialize, Serialize)]
    struct OperationOutput;
    impl ParamValue for OperationOutput {
      type UID = U2;
    }

    #[derive(Clone, Deserialize, Serialize)]
    struct SplitCase1Output;
    impl ParamValue for SplitCase1Output {
      type UID = U3;
    }

    #[derive(Clone, Deserialize, Serialize)]
    struct SplitCase2Output;
    impl ParamValue for SplitCase2Output {
      type UID = U4;
    }

    struct ProduceParamOperation;
    impl Operation for ProduceParamOperation {
      type Consumes = HNil;
      type Produces = HList![OperationOutput];
      type FinalMessage = Message;

      async fn handle(
        &self,
        _consumes: Self::Consumes,
      ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
        Ok(OperationOutcome::Successful(hlist!(OperationOutput)))
      }
    }

    struct AskForInputTwiceForm;
    impl Form for AskForInputTwiceForm {
      type CreateFormConsumes = HNil;
      type ValidateInputConsumes = HNil;
      type Produces = HList![FormOutput];
      type Messages = Messages;

      async fn create_form(&self, _consumes: Self::CreateFormConsumes) -> anyhow::Result<Message> {
        Ok(Message("This will be discarded".into()))
      }

      async fn handle_input(
        &self,
        _consumes: Self::ValidateInputConsumes,
        _user_input: String,
        failed_input_validation_attempts: FailedInputValidationAttempts,
      ) -> anyhow::Result<InputValidation<Self::Produces, Messages>> {
        match failed_input_validation_attempts.0 {
          0 => Ok(InputValidation::Retry(Message("This will be accepted".into()))),
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
      type Messages = Messages;

      async fn create_form(&self, _consumes: Self::CreateFormConsumes) -> anyhow::Result<Message> {
        Ok(Message("choose case".into()))
      }

      async fn handle_input(
        &self,
        _consumes: Self::ValidateInputConsumes,
        user_input: String,
        failed: FailedInputValidationAttempts,
      ) -> anyhow::Result<InputValidation<Self::Produces, Messages>> {
        match (user_input.as_str(), failed.0) {
          ("retry", 0) => Ok(InputValidation::Retry(Message("retry again".into()))),
          ("finish", _) => Ok(InputValidation::Finish(Message("finished early".into()))),
          ("1", _) => Ok(InputValidation::Successful(Self::Produces::inject((Case1, hlist![SplitCase1Output])))),
          _ => Ok(InputValidation::Successful(Self::Produces::inject((Case2, hlist![SplitCase2Output])))),
        }
      }
    }

    let process = DialedSessionEntry::<Messages>::new()
      .show(AskForInputTwiceForm)
      .then(ProduceParamOperation)
      .show_split(TestFormSplitter)
      .case_end(Case1, |x| x.end(ConsumeCase1Final))
      .case_end(Case2, |x| x.end(ConsumeCase2Final))
      .build("test_process", 1);

    let service = QriosUssdApiService::new(process, pool).await.expect("Failed to create service");
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
      _ => panic!("Expected InputView, got {:?}", &resp.action),
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
