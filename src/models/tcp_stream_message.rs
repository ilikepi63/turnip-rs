use tokio::net::TcpStream;

#[derive(Debug)]
pub enum TcpStreamMessage {
    Connect(String, TcpStream),
    Disconnect(String),
    Write(String, Vec<u8>),
    Read(String, Vec<u8>),
    WriteAll(Vec<u8>),
}
