mod handler;

pub use handler::*;

use crate::command_request::Data;
use crate::{CommandRequest, CommandResponse, HikvError, Storage};

pub trait CommandHandler {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

pub fn dispatch(req: CommandRequest, store: &impl Storage) -> CommandResponse {
    match req.data {
        Some(Data::Get(param)) => param.execute(store),
        Some(Data::Set(param)) => param.execute(store),
        Some(Data::Del(param)) => param.execute(store),
        _ => HikvError::Internal("Not implemented".into()).into(),
    }
}
