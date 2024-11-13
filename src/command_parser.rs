use crate::storage::Storage;

#[derive(Debug)]
pub struct CommandParser {
    pub data: [u8; 1024],
}

impl CommandParser {
    pub fn new() -> Self {
        CommandParser { data: [0; 1024] }
    }

    pub fn compute_command(&self, data: &[u8], storage: &mut Storage) -> String {
        let data = std::str::from_utf8(data).expect("Error parsing from utf8");
        let binding: String = data.to_lowercase();
        let lines: Vec<&str> = binding.split("\r\n").collect();
        let command = lines[2];

        match command {
            "echo" => self.redis_protocol_parser(lines),
            "set" => self.set_protocol_parser(lines, storage),
            "get" => self.get_protocol_parser(lines, storage),
            _ => "+PONG\r\n".to_string(),
        }
    }

    fn set_protocol_parser(&self, data: Vec<&str>, storage: &mut Storage) -> String {
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

    fn get_protocol_parser(&self, data: Vec<&str>, storage: &mut Storage) -> String {
        let key = data[4].to_string();
        match storage.get(&key) {
            Some(item) => return format!("+{}\r\n", item.value.clone()),
            None => return "$-1\r\n".to_string(),
        };
    }

    fn redis_protocol_parser(&self, data: Vec<&str>) -> String {
        return format!("+{}\r\n", data[4]);
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        CommandParser { data: [0; 1024] }
    }
}
