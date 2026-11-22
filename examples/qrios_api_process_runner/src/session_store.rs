use sqlx::{Executor, PgPool, Row};
use type_process_builder::builder::{FinalizedProcess, MaybeFormContext, RunnableProcess, SessionContext};
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

pub async fn create_session_context_batch<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  ids: &[Uuid],
  visited_form_steps: &[i32],
  has_backs: &[bool],
  form_contexts: &[MaybeFormContext],
  session_contexts: &[SessionContext],
) -> Result<(), sqlx::Error> {
  let table_name = qualified_table_name(process);
  let sql = format!(
    "INSERT INTO {table_name} (id, visited_form_step, has_back, form_context, session_context) SELECT * FROM UNNEST($1::uuid[], $2::int4[], $3::bool[], $4::bytea[], $5::bytea[]);"
  );

  sqlx::query(&sql)
    .bind(ids)
    .bind(visited_form_steps)
    .bind(has_backs)
    .bind(form_contexts)
    .bind(session_contexts)
    .execute(pool)
    .await?;

  Ok(())
}

pub async fn get_session_step_context_batch<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  ids: &[Uuid],
  visited_form_steps: &[i32],
) -> Result<Vec<(Uuid, i32, MaybeFormContext, SessionContext, bool)>, sqlx::Error> {
  let table_name = qualified_table_name(process);
  let sql = format!(
    "SELECT id, visited_form_step, form_context, session_context, has_back FROM {table_name} WHERE (id, visited_form_step) IN (SELECT * FROM UNNEST($1::uuid[], $2::int4[]))"
  );
  let rows = sqlx::query(&sql).bind(ids).bind(visited_form_steps).fetch_all(pool).await?;

  let mut results = Vec::with_capacity(rows.len());
  for row in rows {
    let id = row.try_get::<Uuid, _>(0)?;
    let visited_form_step = row.try_get::<i32, _>(1)?;
    let form_context = row.try_get::<Option<Vec<u8>>, _>(2)?;
    let session_context = row.try_get::<Vec<u8>, _>(3)?;
    let has_back = row.try_get::<bool, _>(4)?;
    results.push((id, visited_form_step, form_context, session_context, has_back));
  }

  Ok(results)
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

pub async fn update_session_step_context_for_next_interaction_with_same_form_batch<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  ids: &[Uuid],
  visited_form_steps: &[i32],
  form_contexts: &[Vec<u8>],
) -> Result<(), sqlx::Error> {
  let table_name = qualified_table_name(process);
  let sql = format!(
    r"UPDATE {table_name} AS t SET form_context = u.form_context FROM UNNEST($1::uuid[], $2::int4[], $3::bytea[]) AS u(id, visited_form_step, form_context) WHERE t.id = u.id AND t.visited_form_step = u.visited_form_step"
  );
  sqlx::query(&sql).bind(ids).bind(visited_form_steps).bind(form_contexts).execute(pool).await?;
  Ok(())
}

/// Updates the existing database row for target step during back navigation.
/// Note: Assumes target step row already exists in database.
pub async fn update_session_step_context_for_back_navigation_batch<Process: FinalizedProcess>(
  pool: &PgPool,
  process: &RunnableProcess<Process>,
  ids: &[Uuid],
  visited_form_steps: &[i32],
  form_contexts: &[MaybeFormContext],
  session_contexts: &[SessionContext],
) -> Result<(), sqlx::Error> {
  let table_name = qualified_table_name(process);

  let sql = format!(
    "UPDATE {table_name} AS t SET form_context = u.form_context, session_context = u.session_context FROM UNNEST($1::uuid[], $2::int4[], $3::bytea[], $4::bytea[]) AS u(id, visited_form_step, form_context, session_context) WHERE t.id = u.id AND t.visited_form_step = u.visited_form_step;"
  );

  sqlx::query(&sql).bind(ids).bind(visited_form_steps).bind(form_contexts).bind(session_contexts).execute(pool).await?;

  Ok(())
}

fn qualified_table_name<Process: FinalizedProcess>(process: &RunnableProcess<Process>) -> String {
  let process_name = process.get_name();
  let process_version = process.get_version();
  format!("session_store.{process_name}_{process_version}")
}
