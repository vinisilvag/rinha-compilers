#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Location {
    start: i32,
    end: i32,
    filename: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Parameter {
    pub text: String,
    location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Print {
        value: Box<Term>,
        location: Location,
    },

    Let {
        name: Parameter,
        value: Box<Term>,
        next: Box<Term>,
        location: Location,
    },
    Var {
        text: String,
        location: Location,
    },
    Function {
        parameters: Vec<Parameter>,
        value: Box<Term>,
        location: Location,
    },
    Call {
        callee: Box<Term>,
        arguments: Vec<Box<Term>>,
        location: Location,
    },
    Binary {
        lhs: Box<Term>,
        op: BinaryOp,
        rhs: Box<Term>,
        location: Location,
    },
    If {
        condition: Box<Term>,
        then: Box<Term>,
        otherwise: Box<Term>,
        location: Location,
    },

    // Types
    Str {
        value: String,
        location: Location,
    },
    Int {
        value: i32,
        location: Location,
    },
    Bool {
        value: bool,
        location: Location,
    },
    Tuple {
        first: Box<Term>,
        second: Box<Term>,
        location: Location,
    },

    // Tuple functions
    First {
        value: Box<Term>,
        location: Location,
    },
    Second {
        value: Box<Term>,
        location: Location,
    },
}

#[derive(Debug, Clone)]
pub enum RinhaVal {
    String(String),
    Int(i32),
    Bool(bool),
    Tuple((Box<RinhaVal>, Box<RinhaVal>)),
    Void,
}

#[derive(Debug, Clone)]
pub enum Binding {
    Var { name: String, value: RinhaVal },
}

#[derive(Debug)]
pub struct Env {
    items: Vec<Binding>,
}

impl Env {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn insert(&mut self, key: String, value: RinhaVal) {
        self.items.insert(0, Binding::Var { name: key, value });
    }

    pub fn lookup(&self, key: String) -> RinhaVal {
        for item in self.items.clone() {
            match item {
                Binding::Var { name, value } => {
                    if name == key {
                        return value;
                    }
                }
                _ => continue,
            }
        }
        panic!("could not found a definition for {}", key);
    }
}

#[derive(Debug, Deserialize)]
pub struct Ast {
    pub name: String,
    pub expression: Box<Term>,
    pub location: Location,
}
