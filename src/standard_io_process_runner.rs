use std::io;
use std::io::Write;
use type_process_builder::builder::{FinalizedProcess, PreviousRunYieldedAt, RunOutcome, RunnableProcess, StepIndex};
use type_process_builder::step::FailedInputValidationAttempts;

pub(crate) async fn standard_io_process_runner(demo_process: RunnableProcess<impl FinalizedProcess>) -> io::Result<()> {
  let mut previous_run_produced = Vec::new();
  let mut previous_run_yielded_at = PreviousRunYieldedAt(StepIndex::MIN);
  let mut failed_attempts = FailedInputValidationAttempts(0);

  print!("Enter a shortcode");
  loop {
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let user_input = input.trim().to_owned();
    println!("{previous_run_produced:?}");
    match demo_process
      .resume_run(previous_run_produced.clone(), previous_run_yielded_at.clone(), user_input, failed_attempts.clone())
      .await
      .map_err(io::Error::other)?
    {
      RunOutcome::Yield(msg, value, yielded_at) => {
        previous_run_produced = value;
        previous_run_yielded_at = PreviousRunYieldedAt(yielded_at.0);
        failed_attempts = FailedInputValidationAttempts(0);
        println!("yielded: {}", msg.0);
      },
      RunOutcome::RetryUserInput(msg) => {
        failed_attempts = FailedInputValidationAttempts(failed_attempts.0 + 1);
        println!("retry: {}", msg.0);
      },
      RunOutcome::Finish(msg) => {
        println!("finished: {}", msg.0);
        return Ok(());
      },
    }
  }
}
