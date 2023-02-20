use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::{UnixListener, UnixStream};
use tokio::time::{sleep, Duration};
use std::io;
use tokio::fs;


async fn server() {
    // Start Server
    std::fs::remove_file("socket");
    let listener = UnixListener::bind("socket").unwrap();
    match listener.accept().await {
        Ok((mut stream, _addr)) => {
            tokio::spawn(async move {
                println!("new client!");
                let mut buf = [0; 4096];
                // match stream.try_read(&mut buf) {
                //     Ok(n) => {
                //         println!("read {} bytes", n);   
                //         //println!("{:?}", data);   
                //     }
                //     Err(e) => {
                //         println!("Failed");
                //     }
                // }
                let mut buffer = String::new();
                stream.read_to_string(&mut buffer).await.unwrap();
                println!("Got this message: {}", buffer);
            });
        }
        Err(e) => { /* connection failed */ }
    }
}

async fn client() -> anyhow::Result<()>{
    sleep(Duration::from_millis(1000)).await;
    let mut stream = UnixStream::connect("socket").await?;
    loop {
        stream.try_write(b"Hello World")?;
        sleep(Duration::from_millis(1000)).await;
        match stream.shutdown().await {
            Ok(()) => { println!("AAA"); }
            Err(e) => { println!("{e:?}"); }
        }
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        server().await;
    });
    tokio::spawn(async move {
        client().await;
    });
    loop{}
}


