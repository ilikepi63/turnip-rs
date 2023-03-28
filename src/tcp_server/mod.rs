// use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;

use std::error::Error;

use turnip_rs::TcpStreamMessage;

pub async fn create_server(
    addr: String,
    tx: Sender<TcpStreamMessage>,
) -> Result<TcpStream, Box<dyn Error>> {
    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (stream, remote_address) = listener.accept().await?;

        println!("Received connection from {}", remote_address);

        tx.send(TcpStreamMessage::Connect(
            remote_address.to_string(),
            stream,
        ))
        .await;

        // tokio::spawn(async move {
        // let mut buf = vec![0; 1024];
        // loop {
        //     let n = socket
        //         .read(&mut buf)
        //         .await
        //         .expect("failed to read data from socket");

        //     if n == 0 {
        //         return;
        //     }

        //     socket
        //         .write_all(&buf[0..n])
        //         .await
        //         .expect("failed to write data to socket");
        // }
        // });
    }
}
