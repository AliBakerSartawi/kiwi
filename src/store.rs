use std::{collections::HashMap, fmt::Display, sync::Arc};

use tokio::sync::Mutex;

/// A key-value store.
/// 
/// Methods have names similar to Redis commands, if possible.
pub struct Store {
    pub map: HashMap<Key, Value>
}

pub type ArcMutexStore = Arc<Mutex<Store>>;

impl Store {
    pub fn new() -> ArcMutexStore {
        Arc::new(Mutex::new(Store {
            map: HashMap::new()
        }))
    }

    /// Sets a key-value pair in the store.
    pub fn set(&mut self, key: Key, value: Value) {
        self.map.insert(key, value);
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

#[derive(Debug, PartialEq)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool)
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Str(s) => write!(f, "str {}", s),
            Value::Int(i) => write!(f, "int {}", i),
            Value::Float(fl) => write!(f, "float {}", fl),
            Value::Bool(b) => write!(f, "bool {}", b),
        }
    }
}