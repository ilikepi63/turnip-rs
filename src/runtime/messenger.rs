use tokio::sync::mpsc;
use turnip_rs::TcpStreamMessage;

// use crate::TcpStreamMessage::{Connect, Disconnect, Read, Write};

#[derive(Debug, Clone)]
pub struct TurnipMessenger {
    tx: mpsc::Sender<TcpStreamMessage>,
}

impl TurnipMessenger {
    pub fn new(tx: mpsc::Sender<TcpStreamMessage>) -> Self{
        TurnipMessenger { tx: tx }
    }

    pub fn write(addr: String, message: String) {}

    pub fn write_all(message: String) {}
}


unsafe impl Send for TurnipMessenger {}
unsafe impl Sync for TurnipMessenger {}