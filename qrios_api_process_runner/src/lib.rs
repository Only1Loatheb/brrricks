pub mod qrios_api_process_runner {}

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
  ShowView, UssdSessionCommand, UssdSessionEventNewSession, UssdSessionEventNewSessionSessionInput, UssdView,
  UssdViewInfoView, UssdViewInputView,
};
use serde_value::Value;
use std::collections::HashMap;
use type_process_builder::builder::{FinalizedProcess, PreviousRunYieldedAt, RunOutcome, RunnableProcess};
use type_process_builder::step::FailedInputValidationAttempts;

pub struct QriosUssdApiService<Process: FinalizedProcess> {
  process: RunnableProcess<Process>,
}

impl<Process: FinalizedProcess> ErrorHandler<()> for QriosUssdApiService<Process> {}

#[allow(unused_variables)]
#[async_trait]
impl<Process: FinalizedProcess + Sync> qrios_api_axum_server::apis::developers_app_endpoints::DevelopersAppEndpoints
  for QriosUssdApiService<Process>
{
  async fn post_ussdsessionevent_abort(
    &self,
    method: &http::method::Method,
    host: &headers::Host,
    cookies: &axum_extra::extract::cookie::CookieJar,
    header_params: &PostUssdsessioneventAbortHeaderParams,
    body: &AbortSession,
  ) -> Result<PostUssdsessioneventAbortResponse, ()> {
    _ = self.process;
    todo!()
  }

  async fn post_ussdsessionevent_close(
    &self,
    method: &http::method::Method,
    host: &headers::Host,
    cookies: &axum_extra::extract::cookie::CookieJar,
    header_params: &PostUssdsessioneventCloseHeaderParams,
    body: &CloseSession,
  ) -> Result<PostUssdsessioneventCloseResponse, ()> {
    todo!()
  }

  async fn post_ussdsessionevent_continue(
    &self,
    method: &http::method::Method,
    host: &headers::Host,
    cookies: &axum_extra::extract::cookie::CookieJar,
    header_params: &PostUssdsessioneventContinueHeaderParams,
    body: &ContinueSession,
  ) -> Result<PostUssdsessioneventContinueResponse, ()> {
    todo!()
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
    let a = self
      .process
      .resume_run(
        HashMap::from([
          (0, Value::String(body.msisdn.clone())),
          (1, Value::String(body.operator.clone())),
        ]),
        PreviousRunYieldedAt(0),
        shortcode_string,
        FailedInputValidationAttempts(0),
      )
      .await;
    match a {
      // fixme session store the stuff
      Ok(RunOutcome::Yield(message, session_context, current_run_yielded_at)) => {
        Ok(UssdView::UssdViewInputView(UssdViewInputView {
          message: message.0,
          r_type: "InputView".into(),
        }))
      }
      Ok(RunOutcome::RetryUserInput(message)) => Ok(UssdView::UssdViewInputView(UssdViewInputView {
        message: message.0,
        r_type: "InputView".into(),
      })),
      Ok(RunOutcome::Finish(message)) => Ok(UssdView::UssdViewInfoView(UssdViewInfoView {
        message: message.0,
        r_type: "InfoView".into(),
      })),
      Err(_) => Err(()),
    }
    .map(|ussd_view| {
      PostUssdsessioneventNewResponse::Status200_SessionStartHasBeenSuccessfullyHandledByTheDeveloper(
        UssdSessionCommand {
          action: UssdActionOneOf2(models::UssdActionOneOf2 {
            show_view: ShowView {
              r_type: "ShowView".into(),
              view: ussd_view,
            },
          }),
          context_data: "session store magic number goes here".into(),
          session_tag: None,
        },
      )
    })
  }
}
