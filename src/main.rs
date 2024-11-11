use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let _n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => {
                        let result= redis_protocol_parser(&buf[..n]);
                        let format_result = format!("+{}",{result});
                        socket.write(format_result.as_bytes()).await.expect("Error");
                    },
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
            }
        });
    }
}

fn redis_protocol_parser(data: &[u8]) -> String {
    let data_str = std::str::from_utf8(data).expect("Error parsing from utf8");
    let binding: String = data_str.to_lowercase();
    let lines: Vec<&str> = binding.split("\r\n").collect();
    let command = lines[2];
    
    if command != "echo" {
        return "PONG\r\n".to_string();
    }

    let arg = lines[4];
    return arg.to_string()+ "\r\n";
}