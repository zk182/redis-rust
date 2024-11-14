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
        // let mut iterator = args.iter();

        let mut dir = String::from("");
        let mut dbfilename = String::from("");

        // for arg in iterator {
        //     match arg.as_str() {
        //         DIR_COMMAND => {
        //             let next = iterator.next();
        //             match next {
        //                 Some(t)=> {
        //                     dir = t.to_string()
        //                 },
        //                 None => {
        //                     dir = String::from("");
        //                 }
        //             }
        //             // dir = iterator.next().expect("No dir command provided").to_string();
        //         }
        //         DB_FILENAME_COMMAND => {
        //             dbfilename = iterator.next().expect("No dbfilename command provided").to_string();
        //         },
        //         _ => println!("AA"),
        //     }
        // }

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
