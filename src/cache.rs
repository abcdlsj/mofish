use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub trait Cache {
    fn get(&self, key: &str) -> Result<String>;
    fn set(&self, key: &str, value: &str) -> Result<()>;
}

pub struct LocalCache {
    pub cache: HashMap<String, String>,
}

impl LocalCache {
    pub fn new() -> LocalCache {
        LocalCache {
            cache: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Result<String> {
        match self.cache.get(key) {
            Some(v) => Ok(v.to_string()),
            None => Err(anyhow!("key not found")),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        self.cache.insert(key.to_string(), value.to_string());
        Ok(())
    }
}