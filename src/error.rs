use thiserror::Error;

use crate::{ast::expr::BinaryOp, runtime::value::Value};

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("cannot print value of type `{0}`")]
    NonPrintableValue(Value),

    #[error("tuple method `{method}` cannot be called on value of type `{found}`")]
    InvalidTupleAccess { method: String, found: Value },

    #[error("expected `bool` in if condition, found `{0}`")]
    InvalidConditionType(Value),

    #[error("invalid operands for `{op}`: left is `{lhs}`, right is `{rhs}`")]
    InvalidBinaryOperands {
        op: BinaryOp,
        lhs: Box<Value>,
        rhs: Box<Value>,
    },

    #[error("attempted to call a non-function value of type `{0}`")]
    NonCallableValue(Value),

    #[error("function expected {expected} argument(s), but received {found}")]
    InvalidArgumentCount { expected: usize, found: usize },

    #[error("undefined bind for `{0}` (variable or function)")]
    UndefinedBind(String),

    #[error("division by zero")]
    DivisionByZero,
}

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Parser(#[from] serde_json::Error),

    #[error("runtime error: {0}")]
    Runtime(#[from] RuntimeError),
}
