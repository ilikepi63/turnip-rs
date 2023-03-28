use tokio::net::TcpStream;

pub enum TcpStreamMessage {
    Connect(String, TcpStream),
    Disconnect(String),
    Write(String, String),
    Read(String, String),
    WriteAll(String),
}
