use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("could not print a value of type {0}")]
    UnsupportedType(String),

    #[error("tuple method {0} was called on an invalid type {1}")]
    TupleMethodOnInvalidType(String, String),

    #[error("if condition expression was not of type Bool")]
    ConditionNotBoolean(),

    #[error("expected {0} and {1} for {2} operation, found {3} and {4}")]
    UnexpectedType(String, String, String, String, String),

    #[error("no definition for variable {0} was found")]
    UndefinedVariable(String),
}

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("parser error: {0}")]
    Parser(#[from] ParserError),

    #[error("interpreter error: {0}")]
    Interpreter(#[from] RuntimeError),
}
