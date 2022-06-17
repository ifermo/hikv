use std::path::Path;

use rocksdb::DB;

use crate::{HikvError, Storage, Value};

pub struct RocksDb(DB);

impl RocksDb {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(DB::open_default(path).unwrap())
    }
}

/// 把 Result<Option<Vec<u8>>, rocksdb::Error> convert 成 Result<Option<Value>, HikvError>
fn convert(x: Result<Option<Vec<u8>>, rocksdb::Error>) -> Result<Option<Value>, HikvError> {
    x.map_err(|e| e.into())
        .map(|v| v.map(|v| Value::try_from(&v[..]).unwrap()))
}

impl Storage for RocksDb {
    fn set(
        &self,
        key: impl Into<String>,
        value: impl Into<crate::Value>,
    ) -> Result<Option<crate::Value>, crate::HikvError> {
        let key = key.into();
        let ret = self.0.get(key.clone());
        if ret.is_ok() {
            let data: Vec<u8> = value.into().try_into()?;
            if let Err(e) = self.0.put(key, data) {
                return Err(e.into());
            }
        }
        return convert(ret);
    }

    fn get(&self, key: &str) -> Result<Option<crate::Value>, crate::HikvError> {
        let ret = self.0.get(key.clone());
        return convert(ret);
    }

    fn del(&self, key: &str) -> Result<Option<crate::Value>, crate::HikvError> {
        let ret = self.0.get(key);
        if ret.is_ok() {
            if let Err(e) = self.0.delete(key) {
                return Err(e.into());
            }
        }
        return convert(ret);
    }

    fn contains(&self, key: &str) -> Result<bool, crate::HikvError> {
        Ok(self.0.key_may_exist(key))
    }
}
