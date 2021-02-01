
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use std::env;
use tokio::time::{sleep, Duration};
use tokio::sync::broadcast;
use std::str;

use rand::prelude::*;
use rand::rngs::StdRng;


#[tokio::main]
async fn main() {

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    let hei = String::from("hei");

    //let (tx, mut rx) = mpsc::channel(100);
    let (txb, rxb) = broadcast::channel(100);

    let t = listen_tcp(listener, hei, txb.clone());
    let r = write_something(txb.clone());

    r.await;
    t.await;
}

async fn write_something(txb: broadcast::Sender<u16>) {
    let mut r = StdRng::seed_from_u64(32);
    tokio::spawn(async move {
        loop {
            let value: u16 = r.gen();
            sleep(Duration::from_millis(1000)).await;
            println!("Number of recievers to send to: {}", txb.receiver_count());
            txb.send(value).unwrap();
        }
    });
}

async fn listen_tcp(listener: TcpListener, hei: String, txb: broadcast::Sender<u16>) {
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        println!("{}", "New connection!");
        let txb = txb.clone();
        let hei = hei.clone();
        handle(socket, hei, txb).await;
    }
}

async fn handle(mut socket: TcpStream, hei: String, txb: broadcast::Sender<u16>) {
    
    tokio::spawn(async move {
        println!("Handle new connection");
        let mut rx = txb.subscribe();

        let mut buf:Vec<u8> = vec![0; 1024];

        // In a loop, read data from the socket and write the data back.
        loop {
            let v = rx.recv().await.unwrap();
            println!("{}", hei);

            let mut n = socket
                .read(&mut buf)
                .await
                .expect("failed to read data from socket");

            if n == 0 {
                return;
            }

            println!("Got: {:?}", str::from_utf8(&buf[0..n]).unwrap());

            let r = format!(": {}\n", v);
            let r = r.as_bytes();
            //println!("{:?}", r);

            // Del opp vektoren slik at vi bare har ordet.
            buf.split_off(n);
            println!("{:?}", buf);

            // pop av newline
            buf.pop();
            n -= 1;

            // legg til verdien vår
            for i in r.iter(){
                println!("{}", i);
                buf.push(*i);
                n += 1;
            }

            // resize til 1024 igjen.
            buf.resize(1024, 0);

            println!("Sending: {:?}", str::from_utf8(&buf[0..n]).unwrap());

            socket
                .write_all(&buf[0..n])
                .await
                .expect("failed to write data to socket");
        }
    });
}