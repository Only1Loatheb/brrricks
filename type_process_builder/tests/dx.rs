use std::path::PathBuf;
use ui_test::dependencies::DependencyBuilder;
use ui_test::{Config, run_tests};

#[test]
fn test_dx_diagnostics() {
  let mut config = Config::rustc("tests/dx");
  config.out_dir = PathBuf::from("../target/ui");
  config.output_conflict_handling = |_, _, _, _| {};

  let mut deps = DependencyBuilder::default();
  deps.program.args.push("--all-features".into());
  config.comment_defaults.base().set_custom("dependencies", deps);

  run_tests(config).unwrap();
}
