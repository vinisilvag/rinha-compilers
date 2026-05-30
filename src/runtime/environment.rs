use crate::{error::RuntimeError, runtime::value::Value};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Env {
    parent: Option<Rc<Env>>,
    bindings: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            parent: None,
            bindings: HashMap::new(),
        }
    }

    pub fn extend(parent: Rc<Env>) -> Self {
        Self {
            parent: Some(parent),
            bindings: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    pub fn lookup(&self, key: &str) -> Result<Value, RuntimeError> {
        if let Some(val) = self.bindings.get(key) {
            return Ok(val.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.lookup(key);
        }
        Err(RuntimeError::UndefinedBind(key.to_string()))
    }
}
