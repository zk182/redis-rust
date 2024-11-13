
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug)]
pub struct Item {
    pub value: String,
    pub created: Instant,
    pub expires: usize,
}

pub struct Storage {
    pub storage: HashMap<String, Item>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            storage: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str,  value: &str, expires: usize) {
        let item = Item {value: String::from(value), expires, created: Instant::now()};
        println!("Item con key {:?} esta por setearse: {:?}", key, item);
        self.storage.insert(key.to_string(), item);
    }

    pub fn get(&self, key: &str) -> Option<&Item> {
        let item = self.storage.get(key)?;
        let is_expired = item.expires > 0 && item.created.elapsed().as_millis() > item.expires as u128;
        println!("key: {:?}, value: {:?}, is_expired : {:?},", key, item.value, is_expired);
        match is_expired {
            true => None,
            false => Some(item)
        } 
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