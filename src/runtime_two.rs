// use tcp_server::create_server;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use runtime::TurnipRuntime;

use turnip_rs::TcpStreamMessage::{Connect, Disconnect, Read, Write};
// use tcp_server::create_server;

mod runtime;
mod tcp_client;
mod tcp_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut runtime = TurnipRuntime::new("127.0.0.1:8081");

    runtime.add_connections(vec!["127.0.0.1:8080".to_string()]);

    runtime.run_blocking().await;

    // this will just complete if it is not blocking 

    Ok(())
}
