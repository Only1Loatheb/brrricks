pub mod qrios_api_process_runner {}

use async_trait::async_trait;
use qrios_api_axum_server::apis::ErrorHandler;
use qrios_api_axum_server::apis::developers_app_endpoints::{
  PostUssdsessioneventAbortResponse, PostUssdsessioneventCloseResponse, PostUssdsessioneventContinueResponse,
  PostUssdsessioneventNewResponse,
};
use qrios_api_axum_server::models::{
  AbortSession, CloseSession, ContinueSession, PostUssdsessioneventAbortHeaderParams,
  PostUssdsessioneventCloseHeaderParams, PostUssdsessioneventContinueHeaderParams, PostUssdsessioneventNewHeaderParams,
  UssdSessionEventNewSession, UssdSessionEventNewSessionSessionInput,
};
use qrios_api_types::{EntryParam, Msisdn, Operator, ShortcodeString};
use serde::{Deserialize, Serialize};
use serde_value::Value;
use std::collections::HashMap;
use std::ops::Not;
use type_process_builder::builder::{FinalizedProcess, PreviousRunYieldedAt, RunnableProcess};
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
    let shortcode_string = match body.input {
      UssdSessionEventNewSessionSessionInput::UssdSessionEventNewSessionSessionInputOneOf(x) => x.dial.shortcode_string,
      UssdSessionEventNewSessionSessionInput::UssdSessionEventNewSessionSessionInputOneOf1(_) => todo!(),
      UssdSessionEventNewSessionSessionInput::UssdSessionEventNewSessionSessionInputOneOf2(_) => todo!(),
    };
    self
      .process
      .resume_run(
        HashMap::from([(0, Value::String(body.msisdn)), (1, Value::String(body.operator))]),
        PreviousRunYieldedAt(0),
        shortcode_string,
        FailedInputValidationAttempts(0),
      )
      .await?
  }
}
