use mermaid_rs_renderer::{render, RenderConfig, Theme};
use std::fs;
use std::path::Path;
pub fn main() {
  monk::init();

  let filename = "type_process_builder/src/process_builder_diagram.mmd";
  println!("cargo:rerun-if-changed={filename}");
  let contents = fs::read_to_string(Path::new(filename)).expect("failed to read diagram file");
  let diagram = render(&*contents).expect("failed to render diagram file");
  mermaid_rs_renderer::write_output_png(
    &*diagram,
    Path::new("process_builder_diagram.png"),
    &RenderConfig::default(),
    &Theme::mermaid_default(),
  )
  .expect("failed to write diagram file");
}
