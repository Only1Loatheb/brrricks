use typenum::Unsigned;

use crate::brick::*;

// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

pub(crate) const fn params(value: u128) -> Vec<ParamId> {
    let mut params: Vec<ParamId> = vec![];
    let mut i: usize = 0;
    while i < 128 {
        if value >> i & 1 {
            params.psuh(ParamId(i));
        }
        i += 1;
    }
    params
}

pub(crate) struct InternalLinearBrick {
    pub name: &'static str,
    pub consumes: Vec<ParamId>,
    pub produces: Vec<ParamId>,
    pub handler: Box<dyn LinearBrickHandler>,
}

impl InternalLinearBrick {
    pub(crate) fn new<
        CONSUMES: Unsigned,
        REQUIRES: Unsigned,
        FORBIDS: Unsigned,
        PRODUCES: Unsigned,
        ACCOMPLISHES: Unsigned,
    >(
        brick: LinearBrick<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES>,
    ) -> InternalLinearBrick {
        InternalLinearBrick {
            name: brick.name,
            consumes: params(CONSUMES::U128),
            produces: params(PRODUCES::U128),
            handler: brick.handler,
        }
    }
}

// consider https://github.com/rust-phf/rust-phf for SplitIndex
pub(crate) struct InternalSplitterBrick {
    pub name: &'static str,
    pub consumes: Vec<ParamId>,
    // pub produces: Vec<Vec<ParamId>>,
    pub handler: Box<dyn SplitterBrickHandler>,
}

impl InternalSplitterBrick {
    pub(crate) fn new<
    SPLITS: Unsigned,
    CONSUMES: Unsigned,
    REQUIRES: Unsigned,
    FORBIDS: Unsigned,
>(brick: SplitterBrick<SPLITS, CONSUMES, REQUIRES, FORBIDS>) -> InternalSplitterBrick {
        InternalSplitterBrick {
            name: brick.name,
            consumes: params(CONSUMES::U128),
            // produces: brick
            //     .produces_and_accomplishes
            //     .into_iter()
            //     .map(|(_, params)| params)
            //     .collect(),
            handler: brick.handler,
        }
    }
}

pub(crate) struct InternalFinalBrick {
    pub name: &'static str,
    pub consumes: Vec<ParamId>,
    pub handler: Box<dyn FinalBrickHandler>,
}

impl InternalFinalBrick {
    pub(crate) fn new<
        CONSUMES: Unsigned,
        REQUIRES: Unsigned,
        FORBIDS: Unsigned,
        ACCOMPLISHES: Unsigned,
    >(
        brick: FinalBrick<CONSUMES, REQUIRES, FORBIDS, ACCOMPLISHES>,
    ) -> InternalFinalBrick {
        InternalFinalBrick {
            name: brick.name,
            consumes: params(CONSUMES::U128),
            handler: brick.handler,
        }
    }
}
