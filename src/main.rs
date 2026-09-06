use bricks::{Messages, ShortcodeString, build_demo_process};
use std::io::Write;
use type_process_builder::builder::{FinalizedProcess, PreviousRunYieldedAt, RunnableProcess, StepIndex};
use type_process_builder::documentation_diagrams::{SessionState, in_memory_process_runner};
use type_process_builder::param_list::ParamList;
use type_process_builder::{HList, hlist};

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let process = build_demo_process();
  standard_io_process_runner(process).await
}

type EntryConsumes = HList!(ShortcodeString);

async fn standard_io_process_runner(
  process: RunnableProcess<impl FinalizedProcess<Messages = Messages, EntryConsumes = EntryConsumes>>,
) -> std::io::Result<()> {
  let mut state = SessionState {
    session_context: Vec::new(),
    previous_run_yielded_at: PreviousRunYieldedAt(StepIndex::MIN),
    form_context: None,
    visited_form_steps: Vec::new(),
  };
  println!("Enter a shortcode");
  loop {
    print!("> ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let user_input = input.trim();
    if state.session_context.is_empty() {
      let entry_consumes: EntryConsumes = hlist!(ShortcodeString(user_input.to_string()));
      state.session_context = entry_consumes.serialize().expect("Failed to serialize session context");
    }
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
