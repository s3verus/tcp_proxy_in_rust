use std::net::TcpListener;
use rust_proxy::handle_connection;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}