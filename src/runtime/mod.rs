use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;

use std::collections::HashMap;

use crate::models::tcp_stream_message::TcpStreamMessage;
use crate::models::tcp_stream_message::TcpStreamMessage::{
    Connect, Disconnect, Read, Write, WriteAll,
};
use crate::server::create_server;
use error::TurnipRuntimeError;
use messenger::TurnipMessenger;

mod error;
mod messenger;

pub struct TurnipRuntime {
    port: String,
    tx: Option<mpsc::Sender<TcpStreamMessage>>,
    broadcast_tx: Option<broadcast::Sender<Vec<u8>>>,
    init_connections: Vec<String>,
}

impl TurnipRuntime {
    pub fn new(port: &str) -> Self {
        TurnipRuntime {
            port: port.to_string(),
            tx: None::<mpsc::Sender<TcpStreamMessage>>,
            init_connections: vec![],
            broadcast_tx: None::<broadcast::Sender<Vec<u8>>>,
        }
    }

    // Method to implement connections as a client on this node,
    // this needs to be added before "run"
    pub fn add_connections(&mut self, addr: Vec<String>) -> &Self {
        self.init_connections = addr;
        self
    }

    pub fn is_initialized(&self) -> bool {
        self.tx.is_some()
    }

    pub fn run(&mut self) -> () {
        // TODO: think about capacity below
        let (broadcast_tx, _) = broadcast::channel::<Vec<u8>>(16);

        let broadcast_tx_clone = broadcast_tx.clone();

        self.broadcast_tx = Some(broadcast_tx);

        // tcp stream channel
        let (tx, mut rx1) = mpsc::channel::<TcpStreamMessage>(16);

        let thread_tx = tx.clone();

        let connections = self.init_connections.clone();

        // data structure for handling the tcpstreams
        tokio::spawn(async move {
            let mut stream_map: HashMap<String, (mpsc::Sender<Vec<u8>>, JoinHandle<()>)> =
                HashMap::new();

            // TODO: over here, we connect to all of the given ip's given
            for addr in connections.iter() {
                if let Ok(socket) = TcpStream::connect(addr).await {
                    handle_connection(&mut stream_map, socket, addr.clone(), thread_tx.clone());
                }
            }

            while let Some(msg) = rx1.recv().await {
                match msg {
                    Connect(addr, socket) => {
                        handle_connection(&mut stream_map, socket, addr, thread_tx.clone());
                    }
                    Disconnect(addr) => {
                        println!("You've just disconnected with Address: {addr}");

                        // this should run drop on the socket and handle supposedly
                        stream_map.remove(&addr);
                    }
                    Write(addr, msg) => {
                        // implementation for writing to another socket
                        // we would only write to another socket if:
                        // 1) we want to send them metadata based on a received request or
                        // 2) they have specified interest in a collection that we are interested in
                        // 3) we own data that another process is interested in
                        write(&mut stream_map, addr, msg).await;
                    }
                    WriteAll(msg) => {
                        // we want to write all when we make a query(such as 'SELECT first_name, last_name from customer where id = 1;')
                        // this will broadcast to everyone that we are interested in some subset of data.

                        println!("Writing to all: {:?}", msg);
                        write_to_all(&mut stream_map, msg).await;
                    }
                    Read(_addr, msg) => {
                        // implementation for reading from a specific socket
                        // When we read from other sockets, that means that either they:
                        // sending a metadata request(like other ip addresses in the landscape) or
                        // are making a query(either telling us about an insert or a giving us a select)
                        match broadcast_tx_clone.send(msg) {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("Errror with broadcasting the read: {:?}", e)
                            }
                        };
                    }
                }
            }
        });

        let tx_clone = tx.clone();

        self.tx = Some(tx);

        let port = self.port.clone();

