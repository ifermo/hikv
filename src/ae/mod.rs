mod handler;

use std::sync::Arc;

pub use handler::*;
use tracing::debug;

use crate::command_request::Data;
use crate::{CommandRequest, CommandResponse, HikvError, MemTable, Storage};

/// 对 Command 的处理抽象
pub trait CommandHandler {
    /// 处理 Command，返回 Response
    fn handle(self, store: &impl Storage) -> CommandResponse;
}

pub fn dispatch(req: CommandRequest, store: &impl Storage) -> CommandResponse {
    match req.data {
        Some(Data::Get(param)) => param.handle(store),
        Some(Data::Set(param)) => param.handle(store),
        Some(Data::Del(param)) => param.handle(store),
        Some(Data::Exist(param)) => param.handle(store),
        None => HikvError::InvalidCommand("request has not data".into()).into(),
        // _ => HikvError::Internal("Not implemented".into()).into(),
    }
}

pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

pub struct ServiceInner<Store> {
    store: Store,
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store }),
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        debug!("Got request: {:?}", cmd);
        // TODO: on_received 事件
        let ret = dispatch(cmd, &self.inner.store);
        debug!("Executed response: {:?}", ret);
        // TODO: on_executed 事件
        ret
    }
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;
    use crate::{MemTable, Value};

    #[test]
    fn should_work_service() {
        let service = Service::new(MemTable::default());

        let cloned = service.clone();

        let handle = thread::spawn(move || {
            let ret = cloned.execute(CommandRequest::new_set("name", "tom".into()));
            assert_ok(ret, &[Value::default()]);
        });
        handle.join().unwrap();

        let ret = service.execute(CommandRequest::new_get("name"));
        assert_ok(ret, &["tom".into()]);
    }
}

#[cfg(test)]
use crate::Value;

#[cfg(test)]
fn assert_ok(ret: CommandResponse, values: &[Value]) {
    assert_eq!(ret.status, 200);
    assert_eq!(ret.message, "");
    assert_eq!(ret.values, values)
}

#[cfg(test)]
fn assert_err(ret: CommandResponse, code: u32, msg: &str) {
    assert_eq!(ret.status, code);
    assert!(ret.message.contains(msg));
    assert_eq!(ret.values, &[])
}
