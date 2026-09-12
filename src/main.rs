use bricks::{Messages, ShortcodeString, build_demo_process};
use std::io::Write;
use type_process_builder::builder::{FinalizedProcess, RunnableProcess};
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
  let mut state = SessionState { form_context: None, visited_form_steps: Vec::new() };
  println!("Enter a shortcode");
  loop {
    print!("> ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let user_input = input.trim();
    if state.visited_form_steps.is_empty() {
      let entry_consumes: EntryConsumes = hlist!(ShortcodeString(user_input.to_string()));
      let init_context = entry_consumes.serialize_param_list().expect("Failed to serialize session context");
      state.visited_form_steps.push((type_process_builder::builder::StepIndex::MIN, init_context));
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
