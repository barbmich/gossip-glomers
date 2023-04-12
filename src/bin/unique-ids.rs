use maelstrom::{Body, Message, Response};
use serde::{Deserialize, Serialize};
use std::io::{stdin, stdout, StdoutLock, Write};
use ulid::Ulid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {},
    Generate {},
    GenerateOk {
        id: String,
    },
}

struct GenerateNode {
    msg_id: usize,
}

impl Response<Payload> for GenerateNode {
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
            Payload::Generate {} => {
                let ulid = Ulid::new();
                let id = format!("{}-{}", input.src, ulid);
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: Some(self.msg_id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::GenerateOk { id },
                    },
                };

                serde_json::to_writer(&mut *output, &reply)?;
                output.write_all(b"\n")?;
            }
            Payload::GenerateOk { .. } => {}
        };

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = stdin().lock();
    let mut stdout = stdout().lock();
    let mut node = GenerateNode { msg_id: 1 };

    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter();
    for input in inputs {
        let input = input?;
        node.respond(input, &mut stdout)?;
    }

    Ok(())
}
