use crate::{CommandHandler, CommandResponse, Del, Get, HikvError, Set, Storage, Value};

impl CommandHandler for Set {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.set(self.key, self.value.unwrap_or_default()) {
            Ok(Some(v)) => v.into(),
            Ok(None) => Value::default().into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandHandler for Get {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => HikvError::NotFound(self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandHandler for Del {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.del(&self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => HikvError::NotFound(self.key).into(),
            Err(e) => e.into(),
        }
    }
}
