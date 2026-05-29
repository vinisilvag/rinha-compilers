use crate::{ast::expr::Expr, runtime::environment::Env};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Int(i32),
    Bool(bool),
    Tuple(Box<Value>, Box<Value>),
    Closure(Option<String>, Vec<String>, Box<Expr>, Env),
    Void,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(_) => write!(f, "string"),
            Value::Int(_) => write!(f, "int"),
            Value::Bool(_) => write!(f, "bool"),
            Value::Tuple(e0, e1) => {
                write!(f, "({}, {})", e0, e1)
            }
            Value::Closure(_, _, _, _) => write!(f, "closure"),
            Value::Void => write!(f, "void"),
        }
    }
}
