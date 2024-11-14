use crate::storage::Storage;

pub const DIR_COMMAND: &str = "--dir";
pub const DB_FILENAME_COMMAND: &str = "--dbfilename";
#[derive(Debug)]
pub struct CommandParser {}


impl CommandParser {
    pub fn compute_command(data: &[u8], storage: &mut Storage) -> String {
        let data = std::str::from_utf8(data).expect("Error parsing from utf8");
        let lowercased_data = data.to_lowercase();
        let lines: Vec<&str> = lowercased_data.split("\r\n").collect();

        // println!("lines: {:?}", lines);

        let command = lines.get(2).map(|&s| s.trim());
        let key = lines.get(4).map(|&k| k.trim());

        // match command {
        match command {
            Some("echo") => CommandParser::echo_parser(key),
            Some("set") => CommandParser::set_parser(lines, storage),
            Some("get") => CommandParser::get_parser(lines, storage),
            Some("config") => CommandParser::get_config_parser(lines, storage),
            _ => "+PONG\r\n".to_string(),
        }
    }

    fn set_parser(data: Vec<&str>, storage: &mut Storage) -> String {
        let key = data.get(4).map(|k| *k).unwrap_or("");
        let value = data.get(6).map(|v| *v).unwrap_or("");
        let mut expiry = None;

        if data.len() > 8 {
            if let Some(px) = data.get(8) {
                if px == &"px" && data.len() > 10 {
                    expiry = data.get(10).and_then(|exp| exp.parse().ok());
                }
            }
        }

        if let Some(exp) = expiry {
            storage.set(key, value, exp);
        } else {
            storage.set(key, value, 0);
        }

        return "+OK\r\n".to_string();
    }

    fn get_parser(data: Vec<&str>, storage: &mut Storage) -> String {
        data.get(4)
            .map_or("$-1\r\n".to_string(), |key| match storage.get(key) {
                Some(item) => format!("+{}\r\n", item.value),
                None => "$-1\r\n".to_string(),
            })
    }

    fn get_config_parser(data: Vec<&str>, storage: &mut Storage) -> String {
        let value = data.get(6).map(|&v| v.trim()).unwrap();
        match value {
            "dir" => {
                let dir = Storage::get_dir(&storage);
                format!("*2\r\n$3\r\ndir\r\n${}\r\n{}\r\n",dir.len(), dir) 
            },
            "dbfilename" => {
                let dbfilename = Storage::get_dbfilename(&storage);
                format!("+{}\r\n", dbfilename)
            },
            _ => "".to_string(),
        }
    }

    fn echo_parser(key: Option<&str>) -> String {
        key.map_or("$-1\r\n".to_string(), |key| format!("+{}\r\n", key))
    }
}
