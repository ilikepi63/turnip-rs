use crate::models::tcp_stream_message::TcpStreamMessage;
use tokio::sync::mpsc;

// use crate::TcpStreamMessage::{Connect, Disconnect, Read, Write};

#[derive(Debug, Clone)]
pub struct TurnipMessenger {
    tx: mpsc::Sender<TcpStreamMessage>,
}

impl TurnipMessenger {
    pub fn new(tx: mpsc::Sender<TcpStreamMessage>) -> Self {
        TurnipMessenger { tx: tx }
    }

    pub async fn write(&self, addr: String, message: Vec<u8>) {
        match self.tx.send(TcpStreamMessage::Write(addr, message)).await {
            Ok(_r) => {}
            Err(e) => {
                eprintln!("Error with Writing: {:?}", e);
            }
        };
    }

    pub async fn write_all(&self, message: Vec<u8>) {
        match self.tx.send(TcpStreamMessage::WriteAll(message)).await {
            Ok(_r) => {}
            Err(e) => {
                eprintln!("Error with Writing: {:?}", e);
            }
        };
    }
}

unsafe impl Send for TurnipMessenger {}
unsafe impl Sync for TurnipMessenger {}
