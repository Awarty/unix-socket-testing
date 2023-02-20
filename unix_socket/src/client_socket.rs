use anyhow::bail;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::{UnixStream, UnixListener};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::{sleep, Duration};


pub struct Client {
    stream: Option<UnixStream>
}

impl Client {
    pub async fn write(socket_addr: &str, msg: &str) -> anyhow::Result<()> {
        sleep(Duration::from_millis(100)).await;
        match UnixStream::connect(&socket_addr).await {
            Ok(mut stream) => { 
                stream.try_write(msg.as_bytes())?;
                match stream.shutdown().await {
                    Ok(()) => { return Ok(()); }
                    Err(e) => { bail!("[Client] Problem"); }
                }
                Ok(())
            }
            Err(e) => {
                println!("[Client] Could not connect to socket");
                bail!("[Client] Problem");
            }
        }
    }
}

