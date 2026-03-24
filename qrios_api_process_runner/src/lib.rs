mod session_store;

use crate::session_store::*;
use async_trait::async_trait;
use qrios_api_axum_server::apis::ErrorHandler;
use qrios_api_axum_server::apis::developers_app_endpoints::{
  PostUssdsessioneventAbortResponse, PostUssdsessioneventCloseResponse, PostUssdsessioneventContinueResponse,
  PostUssdsessioneventNewResponse,
};
use qrios_api_axum_server::models;
use qrios_api_axum_server::models::UssdAction::UssdActionOneOf2;
use qrios_api_axum_server::models::{
  AbortSession, CloseSession, ContinueSession, PostUssdsessioneventAbortHeaderParams,
  PostUssdsessioneventCloseHeaderParams, PostUssdsessioneventContinueHeaderParams, PostUssdsessioneventNewHeaderParams,
  ShowView, UssdActionResult, UssdSessionCommand, UssdSessionEventNewSession, UssdSessionEventNewSessionSessionInput,
  UssdView, UssdViewInfoView, UssdViewInputView,
};
use serde_value::Value;
use sqlx::PgPool;
use std::collections::HashSet;
use std::ops::Not;
use type_process_builder::builder::{
  FinalizedProcess, ParamUID, PreviousRunYieldedAt, RunOutcome, RunnableProcess, StepIndex,
};
use type_process_builder::step::FailedInputValidationAttempts;

pub struct QriosUssdApiService<Process: FinalizedProcess> {
  process: RunnableProcess<Process>,
  pool: PgPool,
  ordered_all_unique_param_uids: Vec<ParamUID>,
  get_session_context_query: GetSessionContextQuery,
}

impl<Process: FinalizedProcess> QriosUssdApiService<Process> {
  pub async fn new(process: RunnableProcess<Process>, pool: PgPool) -> Result<Self, sqlx::Error> {
    let ordered_all_unique_param_uids = process.ordered_all_unique_param_uids();
    create_session_context_table(&pool, &process, &ordered_all_unique_param_uids).await?;
    let get_session_context_query = build_get_session_context_query(&process, &ordered_all_unique_param_uids);
    Ok(QriosUssdApiService { process, pool, ordered_all_unique_param_uids, get_session_context_query })
  }
}

impl<Process: FinalizedProcess> ErrorHandler<()> for QriosUssdApiService<Process> {}

