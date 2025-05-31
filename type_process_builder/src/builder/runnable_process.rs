use crate::builder::finalized_process::FinalizedProcess;

pub struct RunnableProcess<FINALIZED_PROCESS: FinalizedProcess> {
  finalized_process: FINALIZED_PROCESS, // shouldn't be public
}

impl<FINALIZED_PROCESS: FinalizedProcess> RunnableProcess<FINALIZED_PROCESS> {
  pub fn new(mut finalized_process: FINALIZED_PROCESS) -> Self {
    finalized_process.enumerate_steps(0);
    Self { finalized_process }
  }
}
