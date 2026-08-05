use bricks::{Messages, build_demo_process};
use std::io::Write;
use type_process_builder::builder::{FinalizedProcess, PreviousRunYieldedAt, RunnableProcess, StepIndex};
use type_process_builder::documentation_diagrams::{SessionState, in_memory_process_runner};

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let process = build_demo_process();
  standard_io_process_runner(process).await
}

async fn standard_io_process_runner(
  process: RunnableProcess<impl FinalizedProcess<Messages = Messages>>,
) -> std::io::Result<()> {
  let mut state = SessionState {
    session_context: Vec::new(),
    previous_run_yielded_at: PreviousRunYieldedAt(StepIndex::MIN),
    form_context: None,
    visited_form_steps: Vec::new(),
  };
  print!("Enter a shortcode");
  loop {
    print!("> ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let user_input = input.trim();
    let result = in_memory_process_runner(&process, &mut state, user_input).await;
    match result {
      Ok(msg) => {
        println!("{msg}");
      },
      Err(msg) => {
        println!("{msg}");
        return Ok(());
      },
    }
  }
}
