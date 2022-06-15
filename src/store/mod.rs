use crate::{HikvError, Value};

mod memory;
mod sleddb;
mod rocks_db;
pub use memory::MemTable;
pub use sleddb::SledDb;
pub use rocks_db::RocksDb;

pub trait Storage {
    /// 保存 key-value,返回 old value
    fn set(
        &self,
        key: impl Into<String>,
        value: impl Into<Value>,
    ) -> Result<Option<Value>, HikvError>;

    /// 获取指定 key 对应的 value
    fn get(&self, key: &str) -> Result<Option<Value>, HikvError>;

    /// 删除指定 key
    fn del(&self, key: &str) -> Result<Option<Value>, HikvError>;

    /// 查看指定 key 是否存在
    fn contains(&self, key: &str) -> Result<bool, HikvError>;
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn should_work_memtable_basic() {
        let store = MemTable::new();
        test_basic_interface(store);
    }

    #[test]
    fn should_work_sleddb_basic() {
        let dir = tempdir().unwrap();
        let store = SledDb::new(dir);
        test_basic_interface(store);
    }

    fn test_basic_interface(store: impl Storage) {
        let v = store.set("hello", "world");
        assert!(v.unwrap().is_none());

        let v1 = store.set("hello", "world0");
        assert_eq!(v1.unwrap(), Some("world".into()));

        let v = store.get("hello");
        assert_eq!(v.unwrap(), Some("world0".into()));

        assert_eq!(None, store.get("lang").unwrap());
        assert!(store.get("language").unwrap().is_none());

        assert_eq!(store.contains("hello").unwrap(), true);
        assert_eq!(store.contains("lang").unwrap(), false);
        assert_eq!(store.contains("language").unwrap(), false);

        let v = store.del("hello");
        assert_eq!(v.unwrap(), Some("world0".into()));

        assert_eq!(None, store.del("hello").unwrap());
        assert_eq!(None, store.del("lang").unwrap());
    }
}
