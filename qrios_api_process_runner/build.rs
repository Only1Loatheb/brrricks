use std::env;
use std::process::Command;

const SWAGGER_FILE_NAME: &str = "qrios-ussd-api-swagger.json";
pub fn main() {
  let current_dir = env::current_dir().expect("failed to get current dir");
  let project_dir = current_dir.parent().expect("We are in qrios_api_process_runner").to_str().expect("non-utf8 path");
  let swagger_path = format!("{project_dir}/{SWAGGER_FILE_NAME}");
  println!("cargo:rerun-if-changed={swagger_path}");
  let uid = String::from_utf8(Command::new("id").arg("-u").output().unwrap().stdout).unwrap().trim().to_string();
  let gid = String::from_utf8(Command::new("id").arg("-g").output().unwrap().stdout).unwrap().trim().to_string();
  let status = Command::new("docker")
    .args([
      "run",
      "--rm",
      "--user",
      &format!("{uid}:{gid}"),
      "-v",
      &format!("{project_dir}:/local"),
      "openapitools/openapi-generator-cli:v7.20.0",
      "generate",
      "-i",
      &format!("/local/{SWAGGER_FILE_NAME}"),
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
