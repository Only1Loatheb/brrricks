pub mod back_navigation;
pub mod builder;
pub mod frunk;
pub mod param_list;
pub mod step;

pub use frunk::coproduct::{CNil, CoprodInjector, Coproduct};
pub use frunk::hlist::{HCons, HList, HNil};
pub use frunk::plucker::{Here, Plucker, There};
pub use frunk::to_ref::ToRef;

// cargo doc --no-deps --package type_process_builder --features docs
/// View the diagrams
#[cfg(feature = "docs")]
#[doc = simple_mermaid::mermaid!("../doc/brrricks_app_session_flow.mmd")]
#[doc = simple_mermaid::mermaid!("../doc/process_builder_states.mmd")]
#[cfg_attr(not(feature = "docs"), doc = "")]
pub mod documentation_diagrams {
  use crate::back_navigation::create_back_token;
  use crate::builder::{
    FinalizedProcess, FormContext, PreviousRunYieldedAt, RunOutcome, RunnableProcess, SessionContext, StepIndex,
  };
  use crate::step::ProcessMessages;
  use std::ops::Not;

  pub struct SessionState {
    pub session_context: SessionContext,
    pub previous_run_yielded_at: PreviousRunYieldedAt,
    pub form_context: Option<FormContext>,
    pub visited_form_steps: Vec<StepIndex>,
  }

  #[allow(clippy::missing_panics_doc)]
  pub async fn in_memory_process_runner<Messages: ProcessMessages<FinalMessage = String, FormMessage = String>>(
    process: &RunnableProcess<impl FinalizedProcess<Messages = Messages>>,
    state: &mut SessionState,
    user_input: &str,
  ) -> Result<String, String> {
    let back_token = if state.visited_form_steps.is_empty() { None } else { Some(create_back_token()) };
    let mut run_outcome = process
      .resume_run(
        state.session_context.clone(),
        state.previous_run_yielded_at.clone(),
        user_input.into(),
        state.form_context.clone(),
        back_token,
      )
      .await
      .expect("Test failed");

    let mut was_backed = false;
    if let RunOutcome::Back = run_outcome {
      was_backed = true;
      state.visited_form_steps.pop();
      let target_step_index = *state.visited_form_steps.last().expect("Cannot go back further");
      let back_token = if state.visited_form_steps.len() > 1 { Some(create_back_token()) } else { None };
      run_outcome = process
        .resume_run(
          state.session_context.clone(),
          PreviousRunYieldedAt(target_step_index),
          String::new(),
          None::<FormContext>,
          back_token,
        )
        .await
        .expect("Test failed");
    }
    match run_outcome {
      RunOutcome::Yield(msg, value, yielded_at, context) => {
        state.session_context = value;
        state.previous_run_yielded_at = PreviousRunYieldedAt(yielded_at.0);
        state.form_context = Some(context);
        if was_backed.not() {
          state.visited_form_steps.push(yielded_at.0);
        }
        Ok(msg)
      },
      RunOutcome::RetryUserInput(msg, context) => {
        state.form_context = Some(context);
        Ok(msg)
      },
      RunOutcome::Finish(msg) => Err(msg),
      RunOutcome::Back => unreachable!(),
    }
  }
}
