use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::{
    TypedHeader,
    extract::{CookieJar, Query as QueryExtra},
};
use bytes::Bytes;
use headers::Host;
use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode, header::CONTENT_TYPE};
use tracing::error;
use validator::{Validate, ValidationErrors};

#[allow(unused_imports)]
use crate::{apis, models};
use crate::{header, types::*};
#[allow(unused_imports)]
use crate::{
    models::check_xss_map, models::check_xss_map_nested, models::check_xss_map_string,
    models::check_xss_string, models::check_xss_vec_string,
};


/// Setup API Server.
pub fn new<I, A, E>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::developers_app_endpoints::DevelopersAppEndpoints<E> + Send + Sync + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
    
{
    // build our application with a route
    Router::new()
        .route("/ussdSessionEvent/abort",
            post(post_ussdsessionevent_abort::<I, A, E>)
        )
        .route("/ussdSessionEvent/close",
            post(post_ussdsessionevent_close::<I, A, E>)
        )
        .route("/ussdSessionEvent/continue",
            post(post_ussdsessionevent_continue::<I, A, E>)
        )
        .route("/ussdSessionEvent/new",
            post(post_ussdsessionevent_new::<I, A, E>)
        )
        .with_state(api_impl)
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostUssdsessioneventAbortBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::AbortSession,
    }


