use crate::{CommandHandler, CommandResponse, Del, Exist, Get, HikvError, Set, Storage, Value};

impl CommandHandler for Set {
    fn handle(self, store: &impl Storage) -> CommandResponse {
        match store.set(self.key, self.value.unwrap_or_default()) {
            Ok(Some(v)) => v.into(),
            Ok(None) => Value::default().into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandHandler for Get {
    fn handle(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => HikvError::NotFound(self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandHandler for Del {
    fn handle(self, store: &impl Storage) -> CommandResponse {
        match store.del(&self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => HikvError::NotFound(self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandHandler for Exist {
    fn handle(self, store: &impl Storage) -> CommandResponse {
        match store.contains(&self.key) {
            Ok(is) => Value::from(is).into(),
            Err(e) => e.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CommandRequest, MemTable, ae::{assert_ok, assert_err}, dispatch};

    #[test]
    fn should_work_set() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_set("hello", "world".into());

        let ret = dispatch(cmd.clone(), &store);
        assert_ok(ret, &[Value::default()]);

        let ret = dispatch(cmd, &store);
        assert_ok(ret, &["world".into()]);

        let cmd = CommandRequest::new_set("age", 18.into());
        let ret = dispatch(cmd.clone(), &store);
        assert_ok(ret, &[Value::default()]);

        let ret = dispatch(cmd.clone(), &store);
        assert_ok(ret, &[18.into()]);
    }

    #[test]
    fn should_work_get() {
        let store = MemTable::new();

        let cmd = CommandRequest::new_set("lang", "rust".into());
        dispatch(cmd, &store);

        let cmd = CommandRequest::new_get("lang");
        let ret = dispatch(cmd, &store);
        assert_ok(ret, &["rust".into()])
    }

    #[test]
    fn should_work_with_non_exist_key_404() {
        let store = MemTable::new();

        let cmd = CommandRequest::new_get("language");
        let ret = dispatch(cmd, &store);
        assert_err(ret, 404, "Not Found");
    }

    #[test]
    fn should_work_del_command() {
        let store = MemTable::new();

        let cmd = CommandRequest::new_del("hello");
        let ret = dispatch(cmd, &store);
        assert_err(ret, 404, "Not Found");

        let cmd = CommandRequest::new_set("hello", "world".into());
        dispatch(cmd, &store);

        let cmd = CommandRequest::new_del("hello");
        let ret = dispatch(cmd, &store);
        assert_ok(ret, &["world".into()])
    }

    #[test]
    fn should_work_exist_command(){
        let store = MemTable::new();

        let cmd = CommandRequest::new_exist("country");
        let ret = dispatch(cmd, &store);
        assert_ok(ret, &[false.into()]);

        let cmd = CommandRequest::new_set("country", "china".into());
        dispatch(cmd, &store);

        let cmd = CommandRequest::new_exist("country");
        let ret = dispatch(cmd, &store);
        assert_ok(ret, &[true.into()]);
    }

}
