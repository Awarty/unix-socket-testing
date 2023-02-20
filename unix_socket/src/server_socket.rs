use anyhow::bail;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::{UnixStream, UnixListener};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

use super::unix_socket;

pub struct Server {
    stream: Option<UnixStream>,
    socket_addr: String,
    rx0: Receiver<String>,
    tx0: Sender<String>,
    rx1: Receiver<String>,
    tx1: Sender<String>,
    // rx0: Option<Receiver<String>>,
    // tx0: Option<Sender<String>>,
    // rx1: Option<Receiver<String>>,
    // tx1: Option<Sender<String>>,
    task_handler: Option<tokio::task::JoinHandle<()>>,
}

impl Server {
    pub async fn new(socket_addr: &str) -> Self{
        // Remove the socket if it already exists
        match tokio::fs::remove_file(&socket_addr).await {
            Ok(_) => {
                println!("[Server] Removed old socket: {socket_addr}");
            }
            Err(e) => {
                println!("[Server] Failed to removed file: {e:?}");
            }
        }
        let (tx0, rx0) = mpsc::channel(32);
        let (tx1, rx1) = mpsc::channel(32);
        Server {
            stream: None,
            socket_addr: socket_addr.to_string(),
            rx0: rx0,
            tx0: tx0,
            rx1: rx1,
            tx1: tx1,
            // rx0: None,
            // tx0: None,
            // rx1: None,
            // tx1: None,
            task_handler: None
        }
    }
    
    pub fn start(&mut self) {
        let socket = self.socket_addr.clone();
        let tx = self.tx0.clone();
        let rx = self.rx1;
        let server_handler = tokio::spawn(async move {
            let listener_result = UnixListener::bind(&socket);
            match listener_result {
                Ok(listener) => {
                    loop {
                        match listener.accept().await {
                            Ok((stream, _)) => {
                                println!("[Server] Client Connected");
                                // let (tx0, rx0) = mpsc::channel(32);
                                // let (tx1, rx1) = mpsc::channel(32);
                                // self.rx0 = Some(rx0);
                                // self.tx1 = Some(tx1);
                                Self::listen_socket(stream, tx, rx).await;
                            }
                            Err(e) => {
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("[Server] Faild to create unix listener: {e:?}");
                }
            }
        });
        self.task_handler = Some(server_handler);
    }

    pub async fn listen_socket(mut stream: UnixStream, tx0: Sender<String>, mut rx1: Receiver<String>) {
        tokio::spawn(async move {
            let mut buffer = String::new();
            tokio::select! {
                data_read = stream.read_to_string(&mut buffer) => {}
                data_write = rx1.recv() => {}
            }
            println!("[Server] Got this message: {}", buffer);
            tx0.send(buffer).await;
        });
    }

    pub fn write_socket(&self, data: &str) {
        
    }

    pub async fn read_socket(&mut self) -> Option<String> {
        return self.rx0.recv().await;
    }

    pub fn stop(&self)
    {
        match &self.task_handler {
            Some(handler) => {
                println!("[Server] Socket closed from server");
                handler.abort(); 
            }
            None => { println!("[Server] No handler for a server is active."); }
        }
    }
    
}


