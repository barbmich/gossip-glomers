use serde::{Deserialize, Serialize};
use std::io::StdoutLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<Payload> {
    pub src: String,
    pub dest: String,
    pub body: Body<Payload>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body<Payload> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

pub trait Response<Payload> {
    fn respond(&mut self, inputs: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()>;
}
