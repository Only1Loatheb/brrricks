#[cfg_attr(doc, aquamarine::aquamarine)]
/// It is forbidden to overwrite param value
/// cargo doc --no-deps --workspace
/// ```mermaid
/// stateDiagram-v2
///     [*] --> FinalizedProcess: FinalBrick
///     FinalizedProcess --> [*]
///     [*] --> FlowingProcess: LinearBrick
///     FlowingProcess --> FlowingProcess: LinearBrick\n or FlowingProcess
///     FlowingProcess --> FinalizedProcess: FinalBrick\n or FinalizedProcess
///     [*] --> FinalizedSplitProcess: SplitBrick
///     FlowingProcess --> FinalizedSplitProcess: SplitBrick
///     state finalized_split_cases_final <<choice>>
///     FinalizedSplitProcess --> finalized_split_cases_final: FinalizedProcess
///     finalized_split_cases_final --> FinalizedSplitProcess: some cases left
///     finalized_split_cases_final --> FinalizedProcess: all cases handled
///     state finalized_split_cases_linear <<choice>>
///     FinalizedSplitProcess --> finalized_split_cases_linear: FlowingProcess
///     finalized_split_cases_linear --> FlowingSplitProcess: some cases left
///     finalized_split_cases_linear --> FlowingProcess: all cases handled
///     state flowing_split_cases <<choice>>
///     FlowingSplitProcess --> flowing_split_cases: FinalizedProcess\n or FlowingProcess
///     flowing_split_cases --> FlowingSplitProcess: some cases left
///     flowing_split_cases --> FlowingProcess: all cases handled
/// ```
pub mod brick_domain {
  #[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
  pub struct ParamId(pub usize);

  #[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
  pub struct ActionId(pub usize);

  #[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
  pub struct Message(pub String);

  #[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
  pub struct SplitIndex(pub usize);
}
