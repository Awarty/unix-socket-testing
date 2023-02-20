
//use unix_socket::unix_socket::{ start_socket, connect_socket };
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::{sleep, Duration};

use unix_socket::server_socket::Server;
use unix_socket::client_socket::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    tokio::spawn(async move {
        let mut server = Server::new("socket").await;
        server.start();

        server.write_socket("Hello from the server");
        sleep(Duration::from_millis(1000)).await;            
        let msg = server.read_socket().await;
        match msg {
            Some(msg) => {
                println!("[main server] Msg: {}", msg);
            }
            None => {}
        }
        sleep(Duration::from_millis(1000)).await;            
        server.stop();
    });

    tokio::spawn(async move {
        loop {
            Client::write("socket", "Hello World").await;
            sleep(Duration::from_millis(1000)).await;            
        }
    });

    loop{}
}
