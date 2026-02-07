use crate::builder::{
  CurrentRunYieldedAt, FlowingProcess, IntermediateRunOutcome, IntermediateRunResult, ParamList, PreviousRunYieldedAt,
};
use crate::hlist_concat::Concat;
use crate::hlist_transform::CloneJust;
use crate::step::{Form, InputValidation};
use serde_value::Value;
use std::marker::PhantomData;

pub struct FormFlowingProcess<
  ProcessBefore: FlowingProcess,
  LinearStep: Form,
  ProcessBeforeProducesToCreateFormConsumesIndices,
  ProcessBeforeProducesToValidateInputConsumesIndices,
> {
  pub process_before: ProcessBefore,
  pub last_step: LinearStep,
  pub step_index: usize,
  pub phantom_data: PhantomData<(
    ProcessBeforeProducesToCreateFormConsumesIndices,
    ProcessBeforeProducesToValidateInputConsumesIndices,
  )>,
}

impl<
  ProcessBefore: FlowingProcess,
  CreateFormConsumes: ParamList,
  ValidateInputConsumes: ParamList,
  LastStepProduces: ParamList + Concat<ProcessBefore::Produces>,
  FormStep: Form<
      CreateFormConsumes = CreateFormConsumes,
      ValidateInputConsumes = ValidateInputConsumes,
      Produces = LastStepProduces,
    >,
  ProcessBeforeProducesToCreateFormConsumesIndices,
  ProcessBeforeProducesToValidateInputConsumesIndices,
> FlowingProcess
  for FormFlowingProcess<
    ProcessBefore,
    FormStep,
    ProcessBeforeProducesToCreateFormConsumesIndices,
    ProcessBeforeProducesToValidateInputConsumesIndices,
  >
where
  for<'a> &'a <ProcessBefore as FlowingProcess>::Produces:
    CloneJust<CreateFormConsumes, ProcessBeforeProducesToCreateFormConsumesIndices>,
  for<'a> &'a <ProcessBefore as FlowingProcess>::Produces:
    CloneJust<ValidateInputConsumes, ProcessBeforeProducesToValidateInputConsumesIndices>,
{
  type ProcessBeforeProduces = ProcessBefore::Produces;
  type Produces = <LastStepProduces as Concat<ProcessBefore::Produces>>::Concatenated;

  async fn resume_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateRunResult<Self::Produces> {
    if previous_run_yielded_at.0 < self.step_index {
      let process_before_output = self
        .process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input)
        .await?;
      match process_before_output {
        IntermediateRunOutcome::Continue(process_before_produces) => self.continue_run(process_before_produces).await,
        IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      }
    } else {
      // fixme deserialize only values required only up to the next interaction
      let process_before_produces = ProcessBefore::Produces::deserialize(previous_run_produced)?;
      let last_step_consumes = process_before_produces.clone_just();
      match self.last_step.handle_input(last_step_consumes, user_input).await? {
        InputValidation::Successful(a) => Ok(IntermediateRunOutcome::Continue(a.concat(process_before_produces))),
        InputValidation::Retry(a) => Ok(IntermediateRunOutcome::Yield(
          a,
          process_before_produces.serialize()?,
          CurrentRunYieldedAt(self.step_index),
        )),
        InputValidation::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      }
    }
  }

  async fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> IntermediateRunResult<Self::Produces> {
    let last_step_consumes = process_before_produces.clone_just();
    Ok(IntermediateRunOutcome::Yield(
      self.last_step.create_form(last_step_consumes).await?,
      process_before_produces.serialize()?,
      CurrentRunYieldedAt(self.step_index),
    ))
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.step_index = used_index + 1;
    self.step_index
  }
}
