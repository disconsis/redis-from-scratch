use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
mod resp;
use resp::Msg;

const DEFAULT_ADDR: &str = "127.0.0.1:6379";

fn handle_cmd(args: &[&str]) -> Msg {
    todo!()
}

fn handle_msg(msg: &Msg) -> Msg {
    todo!()
}

fn handle_conn(conn: TcpStream) {
    let peer = conn
        .peer_addr()
        .map_or(String::from("<unknown>"), |addr| { addr.to_string() });

    println!("[*] {peer} connected");

    let conn_reader = conn;
    let mut conn_writer = conn_reader.try_clone().expect("cloning tcp stream for writing");

    for msg in Msg::decoder(conn_reader.bytes()
                            .filter_map(|i| i.ok()))
    {
        println!("[*] {peer} --> {msg:?}");
        let response = handle_msg(&msg);
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
