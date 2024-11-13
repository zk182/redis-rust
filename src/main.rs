use storage::Storage;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod storage;

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
                        let result= compute_command(&buf[..n], &mut storage);
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

fn compute_command(data: &[u8], storage: &mut Storage) -> String{
    let data = std::str::from_utf8(data).expect("Error parsing from utf8");
    let binding: String = data.to_lowercase();
    let lines: Vec<&str> = binding.split("\r\n").collect();
    let command = lines[2];

    match command {
        "echo" => redis_protocol_parser(lines),
        "set" => set_protocol_parser(lines, storage),
        "get" => get_protocol_parser(lines, storage),
        _ =>  "PONG\r\n".to_string(),
    }
}

fn set_protocol_parser(data: Vec<&str>, storage: &mut Storage) -> String {
    let key = data[4].to_string();
    let value = data[6].to_string();

    if data.len() > 6 {
        let px = data[7].to_string();
        if px == "px" {
            let exp: usize = data[8].parse().expect("Bad expiry number");
            storage.set(&key, &value, exp);
        }
    }

    return "OK\r\n".to_string();
}

fn get_protocol_parser(data: Vec<&str>, storage: &mut Storage) -> String {
    let key = data[4].to_string();
    let item = storage.get(&key).unwrap();
    println!("Item es : {:?}", item);
    return item.value.clone();
}

fn redis_protocol_parser(data: Vec<&str>) -> String {
    return data[4].to_string() + "\r\n";
}