use crate::validated::Validated;

use super::value::{V8Value, V8ValueError};

pub struct V8Bool(pub(super) Validated<chromium_sys::cef_v8value_t>);

impl V8Bool {
    pub fn new(value: bool) -> Self {
        let value = match value {
            true => 1,
            false => 0,
        };

        let inner = unsafe { chromium_sys::cef_v8value_create_bool(value) };
        Self(Validated::new(inner))
    }

    pub fn get(&self) -> Result<bool, V8ValueError> {
        let inner = self.0.get()?;
        unsafe {
            let get_value = (*inner).get_bool_value.unwrap();
            Ok(get_value(inner) == 1)
        }
    }
}

impl From<V8Bool> for V8Value {
    fn from(value: V8Bool) -> Self {
        Self::Bool(value)
    }
}
