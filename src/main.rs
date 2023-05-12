use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
mod resp;
use anyhow::{anyhow, bail, Context};
use resp::{Msg, Msg::*};

const DEFAULT_ADDR: &str = "127.0.0.1:6379";

fn handle_msg(msg: &Msg) -> anyhow::Result<Msg> {
    let (cmd, args) = msg
        .as_array()?
        .split_first().ok_or(anyhow!("empty command list"))?;
    let cmd = cmd.as_bulk_string()?;

    match cmd {
        "ping" => {
            if args.len() > 1 {
                bail!("expected 0 or 1 args to PING, got {}", args.len())
            }
            match args.get(0) {
                None => Ok(SimpleString("PONG".to_string())),
                Some(arg) => arg
                    .as_bulk_string()
                    .context("first argument to PING")
                    .map(|s| BulkString(s.to_string()))
            }
        }

        _ => bail!("unknown command {}", cmd)
    }
}

fn handle_conn(conn: TcpStream) {
    let peer = conn
        .peer_addr()
        .map_or(String::from("<unknown>"), |addr| { addr.to_string() });

    println!("[*] {peer} connected");

    let conn_reader = conn;
    let mut conn_writer = conn_reader.try_clone().expect("cloning tcp stream for writing");

    for msg in Msg::decoder(conn_reader
                            .bytes()
                            // .inspect(|x| {if let Ok(c) = *x { println!("CHAR: {:?}", c as char) }})
                            .filter_map(|i| i.ok()))
    {
        match msg {
            Err(err) => {
                println!("[*] Invalid msg from {peer} --> {err}")
            }

            Ok(msg) => {
                println!("[*] {peer} --> {msg:?}");

                let response = match handle_msg(&msg) {
                    Ok(m) => m,

                    Err(e) => {
                        println!("[*] Invalid cmd {msg:?}: {e}");
                        Null
                    }
                };

                let write_ok = conn_writer.write_all(& response.encode()).is_ok();
                println!(
                    "[{}] {peer} <-- {response:?}{}",
                    if write_ok {"*"} else {"!"},
                    if write_ok {""} else {" (FAILED)"}
                );
                if ! write_ok {
                    break;
                }
            }
        }
    }

    println!("[*] {peer} closed");
}

fn main() {
    let listener = TcpListener
        ::bind(DEFAULT_ADDR)
        .expect("creating tcp listener");

    println!("[*] started listening on {DEFAULT_ADDR}");

    for conn in listener.incoming() {
        match conn {
            Ok(conn) => {
                std::thread::spawn(|| handle_conn(conn));
            }
            Err(e) => {
                eprintln!("[!] Error in incoming connection: {e}");
            }
        }
    }
}
