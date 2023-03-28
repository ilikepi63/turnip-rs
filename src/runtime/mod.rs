use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use std::collections::HashMap;
use std::sync::Arc;

use crate::tcp_server::create_server;
use error::TurnipRuntimeError;
use messenger::TurnipMessenger;
use turnip_rs::TcpStreamMessage::{self, Connect, Disconnect, Read, Write, WriteAll};

mod error;
mod messenger;

pub struct TurnipRuntime {
    port: String,
    tx: Option<mpsc::Sender<TcpStreamMessage>>,
    init_connections: Vec<String>,
}

impl TurnipRuntime {
    pub fn new(port: &str) -> Self {
        TurnipRuntime {
            port: port.to_string(),
            tx: None,
            init_connections: vec![],
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
        // tcp stream channel
        let (tx, mut rx1) = mpsc::channel::<TcpStreamMessage>(16);

        let thread_tx = tx.clone();

        let connections = self.init_connections.clone();

        // data structure for handling the tcpstreams
        tokio::spawn(async move {
            let mut stream_map: HashMap<String, (Arc<Mutex<TcpStream>>, JoinHandle<()>)> =
                HashMap::new();

            // TODO: over here, we connect to all of the given ip's given
            for addr in connections.iter() {
                if let Ok(socket) = TcpStream::connect(addr).await {
                    println!("You've just connected with Address: {addr}");

                    // this would then spawn a listener on that stream specifically right?
                    let sock = Arc::new(Mutex::new(socket));

                    let socket_clone = Arc::clone(&sock);

                    let reader_tx = thread_tx.clone();

                    let address = addr.clone();

                    let handle = tokio::spawn(async move {
                        let mut buf = vec![0; 1024];

                        println!("Does this ever get called?");

                        let mut sock_mutex = socket_clone.lock().await;

                        println!("Does this ever get called?");

                        loop {
                            let n = match (*sock_mutex).read(&mut buf).await {
                                Ok(result) => result,
                                Err(e) => {
                                    println!("Failed to read anything from the socket: {:?}", e);
                                    0
                                }
                            };

                            // the connection has been closed
                            if n == 0 {
                                println!("We are going to disconnect now...");
                                reader_tx
                                    .send(TcpStreamMessage::Disconnect(address.clone()))
                                    .await;
                                return;
                            }

                            println!("I am atleast reading something here: {}", n);

                            if let Ok(msg) = std::str::from_utf8(&buf) {
                                reader_tx
                                    .send(TcpStreamMessage::Read(address.clone(), msg.to_string()))
                                    .await;
                            }
                        }
                    });

                    stream_map.insert(addr.to_string(), (sock, handle));
                }
            }

            while let Some(msg) = rx1.recv().await {
                match msg {
                    Connect(addr, socket) => {
                        println!("You've just connected with Address: {addr}");

                        // this would then spawn a listener on that stream specifically right?
                        let sock = Arc::new(Mutex::new(socket));

                        let socket_clone = Arc::clone(&sock);

                        let reader_tx = thread_tx.clone();

                        let address = addr.clone();

                        let handle = tokio::spawn(async move {
                            let mut buf = vec![0; 1024];

                            println!("Does this ever get called?");

                            let mut sock_mutex = socket_clone.lock().await;

                            println!("Does this ever get called?");

                            loop {
                                let n = match (*sock_mutex).read(&mut buf).await {
                                    Ok(result) => result,
                                    Err(e) => {
                                        println!(
                                            "Failed to read anything from the socket: {:?}",
                                            e
                                        );
                                        0
                                    }
                                };

                                // the connection has been closed
                                if n == 0 {
                                    println!("We are going to disconnect now...");
                                    reader_tx
                                        .send(TcpStreamMessage::Disconnect(address.clone()))
                                        .await;
                                    return;
                                }

                                println!("I am atleast reading something here: {}", n);

                                if let Ok(msg) = std::str::from_utf8(&buf) {
                                    reader_tx
                                        .send(TcpStreamMessage::Read(
                                            address.clone(),
                                            msg.to_string(),
                                        ))
                                        .await;
                                }
                            }
                        });

                        stream_map.insert(addr, (sock, handle));
                    }
                    Disconnect(addr) => {
                        println!("You've just disconnected with Address: {addr}");

                        // this should run drop on the socket and handle supposedly
                        stream_map.remove(&addr);
                        println!("This is the result from shutting down: {:?}", stream_map);
                    }
                    Write(addr, msg) => {
                        // implementation for writing to another socket
                        // we would only write to another socket if:
                        // 1) we want to send them metadata based on a received request or
                        // 2) they have specified interest in a collection that we are interested in
                        // 3) we own data that another process is interested in
                    }
                    WriteAll(msg) => {
                        // we want to write all when we make a query(such as 'SELECT first_name, last_name from customer where id = 1;')
                        // this will broadcast to everyone that we are interested in some subset of data.
                    }
                    Read(addr, msg) => {
                        // implementation for reading from a specific socket
                        // When we read from other sockets, that means that either they:
                        // sending a metadata request(like other ip addresses in the landscape) or
                        // are making a query(either telling us about an insert or a giving us a select)
                        println!("Successfully read: {} from the socket", msg);
                    }
                    _ => {}
                }
            }
        });

        let tx_clone = tx.clone();

        self.tx = Some(tx);

        tokio::spawn(async move {
            create_server("127.0.0.1:8080".to_string(), tx_clone).await;
        });
    }

    pub fn get_messenger(&mut self) -> Result<TurnipMessenger, TurnipRuntimeError> {
        if !self.is_initialized() {
            return Err(TurnipRuntimeError::NotIntializedError());
        }

        Ok(TurnipMessenger::new(self.tx.as_ref().unwrap().clone()))
    }

    pub async fn run_blocking(&mut self) -> () {
        // tcp stream channel
        let (tx, mut rx1) = mpsc::channel::<TcpStreamMessage>(16);

        let connections = self.init_connections.clone();

        let thread_tx = tx.clone();

        // data structure for handling the tcpstreams
        tokio::spawn(async move {
            let mut stream_map: HashMap<String, (Arc<Mutex<TcpStream>>, JoinHandle<()>)> =
                HashMap::new();

                
        for addr in connections.iter() {
            if let Ok(socket) = TcpStream::connect(addr).await {
                println!("You've just connected with Address: {addr}");

                // this would then spawn a listener on that stream specifically right?
                let sock = Arc::new(Mutex::new(socket));

                let socket_clone = Arc::clone(&sock);

                let reader_tx = thread_tx.clone();

                let address = addr.clone();

                let handle = tokio::spawn(async move {
                    let mut buf = vec![0; 1024];

                    println!("Does this ever get called?");

                    let mut sock_mutex = socket_clone.lock().await;

                    println!("Does this ever get called?");

                    loop {
                        let n = match (*sock_mutex).read(&mut buf).await {
                            Ok(result) => result,
                            Err(e) => {
                                println!("Failed to read anything from the socket: {:?}", e);
                                0
                            }
                        };

                        // the connection has been closed
                        if n == 0 {
                            println!("We are going to disconnect now...");
                            reader_tx
                                .send(TcpStreamMessage::Disconnect(address.clone()))
                                .await;
                            return;
                        }

                        println!("I am atleast reading something here: {}", n);

                        if let Ok(msg) = std::str::from_utf8(&buf) {
                            reader_tx
                                .send(TcpStreamMessage::Read(address.clone(), msg.to_string()))
                                .await;
                        }
                    }
                });

                stream_map.insert(addr.to_string(), (sock, handle));
            }
        }

            while let Some(msg) = rx1.recv().await {
                match msg {
                    Connect(addr, socket) => {
                        println!("You've just connected with Address: {addr}");

                        // this would then spawn a listener on that stream specifically right?
                        let sock = Arc::new(Mutex::new(socket));

                        let socket_clone = Arc::clone(&sock);

                        let reader_tx = thread_tx.clone();

                        let address = addr.clone();

                        let handle = tokio::spawn(async move {
                            let mut buf = vec![0; 1024];

                            println!("Does this ever get called?");

                            let mut sock_mutex = socket_clone.lock().await;

                            println!("Does this ever get called?");

                            loop {
                                let n = match (*sock_mutex).read(&mut buf).await {
                                    Ok(result) => result,
                                    Err(e) => {
                                        println!(
                                            "Failed to read anything from the socket: {:?}",
                                            e
                                        );
                                        0
                                    }
                                };

                                // the connection has been closed
                                if n == 0 {
                                    println!("We are going to disconnect now...");
                                    reader_tx
                                        .send(TcpStreamMessage::Disconnect(address.clone()))
                                        .await;
                                    return;
                                }

                                println!("I am atleast reading something here: {}", n);

                                if let Ok(msg) = std::str::from_utf8(&buf) {
                                    reader_tx
                                        .send(TcpStreamMessage::Read(
                                            address.clone(),
                                            msg.to_string(),
                                        ))
                                        .await;
                                }
                            }
                        });

                        stream_map.insert(addr, (sock, handle));
                    }
                    Disconnect(addr) => {
                        println!("You've just disconnected with Address: {addr}");

                        // this should run drop on the socket and handle supposedly
                        stream_map.remove(&addr);
                        println!("This is the result from shutting down: {:?}", stream_map);
                    }
                    Write(_addr, _msg) => {
                        // implementation for writing to another socket
                        // we would only write to another socket if:
                        // 1) we want to send them metadata based on a received request or
                        // 2) they have specified interest in a collection that we are interested in
                        // 3) we own data that another process is interested in
                    }
                    WriteAll(_msg) => {
                        // we want to write all when we make a query(such as 'SELECT first_name, last_name from customer where id = 1;')
                        // this will broadcast to everyone that we are interested in some subset of data.
                    }
                    Read(_addr, msg) => {
                        // implementation for reading from a specific socket
                        // When we read from other sockets, that means that either they:
                        // sending a metadata request(like other ip addresses in the landscape) or
                        // are making a query(either telling us about an insert or a giving us a select)
                        println!("Successfully read: {} from the socket", msg);
                    }
                    _ => {}
                }
            }
        });

        let tx_clone = tx.clone();

        self.tx = Some(tx);

        create_server(self.port.clone().to_string(), tx_clone).await;
    }
}
