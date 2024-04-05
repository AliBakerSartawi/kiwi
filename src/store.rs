use std::collections::HashMap;

/// A key-value store.
/// 
/// Methods have names similar to Redis commands, if possible.
pub struct Store {
    pub map: HashMap<Key, Value>
}

impl Store {
    pub fn new() -> Store {
        Store {
            map: HashMap::new()
        }
    }

    /// Sets a key-value pair in the store.
    /// 
    /// Returns the previous value associated with the key, if it exists.
    pub fn set(&mut self, key: Key, value: Value) -> Option<Value> {
        self.map.insert(key, value)
    }

    /// Gets the value associated with the key.
    /// 
    /// Returns `None` if the key does not exist.
    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.map.get(key)
    }

    /// Removes the key-value pair from the store.
    /// 
    /// Returns the value associated with the key, if it exists.
    pub fn del(&mut self, key: &Key) -> Option<Value> {
        self.map.remove(key)
    }
}


pub type Key = String;

pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool)
}