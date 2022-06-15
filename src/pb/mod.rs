mod abi;

pub use abi::{command_request::Data, *};

use crate::HikvError;

impl CommandRequest {
    pub fn new_set(key: impl Into<String>, value: Value) -> Self {
        Self {
            data: Some(Data::Set(Set {
                key: key.into(),
                value: Some(value),
            })),
        }
    }

    pub fn new_get(key: impl Into<String>) -> Self {
        Self {
            data: Some(Data::Get(Get { key: key.into() })),
        }
    }

    pub fn new_del(key: impl Into<String>) -> Self {
        Self {
            data: Some(Data::Del(Del { key: key.into() })),
        }
    }

    pub fn new_exist(key: impl Into<String>) -> Self {
        Self {
            data: Some(Data::Exist(Exist { key: key.into() })),
        }
    }
}

impl From<HikvError> for CommandResponse {
    fn from(err: HikvError) -> Self {
        let mut ret = Self {
            status: 500,
            message: err.to_string(),
            values: vec![],
        };
        match err {
            HikvError::NotFound(_) => ret.status = 404,
            HikvError::InvalidCommand(_) => ret.status = 400,
            _ => {}
        }
        ret
    }
}

impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        Self {
            status: 200,
            message: "".into(),
            values: vec![value],
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self {
            value: Some(value::Value::Bool(b)),
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s)),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(value::Value::String(s.into())),
        }
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(i)),
        }
    }
}
