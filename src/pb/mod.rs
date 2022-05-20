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

    pub fn del(key: impl Into<String>) -> Self {
        Self {
            data: Some(Data::Del(Del { key: key.into() })),
        }
    }
}

impl From<HikvError> for CommandResponse {
    fn from(_: HikvError) -> Self {
        todo!()
    }
}

impl From<Value> for CommandResponse {
    fn from(_: Value) -> Self {
        todo!()
    }
}
