use serde::{Serialize, Deserialize};

mod brick {


  trait Brick<ERROR> {
    fn name(&self) -> str;
    fn consumes(&self) -> Vec<T, T: Serialize + Deserialize>;
  }
}