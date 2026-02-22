use sqlx::{Executor, PgPool};
use type_process_builder::builder::{FinalizedProcess, RunnableProcess};

pub async fn create_table<Process: FinalizedProcess>(
  pool: &PgPool,
  process: RunnableProcess<Process>,
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