        tokio::spawn(async move {
            match create_server(format!("127.0.0.1:{}", port), tx_clone).await {
                Ok(_r) => {}
                Err(e) => {
                    eprintln!("Error with creating server: {:?}", e);
                }
            };
        });
    }

    pub fn get_messenger(&mut self) -> Result<TurnipMessenger, TurnipRuntimeError> {
        if !self.is_initialized() {
            return Err(TurnipRuntimeError::NotIntializedError());
        }

        Ok(TurnipMessenger::new(self.tx.as_ref().unwrap().clone()))
    }

    pub fn get_receiver(&mut self) -> Result<broadcast::Receiver<Vec<u8>>, TurnipRuntimeError> {
        if let Some(tx) = self.broadcast_tx.as_ref() {
            return Ok(tx.subscribe());
        } else {
            return Err(TurnipRuntimeError::NotIntializedError());
        }
    }

    pub async fn run_blocking(&mut self) -> () {
        let (broadcast_tx, _) = broadcast::channel::<Vec<u8>>(16);

        let broadcast_tx_clone = broadcast_tx.clone();

        self.broadcast_tx = Some(broadcast_tx);

        // tcp stream channel
        let (tx, mut rx1) = mpsc::channel::<TcpStreamMessage>(16);

        let connections = self.init_connections.clone();

        let thread_tx = tx.clone();

        // data structure for handling the tcpstreams
        tokio::spawn(async move {
            let mut stream_map: HashMap<String, (mpsc::Sender<Vec<u8>>, JoinHandle<()>)> =
                HashMap::new();

            for addr in connections.iter() {
                if let Ok(socket) = TcpStream::connect(addr).await {
                    handle_connection(&mut stream_map, socket, addr.clone(), thread_tx.clone());
                }   
            }

            while let Some(msg) = rx1.recv().await {
                match msg {
                    Connect(addr, socket) => {
                        handle_connection(&mut stream_map, socket, addr, thread_tx.clone());
                    }
                    Disconnect(addr) => {
                        println!("You've just disconnected with Address: {addr}");

                        // this should run drop on the socket and handle supposedly
                        stream_map.remove(&addr);
                    }
                    Write(addr, msg) => {
                        // implementation for writing to another socket
                        // we would only write to another socket if:
                        // 1) we want to send them metadata based on a received request or
                        // 2) they have specified interest in a collection that we are interested in
                        // 3) we own data that another process is interested in
                        write(&mut stream_map, addr, msg).await;
                    }
                    WriteAll(msg) => {
                        // we want to write all when we make a query(such as 'SELECT first_name, last_name from customer where id = 1;')
                        // this will broadcast to everyone that we are interested in some subset of data.
                        write_to_all(&mut stream_map, msg).await;
                    }
                    Read(_addr, msg) => {
                        // implementation for reading from a specific socket
                        // When we read from other sockets, that means that either they:
                        // sending a metadata request(like other ip addresses in the landscape) or
                        // are making a query(either telling us about an insert or a giving us a select)
                        match broadcast_tx_clone.send(msg) {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("Errror with broadcasting the read: {:?}", e)
                            }
                        };
                    }
                }
            }
        });

        let tx_clone = tx.clone();

        self.tx = Some(tx);

        match create_server(format!("127.0.0.1:{}", self.port), tx_clone).await {
            Ok(_r) => {}
            Err(e) => {
                eprintln!("Error with creating server: {:?}", e);
            }
        };
    }
}

pub fn handle_connection(
    stream_map: &mut HashMap<String, (mpsc::Sender<Vec<u8>>, JoinHandle<()>)>,
    mut socket: TcpStream,
    addr: String,
    tx: mpsc::Sender<TcpStreamMessage>,
) {
    let reader_tx = tx.clone();

    let address = addr.clone();

    let (tx,mut rx) = mpsc::channel::<Vec<u8>>(16);

    let handle = tokio::spawn(async move {

        let mut buf = vec![0; 1024];

        loop{
            select! {
                // we have received something from the socket
                val = socket.read(&mut buf) => {
                    match val{
                        Ok(n) => {
                            if n == 0 {
                                match reader_tx
                                    .send(TcpStreamMessage::Disconnect(address.clone()))
                                    .await
                                {
                                    Ok(_r) => {}
                                    Err(e) => {
                                        eprintln!("Error with sending disconnect: {:?}", e);
                                    }
                                };
                                return;
                            }

                            match reader_tx
                            .send(TcpStreamMessage::Read(address.clone(), buf.clone()))
                            .await
                            {
                                Ok(_r) => {}
                                Err(e) => {
                                    eprintln!("Error with sending disconnect: {:?}", e);
                                }
                            };
                        },
                        Err(_e) => {
                            eprintln!("Failed to write to the socket");
                        }
                    }
                },
                // we are sending something to the socket
                val = rx.recv() => {
                    match val {
                        Some(v) => {
                            println!("Received {:?}", v);
                            match socket.write_all(&v).await {
                                Ok(_v) => {

                                },
                                Err(e) => {
                                    eprintln!("Error: {:?}",e);
                                }
                            };
                        },
                        None => {
                            eprintln!("Error when trying to write to socket");
                        }
                    }
                }
            }

            println!("Looped here");
        }
    });

    stream_map.insert(addr, (tx.clone(), handle));
}

pub async fn write_to_all(
    stream_map: &mut HashMap<String, (mpsc::Sender<Vec<u8>>, JoinHandle<()>)>,
    msg: Vec<u8>,
) {

    let keys:Vec<String> = stream_map.keys().map(|v| v.to_string()).collect();

    println!("here are the keys: {:?}", keys);

    for key in keys {

        println!("Sending to {key}");

        if let Some((socket, _)) = stream_map.get_mut(&key) {

            // TODO: error handling here
            match socket.send(msg.clone()).await {
                Ok(_r) => {}
                Err(e) => {
                    eprintln!("Error with Writing all: {:?}", e);
                }
            };
        }else{
            println!("Failed to get the addr from socket");
        }
    }
}

pub async fn write(
    stream_map: &mut HashMap<String, (mpsc::Sender<Vec<u8>>, JoinHandle<()>)>,
    key: String,
    msg: Vec<u8>,
) {
    if let Some((tx, _)) = stream_map.get_mut(&key) {

        // TODO: error handling here
        match tx.send(msg).await {
            Ok(_r) => {}
            Err(e) => {
                eprintln!("Error with Writing all: {:?}", e);
            }
        };
    }
}
