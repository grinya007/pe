use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use sled::Db;
use std::{
    collections::{HashMap, VecDeque},
    env::temp_dir,
    error::Error,
    fs::remove_dir_all,
    hash::Hash,
};

use super::store::Store;

pub trait Key: Clone + Eq + Hash + Serialize + for<'a> Deserialize<'a> {}
impl<T> Key for T where T: Clone + Eq + Hash + Serialize + for<'a> Deserialize<'a> {}

pub trait Value: Serialize + for<'a> Deserialize<'a> {}
impl<T> Value for T where T: Serialize + for<'a> Deserialize<'a> {}

pub struct StoreDBBuilder {
    buffer_size: usize,
    db_path: Option<String>,
}

impl StoreDBBuilder {
    #[must_use]
    pub fn new(buffer_size: usize) -> Self {
        Self {
            buffer_size,
            db_path: None,
        }
    }

    #[must_use]
    pub fn set_db_path(self, db_path: String) -> Self {
        Self {
            db_path: Some(db_path),
            ..self
        }
    }

    /// # Errors
    pub fn build<K: Key, V: Value>(&self) -> Result<StoreDB<K, V>, Box<dyn Error>> {
        let (db_path, is_temporary) = if let Some(path) = &self.db_path {
            (path.clone(), false)
        } else {
            (
                format!(
                    "{}/sled_db_{}.d",
                    temp_dir().display(),
                    random_string::generate(16, "abcdefghijklmnopqrstuvwxyz1234567890")
                ),
                true,
            )
        };
        Ok(StoreDB {
            memory: HashMap::new(),
            buffer: VecDeque::new(),
            buffer_size: self.buffer_size,
            db_path: db_path.clone(),
            db_handle: sled::open(db_path)?,
            is_temporary,
        })
    }
}

pub struct StoreDB<K: Key, V: Value> {
    memory: HashMap<K, V>,
    buffer: VecDeque<K>,
    buffer_size: usize,
    db_path: String,
    db_handle: Db,
    is_temporary: bool,
}

impl<K: Key, V: Value> Drop for StoreDB<K, V> {
    fn drop(&mut self) {
        if !self.is_temporary {
            for (key, value) in &self.memory {
                self.db_insert(key, value).expect("Stored");
            }
        }

        self.db_handle.flush().unwrap();

        if self.is_temporary {
            remove_dir_all(&self.db_path).unwrap();
        }
    }
}

impl<K: Key, V: Value> StoreDB<K, V> {
    fn db_insert(&self, key: &K, value: &V) -> Result<(), Box<dyn Error>> {
        self.db_handle.insert(
            serialize(key).expect("Key serialized"),
            serialize(value).expect("Value serialized"),
        )?;

        Ok(())
    }

    fn move_lru(&mut self) -> Result<(), Box<dyn Error>> {
        if self.buffer.len() > self.buffer_size {
            let key = self.buffer.pop_back().expect("Least recent key popped");
            let value = self.memory.remove(&key).expect("Least recent value taken");
            self.db_insert(&key, &value)?;
        }

        Ok(())
    }
}

impl<K: Key, V: Value> Store<K, V> for StoreDB<K, V> {
    fn insert(&mut self, key: K, value: V) -> Result<Option<V>, Box<dyn Error>> {
        if let Some(old_value) = self.memory.insert(key.clone(), value) {
            Ok(Some(old_value))
        } else {
            let old_value = self
                .db_handle
                .remove(serialize(&key).expect("Key serialized"))?
                .map(|old_value_bin| deserialize(&old_value_bin).expect("Old value deserialized"));

            self.buffer.push_front(key);
            self.move_lru()?;

            Ok(old_value)
        }
    }

    fn remove(&mut self, key: &K) -> Result<Option<V>, Box<dyn Error>> {
        if let Some(old_value) = self.memory.remove(key) {
            Ok(Some(old_value))
        } else {
            Ok(self
                .db_handle
                .remove(serialize(&key).expect("Key serialized"))?
                .map(|old_value_bin| deserialize(&old_value_bin).expect("Old value deserialized")))
        }
    }

    fn get(&mut self, key: &K) -> Result<Option<&V>, Box<dyn Error>> {
        if self.memory.contains_key(key) {
            Ok(self.memory.get(key))
        } else if let Some(value_bin) = self
            .db_handle
            .remove(serialize(key).expect("Key serialized"))?
        {
            self.memory.insert(
                key.to_owned(),
                deserialize(&value_bin).expect("Old value deserialized"),
            );
            self.buffer.push_front(key.to_owned());
            self.move_lru()?;
            Ok(self.memory.get(key))
        } else {
            Ok(None)
        }
    }

    fn keys(&self) -> Result<Vec<K>, Box<dyn Error>> {
        Ok(self
            .memory
            .keys()
            .map::<Result<K, Box<dyn Error>>, _>(|k| Ok(k.clone()))
            .chain(
                self.db_handle
                    .iter()
                    .keys()
                    .map(|k| Ok(deserialize(&k?).expect("Key deserialized"))),
            )
            .map(Result::unwrap)
            .collect())
    }
}
