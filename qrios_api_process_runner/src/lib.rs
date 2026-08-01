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
use sqlx::PgPool;
use std::collections::HashSet;
use std::ops::Not;
use type_process_builder::back_navigation::create_back_token;
use type_process_builder::builder::{
  FinalizedProcess, FormContext, ParamUID, PreviousRunYieldedAt, RunOutcome, RunnableProcess, StepIndex,
};
use type_process_builder::step::{BackToken, ProcessMessages};

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
    let (previous_run_yielded_at, form_context, mut visited_form_steps, session_context) =
      get_session_context(&self.pool, &self.get_session_context_query, session_id, &self.ordered_all_unique_param_uids)
        .await
        .map_err(|_| ())?;
    let already_stored_params = session_context.iter().map(|x| x.0).collect::<HashSet<_>>();

    let mut run_result = {
      let back_token = visited_form_steps.is_empty().not().then(create_back_token);
      self
        .process
        .resume_run(session_context.clone(), previous_run_yielded_at, user_input, form_context, back_token)
        .await
    };

    if let Ok(RunOutcome::Back) = run_result {
      let target_step_index = visited_form_steps.pop().ok_or(())?;
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
        let params_to_store =
          session_context.into_iter().filter(|x| already_stored_params.contains(&x.0).not()).collect::<Vec<_>>();
        if visited_form_steps.last() != Some(&current_run_yielded_at.0) {
          visited_form_steps.push(current_run_yielded_at.0);
        }
        let id = update_session_context(
          &self.pool,
          &self.process,
          session_id,
          current_run_yielded_at,
          Some(form_context),
          visited_form_steps,
          params_to_store,
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
    let shortcode_string = match body.input.clone() {
      UssdSessionEventNewSessionSessionInput::Dial(x) => x.shortcode_string,
      UssdSessionEventNewSessionSessionInput::Push(_) => todo!(),
      UssdSessionEventNewSessionSessionInput::Redirect(_) => todo!(),
    };
    let init_session_context =
      vec![(0, postcard::to_allocvec(&body.msisdn).unwrap()), (1, postcard::to_allocvec(&body.operator).unwrap())];
    let run_result = self
      .process
      .resume_run(
        init_session_context,
        PreviousRunYieldedAt(StepIndex::MIN),
        shortcode_string,
        None::<FormContext>,
        None::<BackToken>,
      )
      .await;
    match run_result {
      Ok(RunOutcome::Yield(message, session_context, current_run_yielded_at, form_context)) => {
        let id = create_session_context(
          &self.pool,
          &self.process,
          current_run_yielded_at,
          Some(form_context),
          session_context,
        )
        .await
        .map_err(|_| ())?;
        Ok((id, UssdView::InputView(InputView { message: message.0, r_type: "InputView".into() })))
      },
      Ok(RunOutcome::RetryUserInput(..)) => {
        unreachable!("We haven't prompted user for input yet")
      },
      Ok(RunOutcome::Finish(message)) => {
        Ok((i64::MAX, UssdView::InfoView(InfoView { message: message.0, r_type: "InfoView".into() })))
      },
      Ok(RunOutcome::Back) => Err(()),
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
  use qrios_api_process_entry::DialedSessionEntry;
  use serde::{Deserialize, Serialize};
  use type_process_builder::builder::*;
  use type_process_builder::step::Final;
  use type_process_builder::step::*;
  use type_process_builder::{Coprod, HList, HNil, ToRef, hlist};
  use typenum::*;

  #[allow(clippy::too_many_lines)]
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

    let process = DialedSessionEntry::<Messages>::new()
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
