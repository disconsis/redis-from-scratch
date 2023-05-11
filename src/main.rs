use std::net::TcpListener;

const DEFAULT_ADDR: &str = "127.0.0.1:6379";

fn main() {
    let listener = TcpListener
        ::bind(DEFAULT_ADDR)
        .expect("creating tcp listener");
    println!("[*] started listening on {}", DEFAULT_ADDR);

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}
