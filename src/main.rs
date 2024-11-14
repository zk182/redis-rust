use command::CommandParser;
use storage::Storage;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;


mod storage;
mod args;
mod command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut storage = Storage::new();

            // In a loop, read data from the socket and write the data back.
            loop {
                let _n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => {
                        let result = CommandParser::compute_command(&buf[..n], &mut storage);
                        socket.write(result.as_bytes()).await.expect("Error");
                    }
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
            }
        });
    }
}