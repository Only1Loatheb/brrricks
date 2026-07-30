use bricks::build_demo_process;
use bricks::standard_io_process_runner::update_session_context;
use type_process_builder::back_navigation::create_back_token;
use type_process_builder::builder::{BackToken, PreviousRunYieldedAt, RunOutcome, StepIndex};

#[tokio::test]
async fn test_demo_process_predefined_amount() {
  let process = build_demo_process();

  // Step 1: Entry
  let outcome =
    process.resume_run(vec![], PreviousRunYieldedAt(StepIndex::MIN), "*123#".to_string(), None, None).await.unwrap();

  let RunOutcome::Yield(msg, session_context, yielded_at, form_context) = outcome else {
    panic!("Expected Yield from SelectAmountSource");
  };
  assert!(msg.0.contains("Enter 1 for 100 or 2 for custom amount"));

  // Step 2: Select predefined amount "1"
  let outcome = process
    .resume_run(session_context, PreviousRunYieldedAt(yielded_at.0), "1".to_string(), Some(form_context), None)
    .await
    .unwrap();

  let RunOutcome::Finish(msg) = outcome else {
    panic!("Expected Finish after choosing option 1");
  };
  assert_eq!(msg.0, "The amount was: 100. Good bye!");
}

#[tokio::test]
async fn test_demo_process_custom_amount() {
  let process = build_demo_process();

  // Step 1: Entry
  let outcome = process
    .resume_run(vec![], PreviousRunYieldedAt(StepIndex::MIN), "*123#".to_string(), None, None::<BackToken>)
    .await
    .unwrap();

  let RunOutcome::Yield(_, session_context, yielded_at, form_context) = outcome else {
    panic!("Expected Yield from SelectAmountSource");
  };

  // Step 2: Select custom amount "2"
  let outcome = process
    .resume_run(
      session_context.clone(),
      PreviousRunYieldedAt(yielded_at.0),
      "2".to_string(),
      Some(form_context),
      None::<BackToken>,
    )
    .await
    .unwrap();

  let RunOutcome::Yield(msg, mut session_context_2, yielded_at_2, form_context_2) = outcome else {
    panic!("Expected Yield from AmountForm");
  };
  assert!(msg.0.contains("Enter a number"));

  update_session_context(&mut session_context_2, session_context);

  // Step 3: Enter custom amount "50"
  let outcome = process
    .resume_run(
      session_context_2,
      PreviousRunYieldedAt(yielded_at_2.0),
      "50".to_string(),
      Some(form_context_2),
      Some(create_back_token()),
    )
    .await
    .unwrap();

  let RunOutcome::Finish(msg) = outcome else {
    panic!("Expected Finish after custom amount 50");
  };
  assert_eq!(msg.0, "The amount was: 50. Good bye!");
}

#[tokio::test]
async fn test_demo_process_retry_splitter() {
  let process = build_demo_process();

  let outcome = process
    .resume_run(vec![], PreviousRunYieldedAt(StepIndex::MIN), "*123#".to_string(), None, None::<BackToken>)
    .await
    .unwrap();

  let RunOutcome::Yield(_, session_context, yielded_at, form_context) = outcome else {
    panic!("Expected Yield");
  };

  // Invalid input "invalid"
  let outcome = process
    .resume_run(
      session_context.clone(),
      PreviousRunYieldedAt(yielded_at.0),
      "invalid".to_string(),
      Some(form_context),
      None::<BackToken>,
    )
    .await
    .unwrap();

  let RunOutcome::RetryUserInput(msg, retry_context) = outcome else {
    panic!("Expected RetryUserInput");
  };
  assert_eq!(msg.0, "not 1 or 2");

  // Retry with valid choice "1"
  let outcome = process
    .resume_run(
      session_context,
      PreviousRunYieldedAt(yielded_at.0),
      "1".to_string(),
      Some(retry_context),
      None::<BackToken>,
    )
    .await
    .unwrap();

  let RunOutcome::Finish(msg) = outcome else {
    panic!("Expected Finish");
  };
  assert_eq!(msg.0, "The amount was: 100. Good bye!");
}

