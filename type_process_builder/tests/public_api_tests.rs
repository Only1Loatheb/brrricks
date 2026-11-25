use std::path::PathBuf;

#[test]
fn test_anyhow_is_hidden_from_public_api() {
  let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let workspace_root = manifest_dir.parent().unwrap();

  let json_path = rustdoc_json::Builder::default()
    .toolchain("nightly")
    .manifest_path(manifest_dir.join("Cargo.toml"))
    .all_features(true)
    .target_dir(workspace_root.join("target/rustdoc_api_check"))
    .build()
    .expect("failed to build rustdoc JSON");

  let public_api = public_api::Builder::from_rustdoc_json(&json_path).build().expect("failed to parse public API");
  let api_str = public_api.to_string();

  assert!(!api_str.contains("anyhow"), "anyhow leaked into type_process_builder public API:\n{api_str}");
}
