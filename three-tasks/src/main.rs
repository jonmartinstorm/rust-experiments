use std::time::{SystemTime};

#[tokio::main]
async fn main() {
    println!("Main start");
    let now: SystemTime = SystemTime::now();
    while now.elapsed().unwrap().as_secs_f32() < 2.5 {
        let mut time = now.elapsed().unwrap().as_secs_f32();
        listen_unix(time).await;

        listen_tcp(time).await;

        simulation(time).await;

        std::thread::sleep(std::time::Duration::from_millis(100));
        time = now.elapsed().unwrap().as_secs_f32();
        println!("Time: {:.1}", time);
    }

    // let the threads finish
    std::thread::sleep(std::time::Duration::from_millis(3000));
    println!("Program finished in {:.1} seconds", now.elapsed().unwrap().as_secs_f32());
}


async fn listen_unix(time: f32) {
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        println!("Unix listen startet {:.1}, is done", time);
    });
}

async fn listen_tcp(time: f32) {
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
        println!("Tcp listen startet {:.1}, is done", time);
    });
}

async fn simulation(time: f32) {
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
        println!("Simulation startet {:.1}, is done", time);
    });
}