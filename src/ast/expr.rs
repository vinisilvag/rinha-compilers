#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Ast {
    pub name: String,
    pub expression: Box<Expr>,
    pub location: Location,
}

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

impl BinaryOp {
    pub fn op_name(&self) -> String {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Rem => "%",
            BinaryOp::Eq => "==",
            BinaryOp::Neq => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Gt => ">",
            BinaryOp::Lte => "<=",
            BinaryOp::Gte => ">=",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
        }
        .to_owned()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum Expr {
    Print {
        value: Box<Expr>,
        location: Location,
    },

    Let {
        name: Parameter,
        value: Box<Expr>,
        next: Box<Expr>,
        location: Location,
    },
    Var {
        text: String,
        location: Location,
    },
    Function {
        parameters: Vec<Parameter>,
        value: Box<Expr>,
        location: Location,
    },
    Call {
        callee: Box<Expr>,
        arguments: Vec<Box<Expr>>,
        location: Location,
    },
    Binary {
        lhs: Box<Expr>,
        op: BinaryOp,
        rhs: Box<Expr>,
        location: Location,
    },
    If {
        condition: Box<Expr>,
        then: Box<Expr>,
        otherwise: Box<Expr>,
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
        first: Box<Expr>,
        second: Box<Expr>,
        location: Location,
    },

    // Tuple functions
    First {
        value: Box<Expr>,
        location: Location,
    },
    Second {
        value: Box<Expr>,
        location: Location,
    },
}
