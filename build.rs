use std::path::Path;
use std::process::Command;
use std::{env, fs};

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
  // generate_qrios_api_axum_server();
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

fn generate_qrios_api_axum_server() {
  println!("cargo:rerun-if-changed=qrios-ussd-api-swagger.json");
  let project_dir = env::current_dir().expect("failed to get current dir");
  let project_dir = project_dir.to_str().expect("non-utf8 path");
  println!("project_dir: {}", project_dir);
  let uid = String::from_utf8(Command::new("id").arg("-u").output().unwrap().stdout)
    .unwrap()
    .trim()
    .to_string();

  let gid = String::from_utf8(Command::new("id").arg("-g").output().unwrap().stdout)
    .unwrap()
    .trim()
    .to_string();
  let status = Command::new("docker")
    .args([
      "run",
      "--rm",
      "--user",
      &format!("{uid}:{gid}"),
      "-v",
      &format!("{project_dir}:/local"),
      "openapitools/openapi-generator-cli",
      "generate",
      "-i",
      "/local/qrios-ussd-api-swagger.json",
      "-g",
      "rust-axum",
      "-o",
      "/local/qrios_api_axum_server",
      "--additional-properties=packageName=qrios_api_axum_server",
    ])
    .status()
    .expect("failed to run docker");

  if !status.success() {
    panic!("openapi-generator failed");
  }
}
