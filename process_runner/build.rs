fn main() {
  let src = "qrios-ussd-api-swagger.json";
  println!("cargo:rerun-if-changed={}", src);
  let file = std::fs::File::open(src).unwrap();
  let spec = serde_json::from_reader(file).unwrap();
  let mut generator = progenitor::Generator::default();

  let tokens = generator.generate_tokens(&spec).unwrap();
  let ast = syn::parse2(tokens).unwrap();
  let content = prettyplease::unparse(&ast);

  let mut out_file = std::path::Path::new("./src").to_path_buf();
  out_file.push(src.replace(".json", ".rs").replace("-", "_"));

  std::fs::write(out_file, content).unwrap();
}
