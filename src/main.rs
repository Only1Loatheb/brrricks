use crate::bricks::brick::brick::{BrickKind, Param, ParamDeserializationError, ParamSerializationError};

pub mod bricks;
pub mod process;

struct A;

impl Param for A {

    fn name() -> String {
        "AParamName".to_string()
    }

    fn serialize(&self) -> Result<String, ParamSerializationError> {
        Ok("A".to_string())
    }

    fn deserialize(serialized: &str) -> Result<A, ParamDeserializationError> {
        match serialized {
            "A" => Ok(A),
            _ => Err(ParamDeserializationError),
        }
    }
}

struct B;

impl Param for B {

    fn name() -> String {
        "BParamName".to_string()
    }

    fn serialize(&self) -> Result<String, ParamSerializationError> {
        Ok("B".to_string())
    }

    fn deserialize(serialized: &str) -> Result<B, ParamDeserializationError> {
        match serialized {
            "B" => Ok(B),
            _ => Err(ParamDeserializationError),
        }
    }
}

fn main() {
    let _b: BrickKind<A> = BrickKind::LinearBrick{
        name: "Brrrick".to_string(),
        consumes: vec![Box::new(A)],
        produces: vec![Box::new(B), Box::new(A)],
        not_produced_before: vec![],
    };
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
