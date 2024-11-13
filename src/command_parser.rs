use crate::storage::Storage;

#[derive(Debug)]
pub struct CommandParser {
}

impl CommandParser {
    pub fn compute_command(data: &[u8], storage: &mut Storage) -> String {
        let data = std::str::from_utf8(data).expect("Error parsing from utf8");
        let lowercased_data = data.to_lowercase();
        let lines: Vec<&str> = lowercased_data.split("\r\n").collect();


        // match command {
        match lines.get(2).map(|&s| s.trim()) {
            Some("echo") => CommandParser::redis_protocol_parser(lines),
            Some("set") => CommandParser::set_protocol_parser(lines, storage),
            Some("get") => CommandParser::get_protocol_parser(lines, storage),
            _ => "+PONG\r\n".to_string(),
        }
    }

    fn set_protocol_parser(data: Vec<&str>, storage: &mut Storage) -> String {
        let key = data[4].to_string();
        let value = data[6].to_string();
        let mut expiry = None;

        if data.len() > 8 {
            let px = data[8].to_string();
            if px == "px" && data.len() > 10 {
                expiry = Some(data[10].parse().expect("Bad expiry number"));
            }
        }

        if let Some(exp) = expiry {
            storage.set(&key, &value, exp);
        } else {
            storage.set(&key, &value, 0);
        }

        return "+OK\r\n".to_string();
    }

    fn get_protocol_parser(data: Vec<&str>, storage: &mut Storage) -> String {
        data.get(4)
        .map_or("$-1\r\n".to_string(), |key| {
            match storage.get(key) {
                Some(item) => format!("+{}\r\n", item.value),
                None => "$-1\r\n".to_string(),
            }
        })
    }

    fn redis_protocol_parser(data: Vec<&str>) -> String {
        data.get(4)
        .map_or("$-1\r\n".to_string(), |key| format!("+{}\r\n", key))
    }
}
