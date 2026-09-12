use sqlx::postgres::PgQueryResult;
use sqlx::{Executor, PgPool, Row};
use type_process_builder::builder::{
  CurrentRunYieldedAt, FinalizedProcess, MaybeFormContext, RunnableProcess, SessionContext,
};
use uuid::Uuid;

pub async fn create_session_context_table<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
) -> Result<(), sqlx::Error> {
  sqlx::query("CREATE SCHEMA IF NOT EXISTS session_store").execute(pool).await?;

  let table_name = qualified_table_name(process);
  let sql = format!(
    r"
    CREATE TABLE IF NOT EXISTS {table_name} (
      id UUID NOT NULL,
      visited_form_step INTEGER NOT NULL,
      has_back BOOLEAN NOT NULL,
      form_context BYTEA,
      session_context BYTEA NOT NULL,
      PRIMARY KEY (id, visited_form_step))",
  );

  pool.execute(sql.as_str()).await?;

  Ok(())
}

pub async fn create_session_context<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: Uuid,
  current_run_yielded_at: CurrentRunYieldedAt,
  has_back: bool,
  form_context: MaybeFormContext,
  session_context: SessionContext,
) -> Result<(), sqlx::Error> {
  let table_name = qualified_table_name(process);
  let sql = format!(
    "INSERT INTO {table_name} (id, visited_form_step, has_back, form_context, session_context) VALUES ($1, $2, $3, $4, $5);"
  );

  sqlx::query(&sql)
    .bind(id)
    .bind(current_run_yielded_at.0)
    .bind(has_back)
    .bind(form_context)
    .bind(session_context)
    .execute(pool)
    .await?;

  Ok(())
}

pub struct GetSessionContextQuery(String);

pub fn build_get_session_context_query<Process: FinalizedProcess>(
  process: &RunnableProcess<Process>,
) -> GetSessionContextQuery {
  let table_name = qualified_table_name(process);
  let sql = format!(
    "SELECT form_context, session_context, has_back FROM {table_name} WHERE id = $1 AND visited_form_step = $2"
  );
  GetSessionContextQuery(sql)
}

pub async fn get_session_step_context(
  pool: &PgPool,
  sql: &GetSessionContextQuery,
  session_id: Uuid,
  visited_form_step: i32,
) -> Result<(MaybeFormContext, SessionContext, bool), sqlx::Error> {
  let row = sqlx::query(&sql.0).bind(session_id).bind(visited_form_step).fetch_one(pool).await?;

  let form_context = row.try_get::<Option<Vec<u8>>, _>(0)?;
  let session_context = row.try_get::<Vec<u8>, _>(1)?;
  let has_back: bool = row.try_get(2)?;

  Ok((form_context, session_context, has_back))
}

/// Deletes the current step row for `visited_form_step` from the session step stack in Postgres
/// and retrieves the target step context for back navigation.
///
/// Note: Ordering via `visited_form_step < $2 ORDER BY visited_form_step DESC LIMIT 1` relies on
/// process step enumeration [`FinalizedProcess::enumerate_steps`], which assigns monotonically increasing
/// step indices during process construction.
pub async fn pop_session_step_context_for_back_navigation<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: Uuid,
  visited_form_step: i32,
) -> Result<(MaybeFormContext, i32, SessionContext, bool), sqlx::Error> {
  let table_name = qualified_table_name(process);
  let sql = format!(
    r"
    WITH deleted AS (
      DELETE FROM {table_name} WHERE id = $1 AND visited_form_step = $2
    )
    SELECT visited_form_step, form_context, session_context, has_back
    FROM {table_name}
    WHERE id = $1 AND visited_form_step < $2
    ORDER BY visited_form_step DESC
    LIMIT 1"
  );
  let row = sqlx::query(&sql).bind(id).bind(visited_form_step).fetch_one(pool).await?;

  let visited_form_step: i32 = row.try_get(0)?;
  let form_context = row.try_get::<Option<Vec<u8>>, _>(1)?;
  let session_context = row.try_get::<Vec<u8>, _>(2)?;
  let has_back: bool = row.try_get(3)?;

  Ok((form_context, visited_form_step, session_context, has_back))
}

pub async fn delete_session_context<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: Uuid,
) -> Result<u64, sqlx::Error> {
  let table_name = qualified_table_name(process);

  let sql = format!(r"DELETE FROM {table_name} WHERE id = $1");

  let result = sqlx::query(&sql).bind(id).execute(pool).await?;

  Ok(result.rows_affected())
}

pub async fn update_session_step_context_for_next_interaction_with_same_form<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: Uuid,
  visited_form_step: i32,
  form_context: Vec<u8>,
) -> Result<PgQueryResult, sqlx::Error> {
  let table_name = qualified_table_name(process);
  let sql = format!(r"UPDATE {table_name} SET form_context = $1 WHERE id = $2 AND visited_form_step = $3");
  sqlx::query(&sql).bind(form_context).bind(id).bind(visited_form_step).execute(pool).await
}

/// Updates the existing database row for target step during back navigation.
/// Note: Assumes target step row already exists in database.
pub async fn update_session_step_context_for_back_navigation<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: Uuid,
  visited_form_step: i32,
  form_context: MaybeFormContext,
  session_context: SessionContext,
) -> Result<(), sqlx::Error> {
  let table_name = qualified_table_name(process);

  let sql = format!(
    "UPDATE {table_name} SET form_context = $1, session_context = $2 WHERE id = $3 AND visited_form_step = $4;"
  );

  sqlx::query(&sql).bind(form_context).bind(session_context).bind(id).bind(visited_form_step).execute(pool).await?;

  Ok(())
}

fn qualified_table_name<Process: FinalizedProcess>(process: &RunnableProcess<Process>) -> String {
  let process_name = process.get_name();
  let process_version = process.get_version();
  format!("session_store.{process_name}_{process_version}")
}
