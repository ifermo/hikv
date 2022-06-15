pub mod abi;

use crate::HikvError;
use abi::{command_request::Data, *};
use bytes::Bytes;
use prost::Message;
use std::convert::TryFrom;

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

impl TryFrom<Value> for i64 {
    type Error = HikvError;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.value {
            Some(value::Value::Integer(i)) => Ok(i),
            _ => Err(HikvError::ConvertError(v, "Integer")),
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = HikvError;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.value {
            Some(value::Value::Float(f)) => Ok(f),
            _ => Err(HikvError::ConvertError(v, "Float")),
        }
    }
}

impl TryFrom<Value> for Bytes {
    type Error = HikvError;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.value {
            Some(value::Value::Binary(b)) => Ok(b),
            _ => Err(HikvError::ConvertError(v, "Binary")),
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = HikvError;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.value {
            Some(value::Value::Bool(b)) => Ok(b),
            _ => Err(HikvError::ConvertError(v, "Boolean")),
        }
    }
}

impl TryFrom<Value> for Vec<u8> {
    type Error = HikvError;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let mut buf = Vec::with_capacity(v.encoded_len());
        v.encode(&mut buf)?;
        Ok(buf)
    }
}

impl TryFrom<&[u8]> for Value {
    type Error = HikvError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let msg = Value::decode(data)?;
        Ok(msg)
    }
}

impl<const N: usize> From<&[u8; N]> for Value {
    fn from(buf: &[u8; N]) -> Self {
        Bytes::copy_from_slice(&buf[..]).into()
    }
}

impl From<Bytes> for Value {
    fn from(buf: Bytes) -> Self {
        Self {
            value: Some(value::Value::Binary(buf)),
        }
    }
}
