use std::error::Error;
use std::time::Duration;
use tcp_client::make_client;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

mod tcp_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = make_client("127.0.0.1:8080").await?;

    loop {
        sleep(Duration::from_millis(1000)).await;
        let _result = stream.write_all(b"hello world\n").await;
    }

    Ok(())
}
