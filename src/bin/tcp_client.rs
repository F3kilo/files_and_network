use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:5555").expect("connection failed");

    let mut buf = [0u8; 4];
    for i in 0..10u32 {
        stream.write_all(&i.to_be_bytes()).expect("fail to request");
        stream.read_exact(&mut buf).expect("fail to get reply");
        println!("reply: {}", u32::from_be_bytes(buf));
        thread::sleep(Duration::from_secs_f32(0.5));
    }
}