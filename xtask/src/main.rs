pub mod generate_stubs;

use crate::generate_stubs::{generate_qrios_api_axum_server, generate_qrios_api_reqwest_client};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
  let root = env::current_dir().expect("failed to get current dir");
  update_readme(&root);
  generate_client_and_server_from_swagger(&root);
  install_git_hooks();
}

fn generate_client_and_server_from_swagger(root: &Path) {
  let swagger_path = root.join("qrios-ussd-api-swagger.json");
  assert!(swagger_path.exists(), "Swagger file not found: {}", swagger_path.display());
  let generate_stubs_path = root.join("xtask/src/generate_stubs.rs");
  assert!(generate_stubs_path.exists(), "generate_stubs.rs file not found: {}", generate_stubs_path.display());
  let current_hash = {
    let mut hasher = Sha256::default();
    hasher.update(fs::read(&swagger_path).expect("failed to read qrios-ussd-api-swagger.json"));
    hasher.update(fs::read(&generate_stubs_path).expect("failed to read generate_stubs.rs"));
    hasher.finalize()
  };

  let hash_path = root.join(".qrios-ussd-api-swagger.hash");
  if fs::read(hash_path.clone()).ok().as_deref() == Some(current_hash.as_slice()) {
    println!("Swagger unchanged, skipping generation");
    return;
  }
  generate_qrios_api_axum_server(root);
  generate_qrios_api_reqwest_client(root);
  fs::write(hash_path, current_hash).expect("failed to write hash");
}

fn update_readme(root: &Path) {
  update_diagram_in_readme(
    &root.join("type_process_builder/doc/brrricks_app_session_flow.mmd"),
    &root.join("README.md"),
    "## Typical USSD service interaction flow",
  );
  update_diagram_in_readme(
    &root.join("type_process_builder/doc/process_builder_states.mmd"),
    &root.join("README.md"),
    "## Process builder states",
  );
  update_example_in_readme(&root.join("README.md"), &root.join("src/lib.rs"));
}

fn update_diagram_in_readme(diagram_path: &Path, readme_path: &Path, section_header: &str) {
  let mmd = fs::read_to_string(diagram_path).expect("Failed to read mmd");
  let readme = fs::read_to_string(readme_path).expect("Failed to read README.md");

  let header_start = readme.find(section_header).expect("section header not found");
  let block_marker = "```mermaid\n";
  let block_start =
    readme[header_start..].find(block_marker).map(|i| header_start + i).expect("mermaid block start not found");
  let content_start = block_start + block_marker.len();
  let block_end = readme[content_start..].find("```").map(|i| content_start + i).expect("mermaid block end not found");

  let diagram_content = if mmd.ends_with('\n') { mmd } else { format!("{mmd}\n") };

  let updated_readme = format!("{}{}{}", &readme[..content_start], diagram_content, &readme[block_end..]);
  fs::write(readme_path, updated_readme).expect("failed to write README.md");
}

fn update_example_in_readme(readme_path: &Path, example_path: &Path) {
  let readme = fs::read_to_string(readme_path).expect("Failed to read README.md");
  let example = fs::read_to_string(example_path).expect("Failed to read example");

  let generated_section = format!("```rust\n{example}\n```");

  let start_marker = "<!-- EXAMPLE_START -->";
  let end_marker = "<!-- EXAMPLE_END -->";

  let start = readme.find(start_marker).expect("Missing EXAMPLE_START");
  let end = readme.find(end_marker).expect("Missing EXAMPLE_END");

  let new_readme = format!("{}\n\n{}\n\n{}", &readme[..start + start_marker.len()], generated_section, &readme[end..]);

  fs::write(readme_path, new_readme).expect("Failed to write README.md");
}

fn install_git_hooks() {
  if Command::new("monk").arg("--version").output().is_err() {
    let monk_installation =
      Command::new("cargo").args(["install", "monk"]).status().expect("Failed to run cargo install monk");
    assert!(monk_installation.success(), "Failed to cargo install monk");
  }

  let hook_installation = Command::new("monk").args(["install"]).status().expect("Failed to run monk install");
  assert!(hook_installation.success(), "Failed to install git hooks");
}
