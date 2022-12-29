use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5555").expect("bind failed");
    
    while let Some(stream) = listener.incoming().next() {
        if stream.is_err() {
            continue;
        }

        let stream = stream.unwrap();
        let peer = stream.peer_addr();
        println!("connected: {:?}", peer);
        process_stream(stream);
        println!("disconnected: {:?}", peer);
    }
}

fn process_stream(mut stream: TcpStream) {
    let mut buf = [0u8; 4];
    loop {
        if stream.read_exact(&mut buf).is_err() {
            break;
        }

        let reply = u32::from_be_bytes(buf) ^ 0xFFFFFFFF;
        if stream.write_all(&reply.to_be_bytes()).is_err() {
            break;
        }
    }
}