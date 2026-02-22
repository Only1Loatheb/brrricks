use sqlx::{Executor, PgPool};
use type_process_builder::builder::{FinalizedProcess, RunnableProcess};

pub async fn create_table<Process: FinalizedProcess>(
  pool: &PgPool,
  process: RunnableProcess<Process>,
) -> Result<(), sqlx::Error> {
  let mut columns = String::new();

  for col in process.all_columns() {
    let name: u32 = col;
    columns.push_str(&format!("\"{}\" jsonb NULL,", name));
  }

  // remove trailing comma
  columns.pop();

  let sql = format!(
    r#"
        CREATE TABLE IF NOT EXISTS my_table (
            id BIGINT PRIMARY KEY,
            {}
        )
        "#,
    columns
  );

  pool.execute(sql.as_str()).await?;

  Ok(())
}
