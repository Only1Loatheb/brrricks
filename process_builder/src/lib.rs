use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

mod brick;
mod builder;
mod split_index;

use brick::brick::{
    FinalBrick, LinearBrick, LinearBrickData, Named, Param, SplitParam, SplitterBrick,
    SplitterBrickData,
};
use builder;
use split_index::SplitIndex;

pub mod process_builder {
    #[derive(Serialize, Deserialize)]
    struct AParam;

    impl Named for AParam {
        fn name() -> &'static str {
            "AParam"
        }
    }

    #[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
    pub enum SplitP {
        Bar,
        Foo,
    }

    impl Param for SplitP {
        fn name() -> &'static str {
            "SplitP"
        }
    }

    impl SplitParam for SplitP {
        fn split_index(&self) -> SplitIndex {
            match self {
                SplitP::Bar => SplitIndex { value: 0 },
                SplitP::Foo => SplitIndex { value: 1 },
            }
        }
    }

    struct Linear;

    impl LinearBrick for Linear {
        fn data(&self) -> LinearBrickData {
            LinearBrickData {
                name: "Linear",
                consumes: arr![],
                // not_produced_before: arr![],
                produces: arr![],
            }
        }
    }

    struct Splitter;

    impl SplitterBrick<MaxSplitIndex = U0> for Splitter {
        fn data(&self) -> SplitterBrickData {
            SplitterBrickData {
                name: "Splitter",
                consumes: arr![],
                // not_produced_before: arr![],
                produces,
            }
        }
    }

    struct Final;

    impl FinalBrick for Final {
        fn data(&self) -> BrickData {
            BrickData {
                name: "Final",
                consumes: arr![],
                not_produced_before: arr![],
            }
        }
    }

    use std::collections::HashMap;

    // pub const fn
    pub fn get_simple_process() -> NamedProcess {
        process(&Linear)
            .and_then(&Linear)
            .split(
                &Splitter,
                HashMap::from([
                    (SplitP::Bar, empty_process()),
                    (SplitP::Foo, process(&Linear)),
                ]),
            )
            .split_finalized(
                &Splitter,
                HashMap::from([
                    (SplitP::Bar, finnish(&Final)),
                    (SplitP::Foo, process(&Linear).finnish(&Final)),
                ]),
            )
            .close("aa")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Linear.data().base.name, "Linear".to_string());
    }
}
