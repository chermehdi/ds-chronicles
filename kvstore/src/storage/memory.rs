use super::{Storage, StorageOptions};
use crate::Result;
use std::collections::HashMap;

pub struct InMemStorage {
    db: HashMap<String, String>,
}

// A dummy implementation of the `Storage` trait.
impl Storage for InMemStorage {
    fn open(&mut self, _dir: String, _options: StorageOptions) -> Result<()> {
        // This shouldn't do anything really, because everything is stored in memory.
        Ok(())
    }

    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.insert(key, value);
        Ok(())
    }

    fn get(&self, key: &String) -> Result<Option<&String>> {
        Ok(self.db.get(key))
    }

    fn unset(&mut self, key: &String) -> Result<Option<String>> {
        Ok(self.db.remove(key))
    }

    fn close(self) -> Result<()> {
        Ok(())
    }
}

impl InMemStorage {
    pub fn new() -> Self {
        InMemStorage { db: HashMap::new() }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_open() {
        let mut storage = InMemStorage::new();
        let can_open = storage.open(String::from("dummy"), StorageOptions {});
        assert!(can_open.is_ok());
    }

    #[test]
    fn test_set_get() {
        let mut storage = InMemStorage::new();
        let _ = storage.open(String::from("dummy"), StorageOptions {});
        let can_set = storage.set(String::from("a"), String::from("b"));
        assert!(can_set.is_ok());
        let value = storage.get(&String::from("a")).unwrap();
        assert_eq!(value.unwrap(), &String::from("b"));
    }

    #[test]
    fn test_unset() {
        let mut storage = InMemStorage::new();
        let _ = storage.open(String::from("dummy"), StorageOptions {});
        let can_set = storage.set(String::from("a"), String::from("b"));
        assert!(can_set.is_ok());
        let value = storage.unset(&String::from("a")).unwrap();
        assert_eq!(value.unwrap(), String::from("b"));

        let v = storage.get(&String::from("a")).unwrap();
        assert!(!v.is_some());
    }
}
