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
    let run_result = self
      .process
      .resume_run(session_context, previous_run_yielded_at, user_input, failed_input_validation_attempts)
      .await;
    match run_result {
      Ok(RunOutcome::Yield(message, session_context, current_run_yielded_at)) => {
        let id = update_session_context(
          &self.pool,
          &self.process,
          session_id,
          current_run_yielded_at,
          FailedInputValidationAttempts(0),
          &session_context, // you can currently overwrite params with split and later merge
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
      Err(_) => {
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
      Err(_) => Err(()),
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
  use tokio::signal;
  use type_process_builder::builder::*;
  use type_process_builder::param_list::ParamValue;
  use type_process_builder::step::Final;
  use type_process_builder::step::*;
  use typenum::*;

  #[ignore]
  #[tokio::test]
  async fn no_op_process_test() {
    #[derive(Clone, Deserialize, Serialize)]
    struct Split1Param;
    impl ParamValue for Split1Param {
      type UID = U1;
    }

    #[derive(Clone, Deserialize, Serialize)]
    struct Split2Param;
    impl ParamValue for Split2Param {
      type UID = U2;
    }

    #[derive(Clone, Deserialize, Serialize)]
    struct Case1Param;
    impl ParamValue for Case1Param {
      type UID = U4;
    }

    #[derive(Clone, Deserialize, Serialize)]
    struct CommonCaseParam;
    impl ParamValue for CommonCaseParam {
      type UID = U6;
    }

    struct ProduceCaseParam1;
    impl Operation for ProduceCaseParam1 {
      type Consumes = HNil;
      type Produces = HList![Case1Param, CommonCaseParam];
      type FinalMessage = Message;

      async fn handle(
        &self,
        _consumes: Self::Consumes,
      ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
        Ok(OperationOutcome::Successful(hlist!(Case1Param, CommonCaseParam)))
      }
    }

    struct FinishAfterInput;
    impl Form for FinishAfterInput {
      type CreateFormConsumes = HNil;
      type ValidateInputConsumes = HNil;
      type Produces = HList![CommonCaseParam];
      type Messages = Messages;

      async fn create_form(&self, _consumes: Self::CreateFormConsumes) -> anyhow::Result<Message> {
        Ok(Message("Last number in the process".into()))
      }

      async fn handle_input(
        &self,
        _consumes: Self::ValidateInputConsumes,
        _user_input: String,
        _failed_input_validation_attempts: FailedInputValidationAttempts,
      ) -> anyhow::Result<InputValidation<Self::Produces, Messages>> {
        Ok(InputValidation::Finish(Message("Always finnish".into())))
      }
    }

    struct OneInputRetryForm;
    impl Form for OneInputRetryForm {
      type CreateFormConsumes = HNil;
      type ValidateInputConsumes = HNil;
      type Produces = HList![CommonCaseParam];
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
          _ => Ok(InputValidation::Successful(hlist![CommonCaseParam])),
        }
      }
    }

    struct FinalNoConsumes;
    impl Final for FinalNoConsumes {
      type Consumes = HNil;
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

      type Produces = Coprod![(Case1, HList![Split1Param]), (Case2, HList![Split2Param])];
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
          ("1", _) => Ok(InputValidation::Successful(Self::Produces::inject((Case1, hlist![Split1Param])))),
          _ => Ok(InputValidation::Successful(Self::Produces::inject((Case2, hlist![Split2Param])))),
        }
      }
    }

    let _process = DialedSessionEntry::<Messages>::new()
      .show(OneInputRetryForm)
      .then(ProduceCaseParam1)
      .show(FinishAfterInput)
      .show_split(TestFormSplitter)
      .case_end(Case1, |x| x.end(FinalNoConsumes))
      .case_end(Case2, |x| x.end(FinalNoConsumes))
      .build("", 0);

    async fn _shutdown_signal() {
      let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
      };

      #[cfg(unix)]
      let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
          .expect("failed to install signal handler")
          .recv()
          .await;
      };

      #[cfg(not(unix))]
      let terminate = std::future::pending::<()>();

      tokio::select! {
          _ = ctrl_c => {},
          _ = terminate => {},
      }
    }

    // use crate::QriosUssdApiService;
    // use qrios_api_reqwest_client::Client;
    // use qrios_api_reqwest_client::types::*;
    // use std::sync::Arc;
    // use tokio::net::TcpListener;
    // let app = qrios_api_axum_server::server::new(Arc::new(QriosUssdApiService { process }));
    // let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind random port");
    // let addr = listener.local_addr().expect("Failed to get server local address");
    // let _server = tokio::spawn(async move {
    //   axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.expect("Failed to start server");
    // });
    //
    // let resp = Client::new(format!("http://{addr}").as_str())
    //   .post_ussdsessionevent_new(
    //     None,
    //     &UssdSessionEventNewSession {
    //       app_id: "val".into(),
    //       client_id: "val".into(),
    //       input: UssdSessionEventNewSessionSessionInput::Dial(Dial {
    //         type_: DialType::Dial,
    //         shortcode_string: "*425*001*123#".to_string(),
    //       }),
    //       msisdn: "2341234567890".into(),
    //       operator: UssdSessionEventNewSessionOperator::Mtn,
    //       session_id: "val".into(),
    //     },
    //   )
    //   .await
    //   .expect("Failed to get a response from post_ussdsessionevent_new");
    // assert!(matches!(
    //   resp.action.clone(),
    //   UssdAction::ShowView(ShowView{view: UssdView::InfoView(InfoView{message, ..}), ..}) if
    //   message == "Hello from rust"
    // ))
  }
}
