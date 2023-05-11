use std::net::{TcpListener, TcpStream};
use std::time::Duration;

const DEFAULT_ADDR: &str = "127.0.0.1:6379";

fn handle_conn(stream: TcpStream) {
    match stream.peer_addr() {
        Err(err) => {
            eprintln!("[*] got no peer address for incoming connection!: {}", err);
        }

        Ok(peer) => {
            println!("[*] handling connection from {}", peer);
            std::thread::sleep(Duration::from_secs(100));
            println!("[*] closing connection from {}", peer);
        }
    }
}

fn main() {
    let listener = TcpListener
        ::bind(DEFAULT_ADDR)
        .expect("creating tcp listener");
    println!("[*] started listening on {}", DEFAULT_ADDR);

    for conn in listener.incoming() {
        match conn {
            Ok(stream) => {
                std::thread::spawn(|| handle_conn(stream));
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}
