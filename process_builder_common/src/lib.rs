pub mod process_domain {
  #[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
  pub struct ParamId(pub usize);

  // #[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
  // pub struct ActionId(pub usize);

  #[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
  pub struct Message(pub String);
  //
  // #[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
  // pub struct SplitIndex(pub usize);
}
// It is forbidden to overwrite param value
// cargo doc --no-deps --package process_builder_common
