use std::collections::HashMap;
use std::env::args;
use std::time::Instant;

#[derive(Debug)]
pub struct Item {
    pub value: String,
    pub created: Instant,
    pub expires: usize,
}

pub struct Config {
    pub dir: String,
    pub dbfilename: String,
}

pub struct Storage {
    pub storage: HashMap<String, Item>,
    pub config: Config,
}

impl Storage {
    pub fn new() -> Self {

        //let args:Vec<_> = args().collect();
        //println!("dentro de Storage, {}", args);

        //let dir = std::env::var("DIR").unwrap_or_else(|_| "/default/path".to_string());
        //let dbfilename = std::env::var("DBFILENAME").unwrap_or_else(|_| "default.db".to_string());

        let args: Vec<_> = args().collect();

        let mut dir = "/default/path".to_string();
        let mut dbfilename = "default.db".to_string();

        for i in 0..args.len() {
            match args[i].as_str() {
                "--dir" => if i + 1 < args.len() { dir = args[i + 1].clone(); },
                "--dbfilename" => if i + 1 < args.len() { dbfilename = args[i + 1].clone(); },
                _ => {}
            }
        }

        Storage {
            storage: HashMap::new(),
            config: Config { dir, dbfilename },
        }
    }

    pub fn set(&mut self, key: &str, value: &str, expires: usize) {
        let item = Item {
            value: String::from(value),
            expires,
            created: Instant::now(),
        };
        println!("Item con key {:?} esta por setearse: {:?}", key, item);
        self.storage.insert(key.to_string(), item);
    }

    pub fn get(&self, key: &str) -> Option<&Item> {
        let item = self.storage.get(key)?;
        let is_expired =
            item.expires > 0 && item.created.elapsed().as_millis() > item.expires as u128;
        println!(
            "Getting key: {:?}, value: {:?}, is_expired : {:?},",
            key, item.value, is_expired
        );
        match is_expired {
            true => None,
            false => Some(item),
        }
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = config
    }

    pub fn get_config(&self) -> &Config {
        &self.config
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
