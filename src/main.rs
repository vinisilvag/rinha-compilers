mod expr;
mod interp;

use clap::Parser;
use std::fs;
use std::path::PathBuf;

use crate::expr::Ast;
use crate::interp::eval;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    program_ast_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let content = fs::read_to_string(args.program_ast_path).unwrap();
    let ast: Ast = serde_json::from_str(&content).unwrap();
    eval(ast);
}
