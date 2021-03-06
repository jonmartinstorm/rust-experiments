use tokio::net::{UnixListener};
use std::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

use rand::prelude::*;
use rand::rngs::StdRng;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    len: i32,
    msg_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    msg_type: String, 
    address: i32,
    value: u16,
}


#[tokio::main]
async fn main() {
    let path = "/tmp/test.sock";
    fs::remove_file(path).unwrap_or_else(|err| {
        println!("No file to delete: {}", err);
    });
    let unixlistener = UnixListener::bind(path).unwrap();
    println!("Listen on {}", path);
    
    let t = tokio::spawn(async move {
        listen_tcp(unixlistener).await;
    });

    t.await.unwrap();
}

async fn listen_tcp(listener: UnixListener) {
    let mut r = StdRng::seed_from_u64(32);
    loop {
        match listener.accept().await {
            Ok((mut stream, _addr)) => {
                println!("new TCP client! {:?}", _addr);
                let value: u16 = r.gen();
                tokio::spawn(async move {
                    let (mut reader, mut writer) = stream.split();
                    // read header length
                    let mut len = vec![0; 1];
                    reader.read(&mut len).await.unwrap();
                    println!("Header len {}", &len[0]);

                    // read header
                    let mut header = vec![0; len[0] as usize];
                    reader.read(&mut header).await.unwrap();
                    let header_string = std::str::from_utf8(&header).unwrap();
                    println!("Header string {}", header_string);
                    let header: Header = serde_json::from_str(header_string).unwrap();
                    println!("Header {:?}", header);

                    // read payload
                    let mut payload = vec![0; header.len as usize];
                    reader.read(&mut payload).await.unwrap();
                    let payload_string = std::str::from_utf8(&payload).unwrap();
                    println!("Payload string {}", payload_string);
                    let payload: Point = serde_json::from_str(payload_string).unwrap();
                    println!("Payload {:?}", payload);

                    // write something random

                    let hardcoded = Message {
                        msg_type: String::from("input-register"),
                        address: 0,
                        value,
                    };
                    let mut hardcoded = serde_json::to_string(&hardcoded).unwrap();
                    hardcoded.push('\n');
                    writer.write_all(hardcoded.as_bytes()).await.unwrap();
                    println!("TCP Client left!");
                });
            }
            Err(e) => { println!("Err: {}", e) }
        }
    }
}