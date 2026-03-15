use std::env;

pub fn main() {
  let current_dir = env::current_dir().expect("failed to get current dir");
  let project_dir = current_dir.parent().expect("We are in qrios_api_process_runner").to_str().expect("non-utf8 path");
  let swagger_path = format!("{project_dir}/qrios-ussd-api-swagger.json");
  println!("cargo:rerun-if-changed={swagger_path}");
  let file = std::fs::File::open(swagger_path).unwrap();
  let spec = serde_json::from_reader(file).unwrap();
  let mut generator = progenitor::Generator::default();

  let tokens = generator.generate_tokens(&spec).unwrap();
  let ast = syn::parse2(tokens).unwrap();
  let content = prettyplease::unparse(&ast);

  let out_file = std::path::Path::new("./src/lib.rs").to_path_buf();

  std::fs::write(out_file, content).unwrap();
}
