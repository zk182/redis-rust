use super::Request;

pub struct Parser{}

impl Parser {
    pub fn parse_data(data: &[u8]) -> Request {
        let data_str = match std::str::from_utf8(data) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error parsing UTF-8: {}", e);
                return Request::PING;
            }
        };

        let lowercased_data = data_str.to_lowercase();
        let lines: Vec<&str> = lowercased_data.split("\r\n").collect();

        // println!("lines: {:?}", lines);

        let command = lines.get(2).map(|&s| s.trim());
        let key = lines.get(4).map(|&k| k.trim());

         match command {
            Request::ECHO => {
                return Request::ECHO()
            // }
            // Some("echo") => Command::echo_parser(key),
            // Some("set") => Command::set_parser(lines, storage),
            // Some("get") => Command::get_parser(key, storage),
            // Some("config") => Command::get_config_parser(lines, storage),
            // _ => "+PONG\r\n".to_string(),
        }
        
        
        // let lowercased_data = data_str.to_lowercase();
        // let lines: Vec<&str> = lowercased_data.split("\r\n").collect();
        


        // return "aaa".to_string();
    }
}