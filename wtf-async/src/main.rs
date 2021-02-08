use std::time::{SystemTime};
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    println!("Main start with {:?}", std::thread::current().id());
    let now: SystemTime = SystemTime::now();
    let task1 = waiting(now, 1.0);
    let task2 = waiting(now, 2.0);
    let task3 = waiting(now, 4.0);
    
    task1.await;
    task2.await;
    task3.await;


    loop {}
}

async fn waiting(time: SystemTime,delay: f32) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            let starttime = time.elapsed().unwrap().as_secs_f32();
            tokio::time::sleep(tokio::time::Duration::from_secs_f32(delay)).await;
            println!("Waiting process {:?} {:?} started after {:.1} secs, has waited for {:.1} secs!", std::thread::current().id(), std::process::id(), starttime, delay);
        }
    })
}