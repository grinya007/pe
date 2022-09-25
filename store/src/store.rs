use std::error::Error;

pub trait Store<K, V> {
    /// # Errors
    fn insert(&mut self, key: K, value: V) -> Result<Option<V>, Box<dyn Error>>;

    /// # Errors
    fn remove(&mut self, key: &K) -> Result<Option<V>, Box<dyn Error>>;

    /// # Errors
    fn get(&mut self, key: &K) -> Result<Option<&V>, Box<dyn Error>>;

    /// # Errors
    fn keys(&self) -> Result<Vec<K>, Box<dyn Error>>;
}