#[allow(unused_variables)]
#[async_trait]
impl<Process: FinalizedProcess + Sync> qrios_api_axum_server::apis::developers_app_endpoints::DevelopersAppEndpoints
  for QriosUssdApiService<Process>
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
      UssdActionResult::UssdActionResultOneOf(_) => todo!(),
      UssdActionResult::UssdActionResultOneOf1(input_result) => input_result.input_result.value,
      UssdActionResult::UssdActionResultOneOf2(_) => todo!(),
      UssdActionResult::Object(_) => todo!(),
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
        let new_params_to_store =
          session_context.into_iter().filter(|x| already_stored_params.contains(&x.0).not()).collect::<Vec<_>>();
        let id = update_session_context(
          &self.pool,
          &self.process,
          session_id,
          current_run_yielded_at,
          FailedInputValidationAttempts(0),
          &*new_params_to_store,
        )
        .await
        .map_err(|_| ())?;
        Ok(UssdView::UssdViewInputView(UssdViewInputView { message: message.0, r_type: "InputView".into() }))
      },
      Ok(RunOutcome::RetryUserInput(message)) => {
        increment_failed_input_validation_attempts(&self.pool, &self.process, session_id).await.map_err(|_| ())?;
        Ok(UssdView::UssdViewInputView(UssdViewInputView { message: message.0, r_type: "InputView".into() }))
      },
      Ok(RunOutcome::Finish(message)) => {
        delete_session_context(&self.pool, &self.process, session_id).await.map_err(|_| ())?;
        Ok(UssdView::UssdViewInfoView(UssdViewInfoView { message: message.0, r_type: "InfoView".into() }))
      },
      Err(_) => {
        delete_session_context(&self.pool, &self.process, session_id).await.map_err(|_| ())?;
        Err(())
      },
    }
    .map(|ussd_view| {
      PostUssdsessioneventContinueResponse::Status200_SessionContinuationHasBeenSuccessfullyHandledByTheDeveloper(
        UssdSessionCommand {
          action: UssdActionOneOf2(models::UssdActionOneOf2 {
            show_view: ShowView { r_type: "ShowView".into(), view: ussd_view },
          }),
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
      UssdSessionEventNewSessionSessionInput::UssdSessionEventNewSessionSessionInputOneOf(x) => x.dial.shortcode_string,
      UssdSessionEventNewSessionSessionInput::UssdSessionEventNewSessionSessionInputOneOf1(_) => todo!(),
      UssdSessionEventNewSessionSessionInput::UssdSessionEventNewSessionSessionInputOneOf2(_) => todo!(),
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
          &*session_context,
        )
        .await
        .map_err(|_| ())?;
        Ok((id, UssdView::UssdViewInputView(UssdViewInputView { message: message.0, r_type: "InputView".into() })))
      },
      Ok(RunOutcome::RetryUserInput(message)) => {
        unreachable!("We haven't prompted user for input yet")
      },
      Ok(RunOutcome::Finish(message)) => {
        Ok((i64::MAX, UssdView::UssdViewInfoView(UssdViewInfoView { message: message.0, r_type: "InfoView".into() })))
      },
      Err(_) => Err(()),
    }
    .map(|(id, ussd_view)| {
      PostUssdsessioneventNewResponse::Status200_SessionStartHasBeenSuccessfullyHandledByTheDeveloper(
        UssdSessionCommand {
          action: UssdActionOneOf2(models::UssdActionOneOf2 {
            show_view: ShowView { r_type: "ShowView".into(), view: ussd_view },
          }),
          context_data: id.to_string(),
          session_tag: None,
        },
      )
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::QriosUssdApiService;
  use frunk_core::hlist::HNil;
  use qrios_api_process_entry::DialedSessionEntry;
  use qrios_api_reqwest_client::Client;
  use qrios_api_reqwest_client::types::*;
  use sqlx::postgres::PgPoolOptions;
  use std::sync::Arc;
  use tokio::net::TcpListener;
  use tokio::signal;
  use type_process_builder::builder::FinalizedProcess;
  use type_process_builder::builder::{FlowingProcess, Message};
  use type_process_builder::step::Final;

  #[ignore]
  #[tokio::test]
  async fn no_op_process_test() {
    struct NoOpFinalStep;
    impl Final for NoOpFinalStep {
      type Consumes = HNil;
      async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Message> {
        Ok(Message("Good bye".into()))
      }
    }

    let process = DialedSessionEntry.end(NoOpFinalStep).build("no_op_process", 0);

    let pool = PgPoolOptions::new()
      .max_connections(5)
      .connect("postgres://postgres:password@localhost/test")
      .await
      .expect("Failed to connect to PostgreSQL server");

    let service = QriosUssdApiService::new(process, pool).await.expect("Failed to create QriosUssdApiService");
    tracing_subscriber::fmt::init();
    let app = qrios_api_axum_server::server::new(Arc::new(service));
    // Add layers to the router
    // let app = app.layer(...);
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind random port");
    let addr = listener.local_addr().expect("Failed to get server local address");
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.expect("Failed to start server");

    let resp = Client::new(format!("http://{addr}").as_str())
      .post_ussdsessionevent_new(
        None,
        &UssdSessionEventNewSession {
          app_id: "val".into(),
          client_id: "val".into(),
          input: UssdSessionEventNewSessionSessionInput::Dial {
            0: NewSessionSessionInputDial {
              shortcode_string: "*425*001*123#".to_string(),
              type_: NewSessionSessionInputDialType::Dial,
            },
          },
          msisdn: "2341234567890".into(),
          operator: UssdSessionEventNewSessionOperator::Mtn,
          session_id: "val".into(),
        },
      )
      .await
      .expect("Failed to get a response from post_ussdsessionevent_new");
    assert!(matches!(
      resp.action.clone(),
      UssdAction::Variant4{show_view: ShowView{view: UssdView::InfoView(UssdViewInfoView{message, ..}), ..}} if
      message == "Good bye"
    ))
  }

  async fn shutdown_signal() {
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
}
