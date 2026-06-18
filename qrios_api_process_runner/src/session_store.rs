use sqlx::{Executor, PgPool, Row};
use type_process_builder::builder::{CurrentRunYieldedAt, FinalizedProcess, ParamUID, PreviousRunYieldedAt, MaybeFormContext, RunnableProcess, SessionContext};

pub async fn create_session_context_table<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  ordered_all_unique_param_uids: &Vec<ParamUID>,
) -> Result<(), sqlx::Error> {
  let mut param_columns = String::new();
  for col in ordered_all_unique_param_uids {
    let name: &u32 = col;
    param_columns.push_str(&format!(",\"{name}\" BYTEA"));
  }

  let table_name = table_name(process);
  let sql = format!(
    r#"
    CREATE TABLE IF NOT EXISTS {table_name} (
      id BIGSERIAL PRIMARY KEY,
      previous_run_yielded_at INTEGER NOT NULL,
      form_context BYTEA{param_columns})"#,
  );

  pool.execute(sql.as_str()).await?;

  Ok(())
}

pub async fn create_session_context<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  current_run_yielded_at: CurrentRunYieldedAt,
  form_context: MaybeFormContext,
  session_context: SessionContext,
) -> Result<i64, sqlx::Error> {
  let mut columns = vec!["previous_run_yielded_at".to_string(), "form_context".to_string()];
  let mut placeholders = vec!["$1".to_string(), "$2".to_string()];

  for (i, (col, _)) in session_context.iter().enumerate() {
    columns.push(format!("\"{col}\""));
    placeholders.push(format!("${}", i + 3));
  }

  let table_name = table_name(process);
  let sql =
    format!("INSERT INTO {table_name} ({}) VALUES ({}) RETURNING id;", columns.join(", "), placeholders.join(", "));

  let mut query = sqlx::query(&sql).bind(current_run_yielded_at.0).bind(form_context);

  for (_, value) in session_context {
    query = query.bind(value);
  }

  query.fetch_one(pool).await?.try_get("id")
}

use sqlx::postgres::PgQueryResult;

pub struct GetSessionContextQuery(String);
/// Builds:
/// SELECT "previous_run_yielded_at","form_context","0","1","2"
/// FROM session_store.process_version
/// WHERE id = $1
pub fn build_get_session_context_query<Process: FinalizedProcess>(
  process: &RunnableProcess<Process>,
  ordered_all_unique_param_uids: &Vec<ParamUID>,
) -> GetSessionContextQuery {
  let mut sql = String::with_capacity(64 + ordered_all_unique_param_uids.len() * 8);

  sql.push_str("SELECT \"previous_run_yielded_at\",\"form_context\"");
  for uid in ordered_all_unique_param_uids {
    sql.push_str(&format!(",\"{uid}\""));
  }

  let table_name = table_name(process);
  sql.push_str(&format!(" FROM {table_name} WHERE id = $1"));
  GetSessionContextQuery(sql)
}

pub async fn get_session_context(
  pool: &PgPool,
  sql: &GetSessionContextQuery,
  session_id: i64,
  ordered_all_unique_param_uids: &[ParamUID],
) -> Result<(PreviousRunYieldedAt, MaybeFormContext, SessionContext), sqlx::Error> {
  let row = sqlx::query(&sql.0).bind(session_id).fetch_one(pool).await?;

  let previous_run_yielded_at = PreviousRunYieldedAt(row.try_get(0)?);
  let form_context = row.try_get::<Vec<u8>, _>(1)?;

  let mut session_context = Vec::with_capacity(ordered_all_unique_param_uids.len());
  for idx_and_param_uid in ordered_all_unique_param_uids.iter().enumerate() {
    if let Ok(value) = row.try_get::<Vec<u8>, _>(idx_and_param_uid.0 + 2) {
      session_context.push((*idx_and_param_uid.1, value));
    }
  }

  Ok((previous_run_yielded_at, form_context, session_context))
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
      SET form_context = form_context + 1
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
  form_context: MaybeFormContext,
  params_to_store: SessionContext,
  params_to_remove: Vec<u32>,
) -> Result<(), sqlx::Error> {
  let mut assignments =
    vec!["previous_run_yielded_at = $1".to_string(), "form_context = $2".to_string()];

  for (i, (col, _)) in params_to_store.iter().enumerate() {
    assignments.push(format!("\"{}\" = ${}", col, i + 3));
  }

  // We need to remove stale value so the column will be interpreted as unset when determining already_stored_params
  // in the next session interaction.
  // Clearing params missing from session context is necessary when using the same param in split case and reusing it
  // after multiple execution path join into common continuation.
  for col in &params_to_remove {
    assignments.push(format!("\"{col}\" = NULL"));
  }

  let table_name = table_name(process);

  let where_placeholder = params_to_store.len() + 3;

  let sql = format!("UPDATE {table_name} SET {} WHERE id = ${};", assignments.join(", "), where_placeholder);

  let mut query = sqlx::query(&sql).bind(current_run_yielded_at.0).bind(form_context);

  for (_, value) in params_to_store {
    query = query.bind(value);
  }

  query = query.bind(id);

  query.execute(pool).await?;

  Ok(())
}
