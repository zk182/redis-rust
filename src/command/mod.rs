
pub mod parser;

use crate::command::parser::Parser;
use crate::storage::Storage;
use std::time::Duration;

#[derive(Debug)]
pub enum Request {
    PING,
    ECHO(String),
    SET(String, String, Option<Duration>),
    GET(Option<String>),
    ConfigGet(String, String),
    KEYS(Vec<String>),
}

#[derive(Debug)]
pub struct Command {}

impl Command {
    pub fn compute(data: &[u8], storage: &mut Storage) -> String {
        // let data = std::str::from_utf8(data).expect("Error parsing from utf8");
        // let lowercased_data = data.to_lowercase();
        // let lines: Vec<&str> = lowercased_data.split("\r\n").collect();

        // deberia volver, type command

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
            Request::ECHO (s) => {
                return Request::ECHO((key))
            // }
            // Some("echo") => Command::echo_parser(key),
            // Some("set") => Command::set_parser(lines, storage),
            // Some("get") => Command::get_parser(key, storage),
            // Some("config") => Command::get_config_parser(lines, storage),
            // _ => "+PONG\r\n".to_string(),
        }

        // let command = Parser::parse_data(data);

        // return "aaa".to_string();

        // return Command::response(command);
        

        // let command = lines.get(2).map(|&s| s.trim());
        // let key = lines.get(4).map(|&k| k.trim());

        // match command {
        //     Some("echo") => Command::echo_parser(key),
        //     Some("set") => Command::set_parser(lines, storage),
        //     Some("get") => Command::get_parser(key, storage),
        //     Some("config") => Command::get_config_parser(lines, storage),
        //     _ => "+PONG\r\n".to_string(),
        // }
    }

    // fn response(req: Request) {
    //     match req {
    //         Request::PING => "+PONG\r\n".to_string(),
    //         Request::ECHO(s) => {
    //             if let Some(key) = s {
    //                 format!("+{}\r\n", key)
    //             } else {
    //                 "$-1\r\n".to_string()
    //             }
    //         }
    //         Request::GET(k) => {}
    //         Request::SET(k, v, exp) => {}
    //         Request::ConfigGet(k, s) => {}
    //         Request::KEYS(s) => {}
    //     }
    // }

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

    fn get_parser(key: Option<&str>, storage: &mut Storage) -> String {
        key.map_or("$-1\r\n".to_string(), |key| match storage.get(key) {
            Some(item) => format!("+{}\r\n", item.value),
            None => "$-1\r\n".to_string(),
        })
    }

    fn get_config_parser(data: Vec<&str>, storage: &mut Storage) -> String {
        let value = data.get(6).map(|&v| v.trim()).unwrap();
        match value {
            "dir" => {
                let dir = Storage::get_dir(&storage);
                format!("*2\r\n$3\r\ndir\r\n${}\r\n{}\r\n", dir.len(), dir)
            }
            "dbfilename" => {
                let dbfilename = Storage::get_dbfilename(&storage);
                format!("+{}\r\n", dbfilename)
            }
            _ => "".to_string(),
        }
    }

    fn echo_parser(key: Option<&str>) -> String {
        key.map_or("$-1\r\n".to_string(), |key| format!("+{}\r\n", key))
    }
}
