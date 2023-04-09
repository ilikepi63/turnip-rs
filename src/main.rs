use crate::runtime::TurnipRuntime;
use std::error::Error;

pub mod db;
pub mod messaging;
pub mod models;
pub mod runtime;
pub mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut runtime = TurnipRuntime::new("127.0.0.1:8080");

    runtime.run_blocking().await;

    // this will just complete if it is not blocking

    Ok(())
}
