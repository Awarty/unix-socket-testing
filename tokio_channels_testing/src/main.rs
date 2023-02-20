
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::{sleep, Duration};
use std::{thread, time, clone};

async fn PIS () {
    println!("Started 'Passenger Information Service'.");
    let (tx, mut rx) = mpsc::channel(3);
    tokio::spawn(async move { 
        println!("Socket started");
        loop {
            tx.send("Hello World!").await;
            sleep(Duration::from_millis(1000)).await;
        }
    });

    // Read twin
    tokio::spawn(async move { 
        loop {
            let twin = rx.recv().await;
            match twin {
                Some(twin) => { println!("Twin: {twin:?}"); }
                None => { println!("Transmition closed"); }                
            }
        }
    });
}



#[tokio::main]
async fn main() {
    tokio::spawn(async move { PIS().await; });
    
    tokio::spawn(async move {
        loop {
            //socket.write_all(b"hello world\n");
            sleep(Duration::from_millis(1000)).await;
        }
    });
    
    loop{}
}

