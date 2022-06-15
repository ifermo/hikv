use crate::{HikvError, Value};

mod memory;
pub use memory::MemTable;

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
    use super::*;

    #[test]
    fn should_work_memtable_basic() {
        let store = MemTable::new();

        let v = store.set("hello", "world");
        assert!(v.unwrap().is_none());

        let v1 = store.set("hello", "world0");
        assert_eq!(v1, Ok(Some("world".into())));

        let v = store.get("hello");
        assert_eq!(v, Ok(Some("world0".into())));

        assert_eq!(Ok(None), store.get("lang"));
        assert!(store.get("language").unwrap().is_none());

        assert_eq!(store.contains("hello"), Ok(true));
        assert_eq!(store.contains("lang"), Ok(false));
        assert_eq!(store.contains("language"), Ok(false));

        let v = store.del("hello");
        assert_eq!(v, Ok(Some("world0".into())));

        assert_eq!(Ok(None), store.del("hello"));
        assert_eq!(Ok(None), store.del("lang"));
    }
}
