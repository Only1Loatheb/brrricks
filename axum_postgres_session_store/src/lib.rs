use serde_value::Value;
use sqlx::{Executor, PgPool, Row};
use type_process_builder::builder::{CurrentRunYieldedAt, FinalizedProcess, RunnableProcess};
use type_process_builder::step::FailedInputValidationAttempts;

pub async fn create_table<Process: FinalizedProcess>(
  process: &RunnableProcess<Process>,
  pool: &PgPool,
) -> Result<(), sqlx::Error> {
  let mut columns = String::new();
  for col in process.all_param_uids() {
    let name: u32 = col;
    columns.push_str(&format!("\n\"{name}\" jsonb NULL,"));
  }

  let process_name = process.get_name();
  let process_version = process.get_version();
  let mut sql = format!(
    r#"
    CREATE TABLE IF NOT EXISTS session_store.{process_name}_{process_version} (
      id BIGINT PRIMARY KEY,
      previous_run_yielded_at INTEGER,
      failed_input_validation_attempts SMALLINT,{columns}"#,
  );

  // remove trailing comma and close the CREATE TABLE parentheses
  sql.pop();
  sql.push(')');

  pool.execute(sql.as_str()).await?;

  Ok(())
}

pub async fn store_session_context(
  pool: &PgPool,
  current_run_yielded_at: CurrentRunYieldedAt,
  failed_input_validation_attempts: FailedInputValidationAttempts,
  session_context: &[(u32, Value)],
) -> Result<i64, sqlx::Error> {
  let mut columns = vec![
    "previous_run_yielded_at".to_string(),
    "failed_input_validation_attempts".to_string(),
  ];
  let mut placeholders = vec!["$1".to_string(), "$2".to_string()];

  for (i, (col, _)) in session_context.iter().enumerate() {
    columns.push(format!("\"{}\"", col));
    placeholders.push(format!("${}", i + 3));
  }

  let sql = format!(
    "INSERT INTO my_table ({}) VALUES ({}) RETURNING id;",
    columns.join(", "),
    placeholders.join(", ")
  );

  let mut query = sqlx::query(&sql)
    .bind(current_run_yielded_at.0)
    .bind(failed_input_validation_attempts.0 as i16);

  for (_, value) in session_context {
    query = query.bind(sqlx::types::Json(value));
  }

  query.fetch_one(pool).await?.try_get("id")
}
