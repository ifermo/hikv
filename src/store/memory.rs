use crate::{HikvError, Storage, Value};
use dashmap::DashMap;

#[derive(Clone, Debug, Default)]
pub struct MemTable {
    innner: DashMap<String, Value>,
}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }

    // fn get_or_create(&self,name:&str)
}

impl Storage for MemTable {
    fn set(
        &self,
        key: impl Into<String>,
        value: impl Into<Value>,
    ) -> Result<Option<Value>, HikvError> {
        let old_value = self.innner.insert(key.into(), value.into());
        Ok(old_value)
    }

    fn get(&self, key: &str) -> Result<Option<Value>, HikvError> {
        let value = self.innner.get(key).map(|v| v.value().clone());
        Ok(value)
    }

    fn del(&self, key: &str) -> Result<Option<Value>, HikvError> {
        Ok(self.innner.remove(key).map(|(_k, v)| v))
    }

    fn contains(&self, key: &str) -> Result<bool, HikvError> {
        Ok(self.innner.contains_key(key))
    }
}
