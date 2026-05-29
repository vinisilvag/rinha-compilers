use crate::{error::RuntimeError, runtime::value::Value};

#[derive(Debug, Clone)]
pub enum Binding {
    Var { name: String, value: Value },
}

#[derive(Debug)]
pub struct Env {
    items: Vec<Binding>,
}

impl Env {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn insert(&mut self, key: String, value: Value) {
        self.items.insert(0, Binding::Var { name: key, value });
    }

    pub fn lookup(&self, key: String) -> Result<Value, RuntimeError> {
        for item in &self.items {
            if let Binding::Var { name, value } = item {
                if name.to_owned() == key {
                    return Ok(value.clone());
                }
            }
        }
        Err(RuntimeError::UndefinedVariable(key))
    }
}
