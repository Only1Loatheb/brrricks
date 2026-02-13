use std::fs;
use std::path::Path;

pub fn main() {
  monk::init();

  update_diagram_in_readme(
    Path::new("type_process_builder/doc/brrricks_app_session_flow.mmd"),
    "## Brrricks app session flow",
  );
  update_diagram_in_readme(
    Path::new("type_process_builder/doc/process_builder_states.mmd"),
    "## Process builder states",
  );
}

fn update_diagram_in_readme(diagram_path: &Path, section_header: &str) {
  let readme_path = Path::new("README.md");
  // Re-run build if inputs change
  println!("cargo:rerun-if-changed={}", readme_path.display());
  println!("cargo:rerun-if-changed={}", diagram_path.display());

  let readme = fs::read_to_string(readme_path).expect("Failed to read README.md");
  let diagram = fs::read_to_string(diagram_path).expect("Failed to read brrricks_app_session_flow.mmd");

  let start = readme
    .find(section_header)
    .unwrap_or_else(|| panic!("`{section_header}` section not found in {readme_path:?}"));

  // Position just after the section header line
  let content_start = readme[start..]
    .find('\n')
    .map(|i| start + i + 1)
    .expect("Malformed section header");

  // End at next `## ` header or EOF
  let content_end = readme[content_start..]
    .find("\n## ")
    .map(|i| content_start + i)
    .unwrap_or_else(|| readme.len());

  let mut new_readme = String::with_capacity(readme.len() + diagram.len() + 32);

  // Before section content
  new_readme.push_str(&readme[..content_start]);

  // Inject mermaid block
  new_readme.push_str("\n```mermaid\n");
  new_readme.push_str(&diagram);
  if !diagram.ends_with('\n') {
    new_readme.push('\n');
  }
  new_readme.push_str("```\n");

  // After section
  new_readme.push_str(&readme[content_end..]);

  fs::write(readme_path, new_readme).expect("Failed to write updated README.md");
}
