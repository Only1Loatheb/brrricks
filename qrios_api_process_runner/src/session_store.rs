use sqlx::postgres::PgQueryResult;
use sqlx::{Executor, PgPool, Row};
use type_process_builder::builder::{FinalizedProcess, MaybeFormContext, RunnableProcess, SessionContext, StepIndex};
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
      id UUID PRIMARY KEY,
      form_context BYTEA,
      visited_form_steps BYTEA NOT NULL,
      session_context BYTEA NOT NULL)",
  );

  pool.execute(sql.as_str()).await?;

  Ok(())
}

pub async fn create_session_context<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: Uuid,
  visited_form_steps: &[(StepIndex, SessionContext)],
  form_context: MaybeFormContext,
  session_context: &SessionContext,
) -> Result<(), sqlx::Error> {
  let table_name = qualified_table_name(process);
  let sql = format!(
    "INSERT INTO {table_name} (id, form_context, visited_form_steps, session_context) VALUES ($1, $2, $3, $4);"
  );

  let visited_steps_bytes = postcard::to_allocvec(visited_form_steps).map_err(|e| sqlx::Error::Encode(Box::new(e)))?;

  sqlx::query(&sql).bind(id).bind(form_context).bind(visited_steps_bytes).bind(session_context).execute(pool).await?;

  Ok(())
}

pub struct GetSessionContextQuery(String);

pub fn build_get_session_context_query<Process: FinalizedProcess>(
  process: &RunnableProcess<Process>,
) -> GetSessionContextQuery {
  let table_name = qualified_table_name(process);
  let sql = format!("SELECT form_context, visited_form_steps, session_context FROM {table_name} WHERE id = $1");
  GetSessionContextQuery(sql)
}

pub async fn get_session_context(
  pool: &PgPool,
  sql: &GetSessionContextQuery,
  session_id: Uuid,
) -> Result<(MaybeFormContext, Vec<(StepIndex, SessionContext)>, SessionContext), sqlx::Error> {
  let row = sqlx::query(&sql.0).bind(session_id).fetch_one(pool).await?;

  let form_context = row.try_get::<Option<Vec<u8>>, _>(0)?;
  let visited_form_steps_bytes = row.try_get::<Vec<u8>, _>(1)?;
  let visited_form_steps: Vec<(StepIndex, SessionContext)> =
    postcard::from_bytes(&visited_form_steps_bytes).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
  let session_context = row.try_get::<Vec<u8>, _>(2)?;

  Ok((form_context, visited_form_steps, session_context))
}

pub async fn delete_session_context<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: Uuid,
) -> Result<u64, sqlx::Error> {
  let table_name = qualified_table_name(process);

  let sql = format!(r"DELETE FROM {table_name} WHERE id = $1;");

  let result = sqlx::query(&sql).bind(id).execute(pool).await?;

  Ok(result.rows_affected())
}

pub async fn increment_failed_input_validation_attempts<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: Uuid,
  form_context: Vec<u8>,
) -> Result<PgQueryResult, sqlx::Error> {
  let table_name = qualified_table_name(process);
  let sql = format!(r"UPDATE {table_name} SET form_context = $1 WHERE id = $2;");
  sqlx::query(&sql).bind(form_context).bind(id).execute(pool).await
}

pub async fn update_session_context<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: Uuid,
  form_context: MaybeFormContext,
  visited_form_steps: &[(StepIndex, SessionContext)],
  session_context: &SessionContext,
) -> Result<(), sqlx::Error> {
  let table_name = qualified_table_name(process);

  let sql =
    format!("UPDATE {table_name} SET form_context = $1, visited_form_steps = $2, session_context = $3 WHERE id = $4;");

  let visited_steps_bytes = postcard::to_allocvec(visited_form_steps).map_err(|e| sqlx::Error::Encode(Box::new(e)))?;

  sqlx::query(&sql).bind(form_context).bind(visited_steps_bytes).bind(session_context).bind(id).execute(pool).await?;

  Ok(())
}

fn qualified_table_name<Process: FinalizedProcess>(process: &RunnableProcess<Process>) -> String {
  let process_name = process.get_name();
  let process_version = process.get_version();
  format!("session_store.{process_name}_{process_version}")
}
