use crate::{HikvError, Value};

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
}
