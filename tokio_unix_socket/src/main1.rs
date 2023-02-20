use tokio::net::{UnixListener, UnixStream};
use tokio::time::{sleep, Duration};
use std::io;

use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

async fn process(socket: UnixStream) {
    let (tx, mut rx) = mpsc::channel(5);
    tokio::spawn(async move {
        loop{
            let msg = rx.recv().await;
            println!("{:?}", msg);
            println!("AAA");
        }
    });

    let mut data = vec![0; 1024];
    loop {
        match socket.try_read(&mut data) {
            Ok(0) => break,
            Ok(n) => {
                println!("read {} bytes", n);   
                //println!("{:?}", data);  
                tx.send("Hello World".to_string()).await;   
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                println!("Failed");
            }
        }    
    }
}

async fn start_client() -> anyhow::Result<()>{
    sleep(Duration::from_millis(1000)).await;
    let stream = UnixStream::connect("socket").await?;
    loop {
        stream.try_write(b"Hello World")?;
        sleep(Duration::from_millis(1000)).await;
    }
}

#[tokio::main]
async fn main() {
    //Start Client
    tokio::spawn(async move {
        start_client().await;
    });

    // Start Server
    std::fs::remove_file("socket");
    let listener = UnixListener::bind("socket").unwrap();
    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                tokio::spawn(async move {
                    println!("new client!");
                    process(stream).await;
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }
}