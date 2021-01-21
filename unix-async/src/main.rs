use tokio::net::UnixListener;

#[tokio::main]
async fn main() {
    let listener = UnixListener::bind("/path/to/the/socket").unwrap();
    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                println!("new client!");
            }
            Err(e) => { /* connection failed */ }
        }
    }
}