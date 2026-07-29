use bricks::build_demo_process;
use bricks::standard_io_process_runner::standard_io_process_runner;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let process = build_demo_process();
  standard_io_process_runner(process).await
}
