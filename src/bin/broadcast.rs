use maelstrom::{Body, Message, Response};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    io::{stdin, stdout, StdoutLock, Write},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: HashSet<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

struct BroadcastNode {
    msg_id: usize,
    messages: HashSet<usize>,
}

impl Response<Payload> for BroadcastNode {
    fn respond(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Init { .. } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: Some(self.msg_id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::InitOk {},
                    },
                };
                serde_json::to_writer(&mut *output, &reply)?;
                output.write_all(b"\n")?;
            }
            Payload::InitOk { .. } => {
                unreachable!("InitOk occurred: shouldn't happen")
            }
            Payload::Broadcast { message } => {
                // println!("broadcast!");
                self.messages.insert(message);
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: Some(self.msg_id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::BroadcastOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply)?;
                output.write_all(b"\n")?;
            }
            Payload::BroadcastOk => unreachable!("BroadcastOk occurred: shouldn't happen"),
            Payload::Read => {
                // println!("read!");
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: Some(self.msg_id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::ReadOk {
                            messages: self.messages.clone(),
                        },
                    },
                };
                serde_json::to_writer(&mut *output, &reply)?;
                output.write_all(b"\n")?;
            }
            Payload::ReadOk { .. } => unreachable!("ReadOk occurred: shouldn't happen"),
            Payload::Topology { .. } => {
                // println!("topology!");
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: Some(self.msg_id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::TopologyOk {},
                    },
                };
                serde_json::to_writer(&mut *output, &reply)?;
                output.write_all(b"\n")?;
            }
            Payload::TopologyOk => unreachable!("TopologyOk occurred: shouldn't happen"),
        };
        self.msg_id += 1;

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = stdin().lock();
    let mut stdout = stdout().lock();
    let mut node = BroadcastNode {
        msg_id: 1,
        messages: HashSet::new(),
    };

    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter();
    for input in inputs {
        match input {
            Ok(msg) => {
                node.respond(msg, &mut stdout)?;
            }
            Err(err) => {
                println!("{:?}", err);
            }
        };
    }

    Ok(())
}
