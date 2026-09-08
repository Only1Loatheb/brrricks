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
      id UUID PRIMARY KEY,
      form_context BYTEA,
      visited_form_steps BYTEA NOT NULL,
      session_context BYTEA NOT NULL)",
  );

  pool.execute(sql.as_str()).await?;

  Ok(())
}

pub async fn create_session_context_batch<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  ids: &[Uuid],
  current_run_yielded_ats: &[CurrentRunYieldedAt],
  form_contexts: &[MaybeFormContext],
  session_contexts: &[SessionContext],
) -> Result<(), sqlx::Error> {
  let table_name = qualified_table_name(process);
  let sql = format!(
    "INSERT INTO {table_name} (id, form_context, visited_form_steps, session_context) SELECT * FROM UNNEST($1::uuid[], $2::bytea[], $3::bytea[], $4::bytea[]);"
  );

  let visited_steps_bytes_list = current_run_yielded_ats
    .iter()
    .map(|at| postcard::to_allocvec(&vec![at.0]).map_err(|e| sqlx::Error::Encode(Box::new(e))))
    .collect::<Result<Vec<_>, _>>()?;

  sqlx::query(&sql)
    .bind(ids)
    .bind(form_contexts)
    .bind(visited_steps_bytes_list)
    .bind(session_contexts)
    .execute(pool)
    .await?;

  Ok(())
}

pub async fn create_session_context<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: Uuid,
  current_run_yielded_at: CurrentRunYieldedAt,
  form_context: MaybeFormContext,
  session_context: SessionContext,
) -> Result<(), sqlx::Error> {
  create_session_context_batch(pool, process, &[id], &[current_run_yielded_at], &[form_context], &[session_context])
    .await
}



#[derive(Clone)]
pub struct GetSessionContextQuery(String);
/// Builds:
/// SELECT `id`, `form_context`, `visited_form_steps`, `session_context`
/// FROM `session_store.process_version`
/// WHERE id = `ANY($1::uuid`[])
pub fn build_get_session_context_query<Process: FinalizedProcess>(
  process: &RunnableProcess<Process>,
) -> GetSessionContextQuery {
  let table_name = qualified_table_name(process);
  let sql = format!(
    "SELECT id, form_context, visited_form_steps, session_context FROM {table_name} WHERE id = ANY($1::uuid[])"
  );
  GetSessionContextQuery(sql)
}

pub async fn get_session_context(
  pool: &PgPool,
  sql: &GetSessionContextQuery,
  session_id: Uuid,
) -> Result<(MaybeFormContext, Vec<i32>, SessionContext), sqlx::Error> {
  let mut results = get_session_context_batch(pool, sql, &[session_id]).await?;
  let (_id, form_context, visited_form_steps, session_context) = results.pop().ok_or(sqlx::Error::RowNotFound)?;
  Ok((form_context, visited_form_steps, session_context))
}

pub async fn get_session_context_batch(
  pool: &PgPool,
  sql: &GetSessionContextQuery,
  session_ids: &[Uuid],
) -> Result<Vec<(Uuid, MaybeFormContext, Vec<i32>, SessionContext)>, sqlx::Error> {
  let rows = sqlx::query(&sql.0).bind(session_ids).fetch_all(pool).await?;

  let mut results = Vec::with_capacity(rows.len());
  for row in rows {
    let id = row.try_get::<Uuid, _>(0)?;
    let form_context = row.try_get::<Option<Vec<u8>>, _>(1)?;
    let visited_form_steps_bytes = row.try_get::<Vec<u8>, _>(2)?;
    let visited_form_steps: Vec<i32> =
      postcard::from_bytes(&visited_form_steps_bytes).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    let session_context = row.try_get::<Vec<u8>, _>(3)?;
    results.push((id, form_context, visited_form_steps, session_context));
  }

  Ok(results)
}

pub async fn delete_session_context_batch<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  ids: &[Uuid],
) -> Result<u64, sqlx::Error> {
  let table_name = qualified_table_name(process);

  let sql = format!(r"DELETE FROM {table_name} WHERE id = ANY($1::uuid[])");

  let result = sqlx::query(&sql).bind(ids).execute(pool).await?;

  Ok(result.rows_affected())
}

pub async fn delete_session_context<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: Uuid,
) -> Result<u64, sqlx::Error> {
  delete_session_context_batch(pool, process, &[id]).await
}

fn qualified_table_name<Process: FinalizedProcess>(process: &RunnableProcess<Process>) -> String {
  let process_name = process.get_name();
  let process_version = process.get_version();
  format!("session_store.{process_name}_{process_version}")
}

pub async fn increment_failed_input_validation_attempts_batch<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  ids: &[Uuid],
  form_contexts: &[Vec<u8>],
) -> Result<PgQueryResult, sqlx::Error> {
  let table_name = qualified_table_name(process);
  let sql = format!(
    r"UPDATE {table_name} AS t SET form_context = u.form_context FROM UNNEST($1::uuid[], $2::bytea[]) AS u(id, form_context) WHERE t.id = u.id"
  );
  sqlx::query(&sql).bind(ids).bind(form_contexts).execute(pool).await
}

pub async fn increment_failed_input_validation_attempts<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: Uuid,
  form_context: Vec<u8>,
) -> Result<PgQueryResult, sqlx::Error> {
  increment_failed_input_validation_attempts_batch(pool, process, &[id], &[form_context]).await
}

pub async fn update_session_context_batch<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  ids: &[Uuid],
  form_contexts: &[MaybeFormContext],
  visited_form_steps_list: &[Vec<i32>],
  session_contexts: &[SessionContext],
) -> Result<(), sqlx::Error> {
  let table_name = qualified_table_name(process);

  let sql = format!(
    "UPDATE {table_name} AS t SET form_context = u.form_context, visited_form_steps = u.visited_form_steps, session_context = u.session_context FROM UNNEST($1::uuid[], $2::bytea[], $3::bytea[], $4::bytea[]) AS u(id, form_context, visited_form_steps, session_context) WHERE t.id = u.id;"
  );

  let visited_steps_bytes_list = visited_form_steps_list
    .iter()
    .map(|steps| postcard::to_allocvec(steps).map_err(|e| sqlx::Error::Encode(Box::new(e))))
    .collect::<Result<Vec<_>, _>>()?;

  sqlx::query(&sql)
    .bind(ids)
    .bind(form_contexts)
    .bind(visited_steps_bytes_list)
    .bind(session_contexts)
    .execute(pool)
    .await?;

  Ok(())
}

pub async fn update_session_context<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  id: Uuid,
  form_context: MaybeFormContext,
  visited_form_steps: Vec<i32>,
  session_context: SessionContext,
) -> Result<(), sqlx::Error> {
  update_session_context_batch(pool, process, &[id], &[form_context], &[visited_form_steps], &[session_context]).await
}
