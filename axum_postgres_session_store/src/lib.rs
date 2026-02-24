use serde_value::Value;
use sqlx::{Executor, PgPool, Row};
use type_process_builder::builder::{CurrentRunYieldedAt, FinalizedProcess, ParamUID, PreviousRunYieldedAt, RunnableProcess};
use type_process_builder::step::FailedInputValidationAttempts;

/// previous_run_yielded_at can be used for session context caching
pub async fn create_session_context_table<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  ordered_all_unique_param_uids: &Vec<ParamUID>,
) -> Result<(), sqlx::Error> {
  let mut columns = String::new();
  for col in ordered_all_unique_param_uids {
    let name: &u32 = col;
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

pub async fn create_session_context<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
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

  let process_name = process.get_name();
  let process_version = process.get_version();
  let sql = format!(
    "INSERT INTO session_store.{process_name}_{process_version} ({}) VALUES ({}) RETURNING id;",
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

use std::fmt::Write;

pub struct GetSessionContextQuery(String);
/// Builds:
/// SELECT "previous_run_yielded_at","failed_input_validation_attempts","0","1","2"
/// FROM session_store.process_version
/// WHERE id = $1
pub fn build_get_session_context_query<Process: FinalizedProcess>(
  process: &RunnableProcess<Process>,
  ordered_all_unique_param_uids: &Vec<ParamUID>,
) -> GetSessionContextQuery {
  let mut sql = String::with_capacity(64 + ordered_all_unique_param_uids.len() * 8);

  write!(
    sql,
    "SELECT \"previous_run_yielded_at\",\"failed_input_validation_attempts\""
  )
  .unwrap();
  for uid in ordered_all_unique_param_uids {
    sql.push(',');
    write!(sql, "\"{uid}\"").unwrap();
  }

  let process_name = process.get_name();
  let process_version = process.get_version();
  write!(
    sql,
    " FROM session_store.{process_name}_{process_version} WHERE id = $1"
  )
  .unwrap();
  GetSessionContextQuery(sql)
}

pub async fn get_session_context(
  pool: &PgPool,
  sql: &GetSessionContextQuery,
  session_id: i64,
  ordered_all_unique_param_uids: &Vec<ParamUID>,
) -> Result<(PreviousRunYieldedAt, FailedInputValidationAttempts, Vec<(u32, Value)>), sqlx::Error> {
  let row = sqlx::query(&sql.0).bind(session_id).fetch_one(pool).await?;

  let previous_run_yielded_at = PreviousRunYieldedAt(row.try_get(0)?);
  let failed_input_validation_attempts = FailedInputValidationAttempts(row.try_get::<i16, _>(1)? as u8);

  let mut session_context = Vec::with_capacity(ordered_all_unique_param_uids.len());
  for idx_and_param_uid in ordered_all_unique_param_uids.iter().enumerate() {
    let value: sqlx::types::Json<Value> = row.try_get(idx_and_param_uid.0 + 2)?;
    session_context.push((idx_and_param_uid.1.clone(), value.0));
  }

  Ok((
    previous_run_yielded_at,
    failed_input_validation_attempts,
    session_context,
  ))
}
