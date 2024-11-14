use std::collections::HashMap;
use std::env;
use std::time::Instant;

use crate::command_parser::{ DIR_COMMAND, DB_FILENAME_COMMAND };

#[derive(Debug)]
pub struct Item {
    pub value: String,
    pub created: Instant,
    pub expires: usize,
}

#[derive(Debug)]
pub struct Config {
    pub dir: String,
    pub dbfilename: String,
}

#[derive(Debug)]
pub struct Storage {
    pub storage: HashMap<String, Item>,
    pub config: Config,
}

impl Storage {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut dir = String::from("");
        let mut dbfilename = String::from("");

        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                DIR_COMMAND => {
                    if let Some(next_arg) = args.get(i + 1) {
                        dir = next_arg.clone();
                    } else {
                        panic!("No dir value provided after DIR_COMMAND");
                    }
                }
                DB_FILENAME_COMMAND => {
                    if let Some(next_arg) = args.get(i + 1) {
                        dbfilename = next_arg.clone();
                    } else {
                        panic!("No dbfilename value provided after DB_FILENAME_COMMAND");
                    }
                }
                _ => {
                    println!("Argument not recognized: {}", arg);
                }
            }
        }

        Self {
            storage: HashMap::new(),
            config: Config { dir, dbfilename },
        }
    }

    pub fn set(&mut self, key: &str, value: &str, expires: usize) {
        self.storage.insert(key.to_string(), Item {
            value: String::from(value),
            expires,
            created: Instant::now(),
        });
    }

    pub fn get(&self, key: &str) -> Option<&Item> {
        let item = self.storage.get(key)?;
        let is_expired =
            item.expires > 0 && item.created.elapsed().as_millis() > item.expires as u128;

        match is_expired {
            true => None,
            false => Some(item),
        }
    }

    pub fn get_dir(&self) -> &str {
        &self.config.dir
    }

    pub fn get_dbfilename(&self) -> &str {
        &self.config.dbfilename
    }

}

impl Default for Item {
    fn default() -> Self {
        Item {
            value: String::new(),
            created: Instant::now(),
            expires: 0,
        }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Storage::new()
    }
}
