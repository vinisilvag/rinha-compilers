use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct Location {
    start: i32,
    end: i32,
    filename: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Print {
        value: Box<Term>,
        location: Location,
    },

    Str {
        value: String,
        location: Location,
    },
}

pub enum InterpValue {
    String(String),
    Int(i32),
    Bool(bool),
    Nil,
}

#[derive(Debug, Deserialize)]
pub struct Ast {
    pub name: String,
    pub expression: Box<Term>,
    pub location: Location,
}
