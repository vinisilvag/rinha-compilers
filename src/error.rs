use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("could not print a value of type '{0}'")]
    UnsupportedType(String),

    #[error("tuple method {0} was called on an invalid type '{1}'")]
    TupleMethodOnInvalidType(String, String),

    #[error("if condition expression was not of type Bool")]
    ConditionNotBoolean(),

    // TODO: better errors
    #[error("expected a closure, found ..")]
    ExpectedClosure,

    #[error("expected '{0}' parameters on function call, found '{1}'")]
    MissingParameters(usize, usize),

    #[error("no definition for '{0}' was found")]
    UndefinedVariable(String),
}

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("parser error: {0}")]
    Parser(#[from] ParserError),

    #[error("runtime error: {0}")]
    Interpreter(#[from] RuntimeError),
}