#[tracing::instrument(skip_all)]
fn post_ussdsessionevent_abort_validation(
  header_params: models::PostUssdsessioneventAbortHeaderParams,
        body: models::AbortSession,
) -> std::result::Result<(
  models::PostUssdsessioneventAbortHeaderParams,
        models::AbortSession,
), ValidationErrors>
{
  header_params.validate()?;
              let b = PostUssdsessioneventAbortBodyValidator { body: &body };
              b.validate()?;

Ok((
  header_params,
    body,
))
}
/// PostUssdsessioneventAbort - POST /ussdSessionEvent/abort
#[tracing::instrument(skip_all)]
async fn post_ussdsessionevent_abort<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  headers: HeaderMap,
 State(api_impl): State<I>,
          Json(body): Json<models::AbortSession>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::developers_app_endpoints::DevelopersAppEndpoints<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {



    // Header parameters
    let header_params = {
                let header_authorization = headers.get(HeaderName::from_static("authorization"));

                let header_authorization = match header_authorization {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header Authorization - {err}"))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
                    }
                };

       models::PostUssdsessioneventAbortHeaderParams {
          authorization: header_authorization,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_ussdsessionevent_abort_validation(
        header_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    header_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



let result = api_impl.as_ref().post_ussdsessionevent_abort(
      
      &method,
      &host,
      &cookies,
        &header_params,
              &body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::developers_app_endpoints::PostUssdsessioneventAbortResponse::Status200_TheAbortingOfTheSessionHasBeenSuccessfullyHandledByTheDeveloper
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::developers_app_endpoints::PostUssdsessioneventAbortResponse::Status204_TheAbortingOfTheSessionHasBeenSuccessfullyHandledByTheDeveloper
                                                => {
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostUssdsessioneventCloseBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::CloseSession,
    }


#[tracing::instrument(skip_all)]
fn post_ussdsessionevent_close_validation(
  header_params: models::PostUssdsessioneventCloseHeaderParams,
        body: models::CloseSession,
) -> std::result::Result<(
  models::PostUssdsessioneventCloseHeaderParams,
        models::CloseSession,
), ValidationErrors>
{
  header_params.validate()?;
              let b = PostUssdsessioneventCloseBodyValidator { body: &body };
              b.validate()?;

Ok((
  header_params,
    body,
))
}
/// PostUssdsessioneventClose - POST /ussdSessionEvent/close
#[tracing::instrument(skip_all)]
async fn post_ussdsessionevent_close<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  headers: HeaderMap,
 State(api_impl): State<I>,
          Json(body): Json<models::CloseSession>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::developers_app_endpoints::DevelopersAppEndpoints<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {



    // Header parameters
    let header_params = {
                let header_authorization = headers.get(HeaderName::from_static("authorization"));

                let header_authorization = match header_authorization {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header Authorization - {err}"))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
                    }
                };

       models::PostUssdsessioneventCloseHeaderParams {
          authorization: header_authorization,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_ussdsessionevent_close_validation(
        header_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    header_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



let result = api_impl.as_ref().post_ussdsessionevent_close(
      
      &method,
      &host,
      &cookies,
        &header_params,
              &body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::developers_app_endpoints::PostUssdsessioneventCloseResponse::Status200_SessionEndHasBeenSuccessfullyHandledByTheDeveloper
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::developers_app_endpoints::PostUssdsessioneventCloseResponse::Status204_SessionEndHasBeenSuccessfullyHandledByTheDeveloper
                                                => {
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostUssdsessioneventContinueBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::ContinueSession,
    }


#[tracing::instrument(skip_all)]
fn post_ussdsessionevent_continue_validation(
  header_params: models::PostUssdsessioneventContinueHeaderParams,
        body: models::ContinueSession,
) -> std::result::Result<(
  models::PostUssdsessioneventContinueHeaderParams,
        models::ContinueSession,
), ValidationErrors>
{
  header_params.validate()?;
              let b = PostUssdsessioneventContinueBodyValidator { body: &body };
              b.validate()?;

Ok((
  header_params,
    body,
))
}
/// PostUssdsessioneventContinue - POST /ussdSessionEvent/continue
#[tracing::instrument(skip_all)]
async fn post_ussdsessionevent_continue<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  headers: HeaderMap,
 State(api_impl): State<I>,
          Json(body): Json<models::ContinueSession>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::developers_app_endpoints::DevelopersAppEndpoints<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {



    // Header parameters
    let header_params = {
                let header_authorization = headers.get(HeaderName::from_static("authorization"));

                let header_authorization = match header_authorization {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header Authorization - {err}"))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
                    }
                };

       models::PostUssdsessioneventContinueHeaderParams {
          authorization: header_authorization,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_ussdsessionevent_continue_validation(
        header_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    header_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



let result = api_impl.as_ref().post_ussdsessionevent_continue(
      
      &method,
      &host,
      &cookies,
        &header_params,
              &body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::developers_app_endpoints::PostUssdsessioneventContinueResponse::Status200_SessionContinuationHasBeenSuccessfullyHandledByTheDeveloper
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostUssdsessioneventNewBodyValidator<'a> {
          #[validate(nested)]
          body: &'a models::UssdSessionEventNewSession,
    }


#[tracing::instrument(skip_all)]
fn post_ussdsessionevent_new_validation(
  header_params: models::PostUssdsessioneventNewHeaderParams,
        body: models::UssdSessionEventNewSession,
) -> std::result::Result<(
  models::PostUssdsessioneventNewHeaderParams,
        models::UssdSessionEventNewSession,
), ValidationErrors>
{
  header_params.validate()?;
              let b = PostUssdsessioneventNewBodyValidator { body: &body };
              b.validate()?;

Ok((
  header_params,
    body,
))
}
/// PostUssdsessioneventNew - POST /ussdSessionEvent/new
#[tracing::instrument(skip_all)]
async fn post_ussdsessionevent_new<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  headers: HeaderMap,
 State(api_impl): State<I>,
          Json(body): Json<models::UssdSessionEventNewSession>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::developers_app_endpoints::DevelopersAppEndpoints<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {



    // Header parameters
    let header_params = {
                let header_authorization = headers.get(HeaderName::from_static("authorization"));

                let header_authorization = match header_authorization {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header Authorization - {err}"))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
                    }
                };

       models::PostUssdsessioneventNewHeaderParams {
          authorization: header_authorization,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_ussdsessionevent_new_validation(
        header_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    header_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



let result = api_impl.as_ref().post_ussdsessionevent_new(
      
      &method,
      &host,
      &cookies,
        &header_params,
              &body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::developers_app_endpoints::PostUssdsessioneventNewResponse::Status200_SessionStartHasBeenSuccessfullyHandledByTheDeveloper
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[allow(dead_code)]
#[inline]
fn response_with_status_code_only(code: StatusCode) -> Result<Response, StatusCode> {
   Response::builder()
          .status(code)
          .body(Body::empty())
          .map_err(|_| code)
}
