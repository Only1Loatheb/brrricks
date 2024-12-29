pub mod type_process_adapter {
    use async_trait::async_trait;
    use process_builder_common::process_domain::{
        InputParams, SplitIndex, SplitterBrickHandler, SplitterOutput,
    };
    use process_builder_common::internal_brick::InternalSplitterBrick;
    use type_process_builder::step::{
        ParamReprList, SplitterBrick, SplitterOutputRepr, TypeSplitterBrickHandler,
    };

    /// todo
    impl<
            USES: ParamBitSet,
            REQUIRES: Unsigned,
            FORBIDS: Unsigned,
            PRODUCES_AND_ACCOMPLISHES: CaseArray + Len,
        > SplitterBrick<USES, REQUIRES, FORBIDS, PRODUCES_AND_ACCOMPLISHES>
    where
        Length<PRODUCES_AND_ACCOMPLISHES>: Cmp<U1>,
        Length<PRODUCES_AND_ACCOMPLISHES>:
            IsGreaterPrivate<U1, <Length<PRODUCES_AND_ACCOMPLISHES> as Cmp<U1>>::Output>,
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
