use serde_value::Value;
use sqlx::{Executor, PgPool, Row};
use type_process_builder::builder::{
  CurrentRunYieldedAt, FinalizedProcess, ParamUID, PreviousRunYieldedAt, RunnableProcess,
};
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

  let table_name = table_name(process);
  let mut sql = format!(
    r#"
    CREATE TABLE IF NOT EXISTS {table_name} (
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
  let mut columns = vec!["previous_run_yielded_at".to_string(), "failed_input_validation_attempts".to_string()];
  let mut placeholders = vec!["$1".to_string(), "$2".to_string()];

  for (i, (col, _)) in session_context.iter().enumerate() {
    columns.push(format!("\"{}\"", col));
    placeholders.push(format!("${}", i + 3));
  }

  let table_name = table_name(process);
  let sql =
    format!("INSERT INTO {table_name} ({}) VALUES ({}) RETURNING id;", columns.join(", "), placeholders.join(", "));

  let mut query = sqlx::query(&sql).bind(current_run_yielded_at.0).bind(failed_input_validation_attempts.0 as i16);

  for (_, value) in session_context {
    query = query.bind(sqlx::types::Json(value));
  }

  query.fetch_one(pool).await?.try_get("id")
}

use sqlx::postgres::PgQueryResult;
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

  write!(sql, "SELECT \"previous_run_yielded_at\",\"failed_input_validation_attempts\"").unwrap();
  for uid in ordered_all_unique_param_uids {
    sql.push(',');
    write!(sql, "\"{uid}\"").unwrap();
  }

  let table_name = table_name(process);
  write!(sql, " FROM {table_name} WHERE id = $1").unwrap();
  GetSessionContextQuery(sql)
}

pub async fn get_session_context(
  pool: &PgPool,
  sql: &GetSessionContextQuery,
  session_id: i64,
  ordered_all_unique_param_uids: &[ParamUID],
) -> Result<(PreviousRunYieldedAt, FailedInputValidationAttempts, Vec<(u32, Value)>), sqlx::Error> {
  let row = sqlx::query(&sql.0).bind(session_id).fetch_one(pool).await?;

  let previous_run_yielded_at = PreviousRunYieldedAt(row.try_get(0)?);
  let failed_input_validation_attempts = FailedInputValidationAttempts(row.try_get::<i16, _>(1)? as u8);

  let mut session_context = Vec::with_capacity(ordered_all_unique_param_uids.len());
  for idx_and_param_uid in ordered_all_unique_param_uids.iter().enumerate() {
    if let Ok(value) = row.try_get::<sqlx::types::Json<Value>, _>(idx_and_param_uid.0 + 2) {
      session_context.push((*idx_and_param_uid.1, value.0));
    }
  }

  Ok((previous_run_yielded_at, failed_input_validation_attempts, session_context))
}

pub async fn delete_session_context<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: i64,
) -> Result<u64, sqlx::Error> {
  let table_name = table_name(process);

  let sql = format!(r#"DELETE FROM {table_name} WHERE id = $1"#);

  let result = sqlx::query(&sql).bind(id).execute(pool).await?;

  Ok(result.rows_affected())
}

fn table_name<Process: FinalizedProcess>(process: &RunnableProcess<Process>) -> String {
  let process_name = process.get_name();
  let process_version = process.get_version();
  format!("session_store.{process_name}_{process_version}")
}

pub async fn increment_failed_input_validation_attempts<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: i64,
) -> Result<PgQueryResult, sqlx::Error> {
  let table_name = table_name(process);

  let sql = format!(
    r#"
      UPDATE {table_name}
      SET failed_input_validation_attempts = failed_input_validation_attempts + 1
      WHERE id = $1
    "#
  );

  sqlx::query(&sql).bind(id).execute(pool).await
}

pub async fn update_session_context<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: i64,
  current_run_yielded_at: CurrentRunYieldedAt,
  failed_input_validation_attempts: FailedInputValidationAttempts,
  session_context: &[(u32, Value)],
) -> Result<(), sqlx::Error> {
  let mut assignments =
    vec!["previous_run_yielded_at = $1".to_string(), "failed_input_validation_attempts = $2".to_string()];

  for (i, (col, _)) in session_context.iter().enumerate() {
    assignments.push(format!("\"{}\" = ${}", col, i + 3));
  }

  let table_name = table_name(process);

  let where_placeholder = session_context.len() + 3;

  let sql = format!("UPDATE {table_name} SET {} WHERE id = ${};", assignments.join(", "), where_placeholder);

  let mut query = sqlx::query(&sql).bind(current_run_yielded_at.0).bind(failed_input_validation_attempts.0 as i16);

  for (_, value) in session_context {
    query = query.bind(sqlx::types::Json(value));
  }

  query = query.bind(id);

  query.execute(pool).await?;

  Ok(())
}