#[tokio::test]
async fn test_demo_process_retry_amount_form() {
  let process = build_demo_process();

  let outcome = process
    .resume_run(vec![], PreviousRunYieldedAt(StepIndex::MIN), "*123#".to_string(), None, None::<BackToken>)
    .await
    .unwrap();

  let RunOutcome::Yield(_, session_context, yielded_at, form_context) = outcome else {
    panic!("Expected Yield");
  };

  // Choose custom amount "2"
  let outcome = process
    .resume_run(
      session_context,
      PreviousRunYieldedAt(yielded_at.0),
      "2".to_string(),
      Some(form_context),
      None::<BackToken>,
    )
    .await
    .unwrap();

  let RunOutcome::Yield(_, session_context_2, yielded_at_2, form_context_2) = outcome else {
    panic!("Expected Yield from AmountForm");
  };

  // Invalid amount "abc"
  let outcome = process
    .resume_run(
      session_context_2.clone(),
      PreviousRunYieldedAt(yielded_at_2.0),
      "abc".to_string(),
      Some(form_context_2),
      Some(create_back_token()),
    )
    .await
    .unwrap();

  let RunOutcome::RetryUserInput(msg, retry_context) = outcome else {
    panic!("Expected RetryUserInput from AmountForm");
  };
  assert_eq!(msg.0, "Invalid number");

  // Retry with valid amount "250"
  let outcome = process
    .resume_run(
      session_context_2,
      PreviousRunYieldedAt(yielded_at_2.0),
      "250".to_string(),
      Some(retry_context),
      Some(create_back_token()),
    )
    .await
    .unwrap();

  let RunOutcome::Finish(msg) = outcome else {
    panic!("Expected Finish after valid retry amount");
  };
  assert_eq!(msg.0, "The amount was: 250. Good bye!");
}

#[tokio::test]
async fn test_demo_process_back_navigation() {
  let process = build_demo_process();

  // Step 1: Entry
  let outcome = process
    .resume_run(vec![], PreviousRunYieldedAt(StepIndex::MIN), "*123#".to_string(), None, None::<BackToken>)
    .await
    .unwrap();

  let RunOutcome::Yield(_, session_context, yielded_at, form_context) = outcome else {
    panic!("Expected Yield from SelectAmountSource");
  };

  // Step 2: Choose option "2" -> AmountForm
  let outcome = process
    .resume_run(
      session_context.clone(),
      PreviousRunYieldedAt(yielded_at.0),
      "2".to_string(),
      Some(form_context),
      None::<BackToken>,
    )
    .await
    .unwrap();

  let RunOutcome::Yield(_, mut session_context_2, yielded_at_2, form_context_2) = outcome else {
    panic!("Expected Yield from AmountForm");
  };

  update_session_context(&mut session_context_2, session_context);

  // Input "0" at AmountForm triggers Back
  let outcome = process
    .resume_run(
      session_context_2.clone(),
      PreviousRunYieldedAt(yielded_at_2.0),
      "0".to_string(),
      Some(form_context_2),
      Some(create_back_token()),
    )
    .await
    .unwrap();

  assert!(matches!(outcome, RunOutcome::Back));

  // Retrace back to SelectAmountSource (step 1)
  let outcome = process
    .resume_run(session_context_2.clone(), PreviousRunYieldedAt(yielded_at.0), String::new(), None, None::<BackToken>)
    .await
    .unwrap();

  let RunOutcome::Yield(msg, session_context_3, yielded_at_3, form_context_3) = outcome else {
    panic!("Expected Yield when retracing back to SelectAmountSource");
  };
  assert!(msg.0.contains("Enter 1 for 100 or 2 for custom amount"));
  assert_eq!(yielded_at_3.0, yielded_at.0);

  // From back-navigated state, choose option "1"
  let outcome = process
    .resume_run(
      session_context_3,
      PreviousRunYieldedAt(yielded_at_3.0),
      "1".to_string(),
      Some(form_context_3),
      None::<BackToken>,
    )
    .await
    .unwrap();

  let RunOutcome::Finish(msg) = outcome else {
    panic!("Expected Finish after choosing option 1 post back navigation");
  };
  assert_eq!(msg.0, "The amount was: 100. Good bye!");
}
