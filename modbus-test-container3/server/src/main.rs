
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use std::env;
use tokio::time::{sleep, Duration};
use tokio::sync::watch;
use tokio::sync::broadcast;
use std::str;

use serde::{Serialize, Deserialize};

use env_logger;
use log::debug;

use simulation_server_v3::watertank::WaterTank;

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
    tank_level: u16,
    tank_inflow: u16,
}



#[tokio::main]
async fn main() {
    env_logger::init();
    debug!("Log test.");
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:9977".to_string());

    let tank = WaterTank {
        level: 1000,
        areal: 1000000,
        height: 2000,
        inflow: 20.0,
        inflow_mean: 20.0,
        inflow_stddev: 3.0,
        outflow: 20.0,
        set_level: 1000,
    };

    let listener = TcpListener::bind(&addr).await.unwrap();
    debug!("Listening on: {}", addr);

    //let (tx, mut rx) = mpsc::channel(100);
    let (txout, rxout) = watch::channel((0, 0));
    let (txin, rxin) = broadcast::channel(2);

    let t = listen_tcp(listener, rxout.clone(), txin.clone());
    let r = run_simulation(txout, rxin, tank);

    r.await;
    t.await;
}

async fn run_simulation(txout: watch::Sender<(u16, u16)>, mut rxin: broadcast::Receiver<(u16, u16)>, mut tank: WaterTank) {
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(300)).await;
            let (outflow, _r) = rxin.recv().await.unwrap();
            tank.outflow = (outflow as f32 / 65535.0) as f64 * 40.0;
            tank.update_inflow();
            tank.update_level(0.3);
            
            // 0 - 65536
            let max = 65535 as f32 / tank.height as f32;
            let tank_level = (tank.level as f32 * max) as u16;
            
            let max = 65535 as f32 / 40 as f32;
            let tank_inflow = (tank.inflow as f32 * max) as u16;

            let value = (tank_level, tank_inflow);
            
            txout.send(value).unwrap();
            debug!("Tank: {:?}", tank);
        }
    });
}

async fn listen_tcp(listener: TcpListener, rxb: watch::Receiver<(u16, u16)>, txin: broadcast::Sender<(u16, u16)>) {
    
    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        debug!("New connection from {:?}", addr);
        handle(stream, rxb.clone(), txin.clone()).await;
    }
}

async fn handle(mut stream: TcpStream, rxb: watch::Receiver<(u16, u16)>, txin: broadcast::Sender<(u16, u16)>) {
    
    tokio::spawn(async move {
        debug!("Handle new connection");

        // In a loop, read data from the socket and write the data back.
        loop {
            let (level, inflow) = *rxb.borrow();

            let (mut reader, mut writer) = stream.split();

            // read header length
            let mut len = vec![0; 1];
            match reader.peek(&mut len).await.unwrap() {
                0 => {break},
                _ => {},
            };
            reader.read(&mut len).await.unwrap();

            // read header
            let mut header = vec![0; len[0] as usize];
            reader.read(&mut header).await.unwrap();
            let header_string = std::str::from_utf8(&header).unwrap();  
            let header: Header = serde_json::from_str(header_string).unwrap();

            // read payload
            let mut payload = vec![0; header.len as usize];
            reader.read(&mut payload).await.unwrap();
            let payload_string = std::str::from_utf8(&payload).unwrap();
            let payload: Point = serde_json::from_str(payload_string).unwrap();
            debug!("Payload {:?}", payload);

            txin.send((payload.x as u16, payload.y as u16)).unwrap();

            // write something random
            let hardcoded = Message {
                msg_type: String::from("input-register"),
                address: 0,
                tank_level: level,
                tank_inflow: inflow,
            };
            let mut hardcoded = serde_json::to_string(&hardcoded).unwrap();
            debug!("Sending {}", hardcoded);
            hardcoded.push('\n');
            writer.write_all(hardcoded.as_bytes()).await.unwrap();
        }
    });
}