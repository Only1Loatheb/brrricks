use async_trait::async_trait;
use process::brick_domain::{FinalBrickHandler, InputParams, LinearBrickHandler, LinearOutput, Message, ParamId, SplitterBrickHandler, SplitterOutput};
use crate::TemplateApp;

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Brick {
  LinearBrick {
    consumes: Vec<ParamId>,
    produces: Vec<ParamId>,
    handler: Box<Linear>,
  },
  SplitterBrick {
    consumes: Vec<ParamId>,
    produces: Vec<Vec<ParamId>>,
    handler: Box<Splitter>,
  },
  FinalBrick {
    consumes: Vec<ParamId>,
    handler: Box<Final>,
  },
}


impl Default for Brick {
    fn default() -> Self {
        Brick::LinearBrick {
          consumes: vec![],
          produces: vec![],
          handler: Box::new(Linear),
        }
    }
}


#[derive(serde::Deserialize, serde::Serialize)]
struct Linear;

#[async_trait]
impl LinearBrickHandler for Linear {
  async fn handle(&self, input: InputParams) -> anyhow::Result<LinearOutput> {
    todo!()
  }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Splitter;

#[async_trait]
impl SplitterBrickHandler for Splitter {
  async fn handle(&self, input: InputParams) -> anyhow::Result<SplitterOutput> {
    todo!()
  }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Final;

#[async_trait]
impl FinalBrickHandler for Final {
  async fn handle(&self, input: InputParams) -> anyhow::Result<Message> {
    todo!()
  }
}
