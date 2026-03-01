use std::env;
use std::process::Command;

pub fn main() {
  let project_dir = env::current_dir().expect("failed to get current dir");
  let project_dir = project_dir.parent().expect("We are in qrios_api_process_runner").to_str().expect("non-utf8 path");
  println!("cargo:rerun-if-changed={project_dir}/qrios-ussd-api-swagger.json");

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
      "openapitools/openapi-generator-cli",
      "generate",
      "-i",
      "/local/qrios-ussd-api-swagger.json",
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
