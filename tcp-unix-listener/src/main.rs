use tokio::net::{UnixListener, TcpListener};
use std::fs;
use tokio::io::copy;

#[tokio::main]
async fn main() {
    let path = "/tmp/test.sock";
    fs::remove_file(path).unwrap_or_else(|err| {
        println!("No file to delete: {}", err);
    });
    let unixlistener = UnixListener::bind(path).unwrap();
    println!("Listen on {}", path);

    let tcplistener = TcpListener::bind("127.0.0.1:4444").await.unwrap();
    println!("Listen on port: 4444");

    let u = tokio::spawn(async move {
        listen_unix(unixlistener).await;    
    });

    let t = tokio::spawn(async move {
        listen_tcp(tcplistener).await;
    });

    u.await.unwrap();
    t.await.unwrap();
}

async fn listen_unix(listener: UnixListener) {
    loop {
        match listener.accept().await {
            Ok((mut stream, _addr)) => {
                println!("new Unix client!");
                tokio::spawn(async move {
                    let (mut reader, mut writer) = stream.split();
                    copy(&mut reader, &mut writer).await.unwrap();
                    println!("Unix Client left!");
                });
            }
            Err(e) => { println!("Err: {}", e) }
        }
    }
}

async fn listen_tcp(listener: TcpListener) {
    loop {
        match listener.accept().await {
            Ok((mut stream, _addr)) => {
                println!("new TCP client!");
                tokio::spawn(async move {
                    let (mut reader, mut writer) = stream.split();
                    copy(&mut reader, &mut writer).await.unwrap();
                    println!("TCP Client left!");
                });
            }
            Err(e) => { println!("Err: {}", e) }
        }
    }
}