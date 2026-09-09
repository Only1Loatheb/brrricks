use type_process_builder::builder::{FinalizedProcess, RunnableProcess, SessionContext};
use type_process_builder::documentation_diagrams::{SessionState, in_memory_process_runner};
use type_process_builder::step::ProcessMessages;

pub async fn test_process_messages<Messages: ProcessMessages<FinalMessage = String, FormMessage = String>>(
  process: &RunnableProcess<impl FinalizedProcess<Messages = Messages>>,
  initial_session_context: SessionContext,
  messages: Vec<&str>,
) {
  let mut state =
    SessionState { session_context: initial_session_context, form_context: None, visited_form_steps: Vec::new() };
  let mut index = 0;
  while index < messages.len() {
    let user_input = messages[index];
    index += 1;
    let result = in_memory_process_runner(process, &mut state, user_input).await;
    let expected_message = messages[index];
    index += 1;
    match result {
      Ok(msg) | Err(msg) => {
        assert_eq!(msg, expected_message);
      },
    }
  }
}
