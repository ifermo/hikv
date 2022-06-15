use sled::Db;
use std::{convert::TryInto, path::Path, str};

use crate::Storage;

#[derive(Debug)]
pub struct SledDb(Db);

impl SledDb {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(sled::open(path).unwrap())
    }
}

/// 把 Option> flip 成 Result, E>
fn flip<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    x.map_or(Ok(None), |v| v.map(Some))
}

impl Storage for SledDb {
    fn set(
        &self,
        key: impl Into<String>,
        value: impl Into<crate::Value>,
    ) -> Result<Option<crate::Value>, crate::HikvError> {
        let data: Vec<u8> = value.into().try_into()?;
        let ret = self
            .0
            .insert(key.into(), data)?
            .map(|v| v.as_ref().try_into());
        flip(ret)
    }

    fn get(&self, key: &str) -> Result<Option<crate::Value>, crate::HikvError> {
        let ret = self.0.get(key)?.map(|v| v.as_ref().try_into());
        flip(ret)
    }

    fn del(&self, key: &str) -> Result<Option<crate::Value>, crate::HikvError> {
        let ret = self.0.remove(key)?.map(|v| v.as_ref().try_into());
        flip(ret)
    }

    fn contains(&self, key: &str) -> Result<bool, crate::HikvError> {
        Ok(self.0.contains_key(key)?)
    }
}
