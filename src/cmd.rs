use anyhow::{anyhow, bail, Context};
use crate::resp::{Msg, Msg::*};

#[derive(Debug)]
pub enum Cmd<'a> {
    Ping(Option<&'a str>),
    Echo(&'a str),
}

use Cmd::*;

impl<'a> Cmd<'a> {
    pub fn decode(msg: &Msg) -> anyhow::Result<Cmd> {
        let (cmd, args) = msg
            .as_array()?
            .split_first().ok_or(anyhow!("empty command list"))?;
        let cmd = cmd.as_bulk_string()?;
        match cmd.to_uppercase().as_str() {
            "PING" => {
                if args.len() > 1 {
                    bail!("expected 0 or 1 args to PING, got {}", args.len())
                }
                match args.get(0) {
                    None => Ok(Ping(None)),
                    Some(arg) => arg
                        .as_bulk_string()
                        .context("first argument to PING")
                        .map(|s| Ping(Some(s)))
                }
            }

            "ECHO" => {
                if args.len() != 1 {
                    bail!("expected 1 arg to ECHO, got {}", args.len())
                }
                args.first()
                    .unwrap()
                    .as_bulk_string()
                    .context("first argument to ECHO")
                    .map(Echo)
            }

            _ => bail!("unknown command {}", cmd)
        }
    }

    pub fn respond(&self) -> Msg {
        match self {
            Ping(None) => SimpleString("PONG".to_string()),
            Ping(Some(s)) => BulkString(s.to_string()),

            Echo(msg) => BulkString(msg.to_string()),
        }
    }
}
