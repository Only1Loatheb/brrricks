use std::fs;
use std::path::Path;
use std::process::Command;

pub(crate) fn generate_qrios_api_axum_server(project_dir: &Path) {
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
      "openapitools/openapi-generator-cli:v7.25.0",
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

  assert!(status.success(), "openapi-generator failed");
}

pub(crate) fn generate_qrios_api_reqwest_client(root: &Path) {
  let swagger_path = root.join("qrios-ussd-api-swagger.json");

  assert!(swagger_path.exists(), "Swagger file not found: {}", swagger_path.display());

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
