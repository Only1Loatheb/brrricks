use sha2::{Sha256, Digest};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
  let root = env::current_dir().expect("failed to get current dir");
  update_readme(&root);
  generate_client_and_server_from_swagger(&root);
  install_git_hooks();
}

fn generate_client_and_server_from_swagger(root: &PathBuf) {
  let swagger_path = root.join("qrios-ussd-api-swagger.json");
  if !swagger_path.exists() {
    panic!("Swagger file not found: {}", swagger_path.display());
  }
  let file = fs::read(&swagger_path).expect("failed to read swagger");
  let current_hash = Sha256::digest(file);

  let hash_path = root.join(".qrios-ussd-api-swagger.hash");
  if fs::read(hash_path.clone()).ok().as_deref() == Some(current_hash.as_slice()) {
    println!("Swagger unchanged, skipping generation");
    return;
  }
  generate_qrios_api_axum_server(&root);
  generate_qrios_api_reqwest_server(&root);
  fs::write(hash_path, current_hash).expect("failed to write hash");
}

fn update_readme(root: &PathBuf) {
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
  let readme = fs::read_to_string(readme_path).expect("Failed to read README.md");

  let diagram = fs::read_to_string(diagram_path).expect("Failed to read diagram");

  let start = readme.find(section_header).unwrap_or_else(|| panic!("`{section_header}` not found"));

  let content_start = readme[start..].find('\n').map(|i| start + i + 1).expect("Malformed section");

  let content_end = readme[content_start..].find("\n## ").map(|i| content_start + i).unwrap_or_else(|| readme.len());

  let mut new_readme = String::with_capacity(readme.len() + diagram.len() + 32);

  new_readme.push_str(&readme[..content_start]);

  new_readme.push_str("\n```mermaid\n");
  new_readme.push_str(&diagram);
  if !diagram.ends_with('\n') {
    new_readme.push('\n');
  }
  new_readme.push_str("```\n");

  new_readme.push_str(&readme[content_end..]);

  fs::write(readme_path, new_readme).expect("Failed to write README.md");
}

fn update_example_in_readme(readme_path: &Path, example_path: &Path) {
  let readme = fs::read_to_string(readme_path).expect("Failed to read README.md");

  let example = fs::read_to_string(example_path).expect("Failed to read example");

  let generated_section = format!("```rust\n{}\n```", example);

  let start_marker = "<!-- EXAMPLE_START -->";
  let end_marker = "<!-- EXAMPLE_END -->";

  let start = readme.find(start_marker).expect("Missing EXAMPLE_START");

  let end = readme.find(end_marker).expect("Missing EXAMPLE_END");

  let new_readme = format!("{}\n\n{}\n\n{}", &readme[..start + start_marker.len()], generated_section, &readme[end..],);

  fs::write(readme_path, new_readme).expect("Failed to write README.md");
}

fn generate_qrios_api_axum_server(project_dir: &Path) {
  let swagger_file_name: &str = "qrios-ussd-api-swagger.json";
  let uid = String::from_utf8(Command::new("id").arg("-u").output().unwrap().stdout).unwrap().trim().to_string();
  let gid = String::from_utf8(Command::new("id").arg("-g").output().unwrap().stdout).unwrap().trim().to_string();
  let status = Command::new("docker")
    .args([
      "run",
      "--rm",
      "--user",
      &format!("{uid}:{gid}"),
      "-v",
      &format!("{}:/local", project_dir.display()),
      "openapitools/openapi-generator-cli:v7.20.0",
      "generate",
      "-i",
      &format!("/local/{swagger_file_name}"),
      "-g",
      "rust-axum",
      "-o",
      "/local/qrios_api_axum_server",
      "--additional-properties=packageName=qrios_api_axum_server,disableValidator=true",
    ])
    .status()
    .expect("failed to run docker");

  if !status.success() {
    panic!("openapi-generator failed");
  }
}

fn generate_qrios_api_reqwest_server(root: &Path) {
  let swagger_path = root.join("qrios-ussd-api-swagger.json");

  if !swagger_path.exists() {
    panic!("Swagger file not found: {}", swagger_path.display());
  }

  let file = fs::File::open(&swagger_path).expect("failed to open swagger");

  let spec = serde_json::from_reader(file).expect("failed to parse swagger");

  let mut generator = progenitor::Generator::default();

  let tokens = generator.generate_tokens(&spec).expect("generation failed");

  let ast = syn::parse2(tokens).expect("failed to parse tokens");

  let content = prettyplease::unparse(&ast);

  let out_file = root.join("qrios_api_reqwest_client/src/lib.rs");

  fs::write(&out_file, content).expect("failed to write generated client");

  println!("Client written to {}", out_file.display());
}

fn install_git_hooks() {
  let monk_installation =
    Command::new("cargo").args(["install", "monk"]).status().expect("Failed to run cargo install monk");
  if !monk_installation.success() {
    panic!("Failed to cargo install monk");
  }

  let hook_installation = Command::new("monk").arg("install").status().expect("Failed to run install git hooks");
  if !hook_installation.success() {
    panic!("Failed to install git hooks");
  }
}
