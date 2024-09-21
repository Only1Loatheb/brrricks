pub mod type_process_adapter {
  use async_trait::async_trait;
  use process_builder_common::brick_domain::{InputParams, SplitIndex, SplitterBrickHandler, SplitterOutput};
  use process_builder_common::internal_brick::InternalSplitterBrick;
  use type_process_builder::brick::{ParamReprList, SplitterBrick, SplitterReprCase, TypeSplitterBrickHandler};

  struct TypeSplitterBrickHandlerAdapter<'same_process, CASE_THIS: ParamReprList<'same_process>, CASE_OTHER: SplitterReprCase<'same_process>> {
    inner: Box<dyn TypeSplitterBrickHandler<'same_process, CASE_THIS, CASE_OTHER>>,
  }

  impl<'same_process, CASE_THIS: ParamReprList<'same_process>, CASE_OTHER: SplitterReprCase<'same_process>>
  TypeSplitterBrickHandlerAdapter<'same_process, CASE_THIS, CASE_OTHER> {
    fn new(inner: Box<dyn TypeSplitterBrickHandler<'same_process, CASE_THIS, CASE_OTHER>>) -> Self {
      Self { inner }
    }
  }

  #[async_trait]
  impl<'same_process, CASE_THIS: ParamReprList<'same_process>, CASE_OTHER: SplitterReprCase<'same_process>> SplitterBrickHandler
  for TypeSplitterBrickHandlerAdapter<'same_process, CASE_THIS, CASE_OTHER> {
    async fn handle(&self, input: InputParams) -> anyhow::Result<SplitterOutput> {
      let result = self.inner.handle(input).await?;
      anyhow::Ok(SplitterOutput(SplitIndex(result.0.get()), result.1))
    }
  }

  /// todo
  impl<
  USES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES_AND_ACCOMPLISHES: CaseArray + Len,
> SplitterBrick<USES, REQUIRES, FORBIDS, PRODUCES_AND_ACCOMPLISHES>
where
  Length<PRODUCES_AND_ACCOMPLISHES>: Cmp<U1>,
  Length<PRODUCES_AND_ACCOMPLISHES>: IsGreaterPrivate<U1, <Length<PRODUCES_AND_ACCOMPLISHES> as Cmp<U1>>::Output>,
  Gr<Length<PRODUCES_AND_ACCOMPLISHES>, U1>: NonZero,
{
  pub(crate) fn to_internal(self) -> InternalSplitterBrick {
    InternalSplitterBrick {
      name: self.name,
      uses: USES::get().0,
      produces: PRODUCES_AND_ACCOMPLISHES::get(),
      handler: Box::new(TypeSplitterBrickHandlerAdapter::new(self.handler)),
    }
  }
}
}
