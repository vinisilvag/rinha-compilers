#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Int(i32),
    Bool(bool),
    Tuple(Box<Value>, Box<Value>),
    Void,
}

impl Value {
    pub fn val_type(&self) -> String {
        match self {
            Value::String(_) => "String",
            Value::Int(_) => "Int",
            Value::Bool(_) => "Bool",
            Value::Tuple(_, _) => "Tuple",
            Value::Void => "void",
        }
        .to_owned()
    }
}
