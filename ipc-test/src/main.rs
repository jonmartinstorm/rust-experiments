use std::time::{SystemTime};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[tokio::main]
async fn main() {
    println!("Main start");
    let now: SystemTime = SystemTime::now();
    let counter: u64 = 1;
    let (tx, mut rx) = mpsc::channel(32);
    let (txo, rxo) = oneshot::channel();

    let mut time = now.elapsed().unwrap().as_secs_f32();
    listen_unix(time, counter, tx.clone(), txo).await;

    listen_tcp(time, counter, tx.clone(), rxo).await;

    simulation(time, counter, tx.clone()).await;

    std::thread::sleep(std::time::Duration::from_millis(100));
    time = now.elapsed().unwrap().as_secs_f32();
    println!("Time: {:.1}. starting Unix, Tcp and simulation {}", time, counter);

    // spawn reciever as a task so it ends when the program ends, maybe not the best solution
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("Got = {}", message);
        }
    });
    
    // let the threads finish
    std::thread::sleep(std::time::Duration::from_millis(3000));
    println!("Program finished in {:.1} seconds", now.elapsed().unwrap().as_secs_f32());
}


async fn listen_unix(time: f32, counter: u64, txi: mpsc::Sender<String>, txo: oneshot::Sender<String>) {
    for _ in 0..10 {
        let tx = txi.clone();
        tokio::spawn(async move {
            let id = std::thread::current().id();
            let t1: SystemTime = SystemTime::now();
            tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
            tx.send(format!("Unx listen {:02} startet {:.1}, is done after {:.1} secs and {:?}", counter, time, t1.elapsed().unwrap().as_secs_f32(), id)).await.unwrap();
            //println!("Unix listen {} startet {:.1}, is done {:?}", counter, time, id);
        });
    }
    let p = Point { x: 1, y: 2 };
    txo.send(serde_json::to_string(&p).unwrap()).unwrap();
}

async fn listen_tcp(time: f32, counter: u64, tx: mpsc::Sender<String>, rxo: oneshot::Receiver<String>) {
    tokio::spawn(async move {
        let id = std::thread::current().id();
        let t1: SystemTime = SystemTime::now();
        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
        tx.send(format!("Tcp listen {:02} startet {:.1}, is done after {:.1} secs and {:?}", counter, time, t1.elapsed().unwrap().as_secs_f32(), id)).await.unwrap();
        //println!("Tcp listen {} startet {:.1}, is done {:?}", counter, time, id);
        let s = match rxo.await {
            Ok(msg) => msg,
            _ => "nothing".to_string(),
        };
        println!("Recieved: {}", s);
    });
}

async fn simulation(time: f32, counter: u64, tx: mpsc::Sender<String>) {
    tokio::spawn(async move {
        let id = std::thread::current().id();
        let t1: SystemTime = SystemTime::now();
        tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
        tx.send(format!("Simulation {:02} startet {:.1}, is done after {:.1} secs and {:?}", counter, time, t1.elapsed().unwrap().as_secs_f32(), id)).await.unwrap();
        //println!("Simulation {} startet {:.1}, is done {:?}", counter, time, id);
    });
}
