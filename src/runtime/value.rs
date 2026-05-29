use crate::{ast::expr::Expr, runtime::environment::Env};

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Int(i32),
    Bool(bool),
    Tuple(Box<Value>, Box<Value>),
    Closure(Option<String>, Vec<String>, Box<Expr>, Env),
    Void,
}

impl Value {
    pub fn val_type(&self) -> String {
        match self {
            Value::String(_) => "string",
            Value::Int(_) => "int",
            Value::Bool(_) => "bool",
            Value::Tuple(_, _) => "tuple",
            Value::Closure(_, _, _, _) => "<#closure>",
            Value::Void => "void",
        }
        .to_owned()
    }
}
