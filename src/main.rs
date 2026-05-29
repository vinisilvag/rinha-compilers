mod ast;
mod error;
mod runtime;

use clap::Parser;
use std::fs;
use std::path::PathBuf;

use crate::ast::expr::Ast;
use crate::error::InterpreterError;
use crate::runtime::evaluator::eval;
use crate::runtime::value::Value;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    program_ast_path: PathBuf,
}

fn run() -> Result<Value, InterpreterError> {
    let program = fs::read_to_string(Args::parse().program_ast_path)?;
    let ast: Ast = serde_json::from_str(&program)?;
    Ok(eval(ast)?)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
