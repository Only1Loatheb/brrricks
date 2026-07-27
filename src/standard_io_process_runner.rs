use std::io;
use std::io::Write;
use type_process_builder::builder::{FinalizedProcess, PreviousRunYieldedAt, RunOutcome, RunnableProcess, StepIndex};
use type_process_builder::step::ProcessMessages;

pub(crate) struct Message(pub String);

pub(crate) struct Messages;
impl ProcessMessages for Messages {
  type FormMessage = Message;
  type FinalMessage = Message;
}

pub(crate) async fn standard_io_process_runner(
  demo_process: RunnableProcess<impl FinalizedProcess<Messages = Messages>>,
) -> io::Result<()> {
  let mut previous_run_produced = Vec::new();
  let mut previous_run_yielded_at = PreviousRunYieldedAt(StepIndex::MIN);
  let mut form_context = None;
  let mut visited_form_steps = Vec::new();

  print!("Enter a shortcode");
  loop {
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let user_input = input.trim().to_owned();

    let back_navigation_available = visited_form_steps.len() > 1;
    let mut run_outcome = demo_process
      .resume_run(
        previous_run_produced.clone(),
        previous_run_yielded_at.clone(),
        user_input,
        form_context.clone(),
        back_navigation_available,
      )
      .await
      .map_err(io::Error::other)?;

    if let RunOutcome::Back = run_outcome {
      visited_form_steps.pop();
      let target_step_index = *visited_form_steps.last().ok_or_else(|| io::Error::other("Cannot go back further"))?;
      let back_navigation_available = visited_form_steps.len() > 1;
      run_outcome = demo_process
        .resume_run(
          previous_run_produced.clone(),
          PreviousRunYieldedAt(target_step_index),
          "".to_string(),
          None,
          back_navigation_available,
        )
        .await
        .map_err(io::Error::other)?;
    }

    match run_outcome {
      RunOutcome::Yield(msg, value, yielded_at, context) => {
        previous_run_produced = value;
        previous_run_yielded_at = PreviousRunYieldedAt(yielded_at.0);
        form_context = Some(context);
        if visited_form_steps.last() != Some(&yielded_at.0) {
          visited_form_steps.push(yielded_at.0);
        }
        println!("yielded: {}", msg.0);
      },
      RunOutcome::RetryUserInput(msg, context) => {
        form_context = Some(context);
        println!("retry: {}", msg.0);
      },
      RunOutcome::Finish(msg) => {
        println!("finished: {}", msg.0);
        return Ok(());
      },
      RunOutcome::Back => unreachable!(),
    }
  }
}
