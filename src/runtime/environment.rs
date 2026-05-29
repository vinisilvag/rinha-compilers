use crate::{error::RuntimeError, runtime::value::Value};

#[derive(Debug, Clone)]
pub struct Bind {
    name: String,
    value: Value,
}

#[derive(Debug, Clone)]
pub struct Env {
    items: Vec<Bind>,
}

impl Env {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn insert(&mut self, name: String, value: Value) {
        self.items.insert(0, Bind { name, value });
    }

    pub fn lookup(&self, key: String) -> Result<Value, RuntimeError> {
        for item in &self.items {
            if item.name == key {
                return Ok(item.value.clone());
            }
        }
        Err(RuntimeError::UndefinedVariable(key))
    }
}
