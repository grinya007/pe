//! The pure in-memory key-value store
use std::{collections::HashMap, error::Error, hash::Hash};

use super::store::Store;

#[derive(Default)]
pub struct StoreMem<K: Eq + Clone + Hash, V> {
    memory: HashMap<K, V>,
}

impl<K: Eq + Clone + Hash, V> StoreMem<K, V> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }
}

impl<K: Eq + Clone + Hash, V> Store<K, V> for StoreMem<K, V> {
    fn insert(&mut self, key: K, value: V) -> Result<Option<V>, Box<dyn Error>> {
        Ok(self.memory.insert(key, value))
    }

    fn remove(&mut self, key: &K) -> Result<Option<V>, Box<dyn Error>> {
        Ok(self.memory.remove(key))
    }

    fn get(&mut self, key: &K) -> Result<Option<&V>, Box<dyn Error>> {
        Ok(self.memory.get(key))
    }

    fn keys(&self) -> Result<Vec<K>, Box<dyn Error>> {
        Ok(self.memory.keys().map(|k| (*k).clone()).collect())
    }
}
