// use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;

pub async fn make_client(addr: &str) -> Result<TcpStream, Box<dyn Error>> {
    Ok(TcpStream::connect(addr).await?)
}

// let mut stream =
// println!("created stream");

// let result = stream.write(b"hello world\n").await;
// println!("wrote to stream; success={:?}", result.is_ok());
