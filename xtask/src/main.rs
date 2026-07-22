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
  let file = fs::read(&swagger_path).expect("failed to read swagger");
  let current_hash = Sha256::digest(file);

  let hash_path = root.join(".qrios-ussd-api-swagger.hash");
  if fs::read(hash_path.clone()).ok().as_deref() == Some(current_hash.as_slice()) {
    println!("Swagger unchanged, skipping generation");
    return;
  }
  generate_qrios_api_axum_server(root);
  generate_qrios_api_reqwest_server(root);
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
  update_example_in_readme(&root.join("README.md"), &root.join("src/main.rs"));
}

fn update_diagram_in_readme(diagram_path: &Path, readme_path: &Path, section_header: &str) {
  let mmd = fs::read_to_string(diagram_path).expect("Failed to read mmd");
  let readme = fs::read_to_string(readme_path).expect("Failed to read README.md");

  let header_start = readme.find(section_header).expect("section header not found");
  let mmd_start = readme[header_start..].find("```mermaid\n").map(|i| header_start + i + 12).expect("mermaid block start not found");
  let mmd_end = readme[mmd_start..].find("\n```").map(|i| mmd_start + i).expect("mermaid block end not found");

  let updated_readme = format!("{}{}{}", &readme[..mmd_start], mmd, &readme[mmd_end..]);
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

fn generate_qrios_api_axum_server(_root: &Path) {
  let openapi = Command::new("npx")
    .args(["@openapitools/openapi-generator-cli", "generate"])
    .args(["-g", "rust-axum"])
    .args(["-i", "qrios-ussd-api-swagger.json"])
    .args(["-o", "qrios_api_axum_server"])
    .args(["--package-name", "qrios_api_axum_server"])
    .status()
    .expect("openapi-generator failed");

  assert!(openapi.success(), "openapi-generator failed");
}

fn generate_qrios_api_reqwest_server(root: &Path) {
  let mut progenitor = Command::new("cargo");
  progenitor.args(["progenitor"]);

  let output = progenitor.arg(root.join("qrios-ussd-api-swagger.json")).output();

  if output.as_ref().is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound) {
    Command::new("cargo")
      .args(["install", "cargo-progenitor"])
      .status()
      .expect("failed to install cargo-progenitor");
  }

  let code = progenitor.output().expect("cargo progenitor failed");

  fs::write(root.join("qrios_api_reqwest_client/src/lib.rs"), code.stdout).expect("failed to write progenitor code");
}

fn install_git_hooks() {
  let monk_installation = Command::new("cargo").args(["install", "monk"]).status().expect("Failed to run cargo install monk");
  assert!(monk_installation.success(), "Failed to cargo install monk");

  let hook_installation = Command::new("monk").args(["install"]).status().expect("Failed to run monk install");
  assert!(hook_installation.success(), "Failed to install git hooks");
}
