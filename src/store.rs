use std::{
    fmt::Display,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use scc::HashMap;

use ahash::AHasher;
use std::hash::BuildHasher;

// Define a BuildHasher that uses ahash
pub struct AHashBuilder;

impl BuildHasher for AHashBuilder {
    type Hasher = AHasher;

    fn build_hasher(&self) -> Self::Hasher {
        AHasher::default() // Use some fixed seeds
    }
}

pub type Key = String;

/// A key-value store.
///
/// Methods have names similar to Redis commands, if possible.
pub struct Store {
    /// Why chose scc::HashMap with ahash instead of DashMap
    /// https://github.com/wvwwvwwv/conc-map-bench?tab=readme-ov-file
    pub map: HashMap<Key, Data, AHashBuilder>,
}

pub type ConcurrentStore = Arc<Store>;

impl Store {
    pub fn new() -> ConcurrentStore {
        Arc::new(Store {
            map: HashMap::with_hasher(AHashBuilder),
        })
    }

    /// Touches a key, updating its last accessed time.
    ///
    /// This is useful for implementing LRU cache eviction.
    pub fn touch(&self, key: &Key) -> usize {
        if let Some(mut entry) = self.map.get(key) {
            let data = entry.get_mut();
            data.last_accessed = current_epoch_millis();
            data.times_accessed += 1;
            return 1;
        }
        0
    }

    pub fn touch_many(&self, keys: Vec<Key>) -> usize {
        keys.into_iter().map(|key| self.touch(&key)).sum()
    }

    /// Sets a key-value pair in the store.
    pub fn set(&self, key: Key, value: Value) {
        if self.map.contains(&key) {
            // TODO optimize by moving the touch logic inside the update below to avoid multiple lookups
            self.touch(&key);
            self.map.update(&key, |_, data| {
                data.value = value;
            });
        } else {
            // TODO handle Result here
            let _ = self.map.insert(key, Data::new(value));
        }
    }

    /// Gets the value associated with the key.
    ///
    /// Returns `None` if the key does not exist.
    pub fn get(&self, key: &Key) -> Option<String> {
        self.touch(&key);
        self.map.read(key, |_, data| data.value.to_string())
    }

    /// Removes the key-value pair from the store.
    ///
    /// Returns the value associated with the key, if it exists.
    pub fn del(&self, key: &Key) -> Option<Value> {
        self.map.remove(key).map(|v| v.1.value)
    }

    // TODO: optimize by checking if multiple keys can be deleted at once instead of iterating
    pub fn del_many(&self, keys: Vec<Key>) -> usize {
        keys.into_iter().filter_map(|key| self.del(&key)).count()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Str(String),
}

// TODO: refactor this to `toResp3` and `toResp2` instead of `to_string`
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Str(s) => write!(f, "str {}", s),
        }
    }
}

#[derive(Clone)]
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
