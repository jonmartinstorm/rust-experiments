use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::prelude::*;

fn handle_client(mut stream: TcpStream) {
    println!("yes.!");
    let mut buffer = [0;10];

    stream.read(&mut buffer).unwrap();
    let mut buffer = std::str::from_utf8(&buffer).unwrap();
    let b: Vec<&str> = buffer.split("\n").collect();
    println!("Buffer 10 bit: {:?}", &b[0]);

}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9977")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
