use mermaid_rs_renderer::render;
use std::fs;
use std::path::Path;
pub fn main() {
  monk::init();

  let filename = "type_process_builder/src/process_builder_diagram.mmd";
  println!("cargo:rerun-if-changed={filename}");
  let contents = fs::read_to_string(Path::new(filename)).expect("failed to read diagram file");
  let diagram = render(&*contents).expect("failed to render diagram file");
  fs::write(Path::new("process_builder_diagram.svg"), diagram).expect("failed to write diagram file");
}
