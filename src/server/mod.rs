use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;

use std::error::Error;

use crate::models::tcp_stream_message::TcpStreamMessage;

pub async fn create_server(
    addr: String,
    tx: Sender<TcpStreamMessage>,
) -> Result<TcpStream, Box<dyn Error>> {
    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (stream, remote_address) = listener.accept().await?;

        println!("Received connection from {}", remote_address);

        match tx
            .send(TcpStreamMessage::Connect(
                remote_address.to_string(),
                stream,
            ))
            .await
        {
            Ok(_r) => {}
            Err(e) => {
                eprintln!("Error with writing to sockets: {:?} ", e);
            }
        };
    }
}
