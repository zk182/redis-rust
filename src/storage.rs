use std::collections::HashMap;
use std::env;
use std::time::Instant;

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

pub struct Storage {
    pub storage: HashMap<String, Item>,
    pub config: Config,
}

impl Storage {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();

        let dir: String = args[2].clone();
        let dbfilename: String = args[4].clone();

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
