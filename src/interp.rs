use crate::expr::{Ast, InterpValue, Term};

pub fn eval(ast: Ast) {
    fn eval_rec(term: &Term) -> InterpValue {
        match term {
            Term::Print { value, .. } => {
                let val = eval_rec(&*value);
                match val {
                    InterpValue::String(s) => println!("{}", s),
                    InterpValue::Int(i) => println!("{}", i),
                    InterpValue::Bool(b) => println!("{}", b),
                    InterpValue::Nil => panic!("should not print nil (I guess)"),
                }
                InterpValue::Nil
            }
            Term::Str { value, .. } => InterpValue::String(value.to_owned()),
            _ => !unimplemented!(),
        }
    }

    println!("eval of: {:?}", ast.name);
    eval_rec(&*ast.expression);
}
