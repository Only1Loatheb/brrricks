use type_process_builder::builder::{
  FinalizedProcess, FormContext, PreviousRunYieldedAt, RunOutcome, RunnableProcess, SessionContext, StepIndex,
};
use type_process_builder::step::{BackToken, ProcessMessages};

pub struct SessionState {
  pub form_context: Option<FormContext>,
  pub visited_form_steps: Vec<(StepIndex, SessionContext)>,
}

#[allow(clippy::missing_panics_doc)]
pub async fn in_memory_process_runner<Messages: ProcessMessages<FinalMessage = String, FormMessage = String>>(
  process: &RunnableProcess<impl FinalizedProcess<Messages = Messages>>,
  state: &mut SessionState,
  user_input: &str,
) -> Result<String, String> {
  let back_token = if state.visited_form_steps.len() > 1 { Some(BackToken::create()) } else { None };
  let (previous_step, active_session_context) =
    state.visited_form_steps.last().map_or((StepIndex::MIN, &[][..]), |(step, ctx)| (*step, ctx.as_slice()));
  let previous_run_yielded_at = PreviousRunYieldedAt(previous_step);
  let mut run_outcome = process
    .resume_run(
      active_session_context,
      previous_run_yielded_at,
      user_input.into(),
      state.form_context.as_deref(),
      back_token,
    )
    .await
    .expect("Failed to resume process");

  let mut was_backed = false;
  if let RunOutcome::Back = run_outcome {
    was_backed = true;
    state.visited_form_steps.pop();
    let (target_step_index, target_session_context) =
      state.visited_form_steps.last().map(|(step, ctx)| (*step, ctx.as_slice())).expect("Cannot go back further");
    let back_token = if state.visited_form_steps.len() > 1 { Some(BackToken::create()) } else { None };
    run_outcome = process
      .resume_run(target_session_context, PreviousRunYieldedAt(target_step_index), String::new(), None, back_token)
      .await
      .expect("Failed to resume process");
  }
  match run_outcome {
    RunOutcome::Yield(msg, value, yielded_at, context) => {
      state.form_context = Some(context);
      if was_backed {
        if let Some(last) = state.visited_form_steps.last_mut() {
          *last = (yielded_at.0, value);
        }
      } else {
        state.visited_form_steps.push((yielded_at.0, value));
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
