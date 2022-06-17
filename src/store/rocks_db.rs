use std::path::Path;

use rocksdb::DB;

use crate::{HikvError, Storage, Value};

pub struct RocksDb(DB);

impl RocksDb {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(DB::open_default(path).unwrap())
    }
}

#[inline]
fn to_value(val: Option<Vec<u8>>) -> Result<Option<Value>, HikvError> {
    val.map(|v| Value::try_from(&v[..]))
        .map_or(Ok(None), |v| v.map(Some))
}

impl Storage for RocksDb {
    fn set(
        &self,
        key: impl Into<String>,
        value: impl Into<crate::Value>,
    ) -> Result<Option<crate::Value>, crate::HikvError> {
        let key = key.into();

        let val = self.0.get(&key)?;
        {
            let data: Vec<u8> = value.into().try_into()?;
            self.0.put(key, data)?;
        }
        to_value(val)
    }

    fn get(&self, key: &str) -> Result<Option<crate::Value>, crate::HikvError> {
        to_value(self.0.get(key)?)
    }

    fn del(&self, key: &str) -> Result<Option<crate::Value>, crate::HikvError> {
        let val = self.0.get(key)?;
        {
            self.0.delete(key)?;
        }
        to_value(val)
    }

    fn contains(&self, key: &str) -> Result<bool, crate::HikvError> {
        Ok(self.0.key_may_exist(key))
    }
}
