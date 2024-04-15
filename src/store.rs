use std::{collections::HashMap, fmt::Display, sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use tokio::sync::Mutex;

pub type Key = String;

/// A key-value store.
///
/// Methods have names similar to Redis commands, if possible.
pub struct Store {
    pub map: HashMap<Key, Data>,
}

pub type ArcMutexStore = Arc<Mutex<Store>>;

impl Store {
    pub fn new() -> ArcMutexStore {
        Arc::new(Mutex::new(Store {
            map: HashMap::new(),
        }))
    }

    /// Touches a key, updating its last accessed time.
    /// 
    /// This is useful for implementing LRU cache eviction.
    pub fn touch(&mut self, key: &Key) -> usize {
        if let Some(v) = self.map.get_mut(key) {
            v.last_accessed = current_epoch_millis();
            v.times_accessed += 1;
            return 1;
        }
        0
    }

    pub fn touch_many(&mut self, keys: Vec<Key>) -> usize {
        keys.into_iter().map(|key| self.touch(&key)).sum()
    }

    /// Sets a key-value pair in the store.
    pub fn set(&mut self, key: Key, value: Value) {
        self.touch(&key);
        self.map.insert(key, Data::new(value));
    }

    /// Gets the value associated with the key.
    ///
    /// Returns `None` if the key does not exist.
    pub fn get(&mut self, key: &Key) -> Option<&Value> {
        self.touch(&key);
        self.map.get(key).map(|v| &v.value)
    }

    /// Removes the key-value pair from the store.
    ///
    /// Returns the value associated with the key, if it exists.
    pub fn del(&mut self, key: &Key) -> Option<Value> {
        self.map.remove(key).map(|v| v.value)
    }

    pub fn del_many(&mut self, keys: Vec<Key>) -> usize {
        keys.into_iter().filter_map(|key| self.del(&key)).count()
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
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

pub struct Data {
    pub value: Value,
    pub last_accessed: u128, // Milliseconds since UNIX epoch
    pub times_accessed: usize,
}

impl Data {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            last_accessed: 0,
            times_accessed: 0,
        }
    }
}

// TODO: move into a utils module or something
fn current_epoch_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

// TODO: write tests for store methods