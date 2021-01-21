use tokio::net::UnixListener;
use std::fs;
use tokio::io::copy;

#[tokio::main]
async fn main() {
    let path = "/tmp/test.sock";
    fs::remove_file(path).unwrap();
    let listener = UnixListener::bind(path).unwrap();
    println!("Listen on {}", path);
    loop {
        match listener.accept().await {
            Ok((mut stream, _addr)) => {
                println!("new client!");
                tokio::spawn(async move {
                    let (mut reader, mut writer) = stream.split();
                    copy(&mut reader, &mut writer).await.unwrap();
                    println!("Client left!");
                });
            }
            Err(e) => { println!("Err: {}", e) }
        }
    }
    
}