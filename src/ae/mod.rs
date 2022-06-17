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
    on_received: Vec<fn(&CommandRequest)>,
    on_executed: Vec<fn(&CommandResponse)>,
    on_before_reply: Vec<fn(&mut CommandResponse)>,
    on_after_reply: Vec<fn()>,
}

impl<Store: Storage> ServiceInner<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            store,
            on_received: Vec::new(),
            on_executed: Vec::new(),
            on_before_reply: Vec::new(),
            on_after_reply: Vec::new(),
        }
    }

    pub fn fn_received(mut self, f: fn(&CommandRequest)) -> Self {
        self.on_received.push(f);
        self
    }

    pub fn fn_executed(mut self, f: fn(&CommandResponse)) -> Self {
        self.on_executed.push(f);
        self
    }

    pub fn fn_before_reply(mut self, f: fn(&mut CommandResponse)) -> Self {
        self.on_before_reply.push(f);
        self
    }

    pub fn fn_after_reply(mut self, f: fn()) -> Self {
        self.on_after_reply.push(f);
        self
    }
}

impl<Store: Storage> Service<Store> {
    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        debug!("Got request: {:?}", cmd);
        self.inner.on_received.notify(&cmd);
        let mut ret = dispatch(cmd, &self.inner.store);
        debug!("Executed response: {:?}", ret);
        self.inner.on_executed.notify(&ret);

        self.inner.on_before_reply.notify(&mut ret);
        if !self.inner.on_before_reply.is_empty() {
            debug!("Modified response: {:?}", ret);
        }

        ret
    }
}

impl<Store: Storage> From<ServiceInner<Store>> for Service<Store> {
    fn from(inner: ServiceInner<Store>) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

/// Immutable event notification
pub trait Notify<Arg> {
    fn notify(&self, arg: &Arg);
}
impl<Arg> Notify<Arg> for Vec<fn(&Arg)> {
    #[inline]
    fn notify(&self, arg: &Arg) {
        for f in self {
            f(arg);
        }
    }
}

/// mutable event notification
pub trait NotifyMut<Arg> {
    fn notify(&self, arg: &mut Arg);
}
impl<Arg> NotifyMut<Arg> for Vec<fn(&mut Arg)> {
    #[inline]
    fn notify(&self, arg: &mut Arg) {
        for f in self {
            f(arg);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use tracing::info;

    use super::*;
    use crate::{MemTable, Value};

    #[test]
    fn should_work_service() {
        let service: Service = ServiceInner::new(MemTable::new()).into();

        let cloned = service.clone();

        let handle = thread::spawn(move || {
            let ret = cloned.execute(CommandRequest::new_set("name", "tom".into()));
            assert_ok(ret, &[Value::default()]);
        });
        handle.join().unwrap();

        let ret = service.execute(CommandRequest::new_get("name"));
        assert_ok(ret, &["tom".into()]);
    }

    #[test]
    fn should_work_event_register() {
        fn received0(cmd: &CommandRequest) {
            info!("Got: {:?}", cmd);
        }
        fn executed0(resp: &CommandResponse) {
            info!("{:?}", resp);
        }
        fn before_reply0(resp: &mut CommandResponse) {
            resp.status = 201;
        }
        fn after_reply0() {
            info!("Data is sent")
        }
        let service: Service = ServiceInner::new(MemTable::default())
            .fn_received(|_: &CommandRequest| {})
            .fn_received(received0)
            .fn_executed(executed0)
            .fn_before_reply(before_reply0)
            .fn_after_reply(after_reply0)
            .into();

        let ret = service.execute(CommandRequest::new_set("k1", "v1".into()));
        assert_eq!(ret.status, 201);
        assert_eq!(ret.message, "");
        assert_eq!(ret.values, vec![Value::default()]);
    }
}

#[cfg(test)]
use crate::Value;

#[cfg(test)]
pub fn assert_ok(ret: CommandResponse, values: &[Value]) {
    assert_eq!(ret.status, 200);
    assert_eq!(ret.message, "");
    assert_eq!(ret.values, values)
}

#[cfg(test)]
pub fn assert_err(ret: CommandResponse, code: u32, msg: &str) {
    assert_eq!(ret.status, code);
    assert!(ret.message.contains(msg));
    assert_eq!(ret.values, &[])
}
