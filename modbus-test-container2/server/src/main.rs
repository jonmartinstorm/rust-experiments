
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use std::env;
use tokio::time::{sleep, Duration};
use tokio::sync::watch;
use std::str;

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

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:9977".to_string());

    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    let hei = String::from("hei");

    //let (tx, mut rx) = mpsc::channel(100);
    let (txb, rxb) = watch::channel(100);

    let t = listen_tcp(listener, hei, rxb.clone());
    let r = write_something(txb);

    r.await;
    t.await;
}

async fn write_something(txb: watch::Sender<u16>) {
    let mut r = StdRng::seed_from_u64(32);
    tokio::spawn(async move {
        loop {
            let value: u16 = r.gen();
            sleep(Duration::from_millis(1000)).await;
            //println!("Sending {}", value);
            txb.send(value).unwrap();
        }
    });
}

async fn listen_tcp(listener: TcpListener, hei: String, rxb: watch::Receiver<u16>) {
    loop {
        let (mut stream, addr) = listener.accept().await.unwrap();
        println!("New connection from {:?}", addr);
        let hei = hei.clone();
        handle(stream, hei, rxb.clone()).await;
    }
}

async fn handle(mut stream: TcpStream, hei: String, rxb: watch::Receiver<u16>) {
    
    tokio::spawn(async move {
        println!("Handle new connection");

        // In a loop, read data from the socket and write the data back.
        loop {
            let v = *rxb.borrow();
            println!("{}", hei);

            let (mut reader, mut writer) = stream.split();

            // read header length
            let mut len = vec![0; 1];
            match reader.peek(&mut len).await.unwrap() {
                0 => {break},
                _ => {},
            };
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
                value: v,
            };
            let mut hardcoded = serde_json::to_string(&hardcoded).unwrap();
            println!("Sending {}", hardcoded);
            hardcoded.push('\n');
            writer.write_all(hardcoded.as_bytes()).await.unwrap();
        }
    });
}