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
    text: String,
    location: Location,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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
pub enum RinhaValue {
    String(String),
    Int(i32),
    Bool(bool),
    Tuple((Box<RinhaValue>, Box<RinhaValue>)),
    Nil,
}

#[derive(Debug, Clone)]
pub enum Env {
    Var { name: Parameter, value: RinhaValue },
}

#[derive(Debug, Deserialize)]
pub struct Ast {
    pub name: String,
    pub expression: Box<Term>,
    pub location: Location,
}
