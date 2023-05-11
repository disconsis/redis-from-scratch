use std::net::TcpListener;

const DEFAULT_PORT: u16 = 6379;

fn main() {
    let listener = TcpListener
        ::bind(("127.0.0.1", DEFAULT_PORT))
        .expect("creating tcp listener");

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
