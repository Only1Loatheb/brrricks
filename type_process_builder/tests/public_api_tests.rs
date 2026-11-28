use std::path::PathBuf;

#[test]
fn test_exposed_crates_in_public_api() {
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

  let mut exposed_root_crates: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
  for line in api_str.lines() {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
      let is_ident_start = bytes[i].is_ascii_alphabetic() || bytes[i] == b'_';
      if is_ident_start
        && (i == 0 || (!bytes[i - 1].is_ascii_alphanumeric() && bytes[i - 1] != b'_' && bytes[i - 1] != b':'))
      {
        let start = i;
        while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
          i += 1;
        }
        if bytes[i..].starts_with(b"::") {
          if let Some(root) = line.get(start..i)
            && root.chars().next().is_some_and(|c| c.is_ascii_lowercase())
            && !matches!(root, "self" | "super" | "crate")
          {
            exposed_root_crates.insert(root);
          }
          // Skip the rest of this `a::b::c` path so submodules aren't counted as root crates.
          while bytes[i..].starts_with(b"::") {
            i += 2;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
              i += 1;
            }
          }
          continue;
        }
      }
      i += 1;
    }
  }

  let expected_crates: std::collections::BTreeSet<&str> =
    ["alloc", "core", "serde_core", "type_process_builder", "typenum"].into_iter().collect();
  assert_eq!(
    exposed_root_crates, expected_crates,
    "unexpected crates exposed in type_process_builder public API:\n{api_str}"
  );
}
