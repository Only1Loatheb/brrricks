use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use headers::Host;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostUssdsessioneventAbortResponse {
    /// The aborting of the session has been successfully handled by the developer; the server acknowledges that the session has been aborted. Response body is empty.
    Status200_TheAbortingOfTheSessionHasBeenSuccessfullyHandledByTheDeveloper
    ,
    /// The aborting of the session has been successfully handled by the developer; the server acknowledges that the session has been aborted. Response body is empty.
    Status204_TheAbortingOfTheSessionHasBeenSuccessfullyHandledByTheDeveloper
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostUssdsessioneventCloseResponse {
    /// Session end has been successfully handled by the developer; the server acknowledges that the session has been closed. Response body is empty.
    Status200_SessionEndHasBeenSuccessfullyHandledByTheDeveloper
    ,
    /// Session end has been successfully handled by the developer; the server acknowledges that the session has been closed. Response body is empty.
    Status204_SessionEndHasBeenSuccessfullyHandledByTheDeveloper
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostUssdsessioneventContinueResponse {
    /// Session continuation has been successfully handled by the developer, the response contains a USSD command issued to a user.
    Status200_SessionContinuationHasBeenSuccessfullyHandledByTheDeveloper
    (models::UssdSessionCommand)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostUssdsessioneventNewResponse {
    /// Session start has been successfully handled by the developer, the response contains a USSD command issued to a user.
    Status200_SessionStartHasBeenSuccessfullyHandledByTheDeveloper
    (models::UssdSessionCommand)
}




/// DevelopersAppEndpoints
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait DevelopersAppEndpoints<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Called when a USSD session is being aborted (internal error).
    ///
    /// PostUssdsessioneventAbort - POST /ussdSessionEvent/abort
    async fn post_ussdsessionevent_abort(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::PostUssdsessioneventAbortHeaderParams,
            body: &models::AbortSession,
    ) -> Result<PostUssdsessioneventAbortResponse, E>;

    /// Called when a USSD session is closing gracefully (user cannot provide more input, user exits, session times out).
    ///
    /// PostUssdsessioneventClose - POST /ussdSessionEvent/close
    async fn post_ussdsessionevent_close(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::PostUssdsessioneventCloseHeaderParams,
            body: &models::CloseSession,
    ) -> Result<PostUssdsessioneventCloseResponse, E>;

    /// Called when a USSD session is continuing (user responded to previous message).
    ///
    /// PostUssdsessioneventContinue - POST /ussdSessionEvent/continue
    async fn post_ussdsessionevent_continue(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::PostUssdsessioneventContinueHeaderParams,
            body: &models::ContinueSession,
    ) -> Result<PostUssdsessioneventContinueResponse, E>;

    /// Called when a new USSD session is starting.
    ///
    /// PostUssdsessioneventNew - POST /ussdSessionEvent/new
    async fn post_ussdsessionevent_new(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::PostUssdsessioneventNewHeaderParams,
            body: &models::UssdSessionEventNewSession,
    ) -> Result<PostUssdsessioneventNewResponse, E>;
}
