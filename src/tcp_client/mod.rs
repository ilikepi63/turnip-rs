use tokio::net::TcpStream;

use std::error::Error;

pub async fn make_client(addr: &str) -> Result<TcpStream, Box<dyn Error>> {
    Ok(TcpStream::connect(addr).await?)
}
