use crate::expr::{Ast, BinaryOp, Env, RinhaValue, Term};

pub fn eval(ast: Ast) {
    fn eval_rec(term: &Term, env: &Vec<Env>) -> RinhaValue {
        match term {
            // Print
            Term::Print { value, .. } => {
                let val = eval_rec(&*value, env);
                match val {
                    RinhaValue::String(s) => println!("{}", s),
                    RinhaValue::Int(i) => println!("{}", i),
                    RinhaValue::Bool(b) => println!("{}", b),
                    RinhaValue::Nil => panic!("should not print nil (I guess)"),
                    _ => unimplemented!("a print method for {:?} is not implemented yet", val),
                }
                RinhaValue::Nil
            }

            // Types
            Term::Str { value, .. } => RinhaValue::String(value.to_owned()),
            Term::Int { value, .. } => RinhaValue::Int(*value),
            Term::Bool { value, .. } => RinhaValue::Bool(*value),
            Term::Tuple { first, second, .. } => RinhaValue::Tuple((
                Box::new(eval_rec(&*first, env)),
                Box::new(eval_rec(&*second, env)),
            )),

            // Tuple functions
            Term::First { value, .. } => {
                let val = eval_rec(&*value, env);
                match val {
                    RinhaValue::Tuple(t) => *t.0,
                    _ => panic!("first called without a tuple"),
                }
            }
            Term::Second { value, .. } => {
                let val = eval_rec(&*value, env);
                match val {
                    RinhaValue::Tuple(t) => *t.1,
                    _ => panic!("second called without a tuple"),
                }
            }

            // Expressions
            Term::If {
                condition,
                then,
                otherwise,
                ..
            } => {
                let cond = eval_rec(&*condition, env);
                match cond {
                    RinhaValue::Bool(b) => {
                        if b {
                            eval_rec(&*then, env)
                        } else {
                            eval_rec(&*otherwise, env)
                        }
                    }
                    _ => panic!("if condition should be a boolean"),
                }
            }
            Term::Binary { lhs, op, rhs, .. } => {
                let lhs_eval = eval_rec(&*lhs, env);
                let rhs_eval = eval_rec(&*rhs, env);
                match op {
                    BinaryOp::Add => match (lhs_eval, rhs_eval) {
                        (RinhaValue::Int(l), RinhaValue::Int(r)) => RinhaValue::Int(l + r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Sub => match (lhs_eval, rhs_eval) {
                        (RinhaValue::Int(l), RinhaValue::Int(r)) => RinhaValue::Int(l - r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Mul => match (lhs_eval, rhs_eval) {
                        (RinhaValue::Int(l), RinhaValue::Int(r)) => RinhaValue::Int(l * r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Div => match (lhs_eval, rhs_eval) {
                        (RinhaValue::Int(l), RinhaValue::Int(r)) => RinhaValue::Int(l / r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Rem => match (lhs_eval, rhs_eval) {
                        (RinhaValue::Int(l), RinhaValue::Int(r)) => RinhaValue::Int(l % r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Eq => match (lhs_eval, rhs_eval) {
                        (RinhaValue::Int(l), RinhaValue::Int(r)) => RinhaValue::Bool(l == r),
                        (RinhaValue::String(l), RinhaValue::String(r)) => RinhaValue::Bool(l == r),
                        (RinhaValue::Bool(l), RinhaValue::Bool(r)) => RinhaValue::Bool(l == r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Neq => match (lhs_eval, rhs_eval) {
                        (RinhaValue::Int(l), RinhaValue::Int(r)) => RinhaValue::Bool(l != r),
                        (RinhaValue::String(l), RinhaValue::String(r)) => RinhaValue::Bool(l != r),
                        (RinhaValue::Bool(l), RinhaValue::Bool(r)) => RinhaValue::Bool(l != r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Lt => match (lhs_eval, rhs_eval) {
                        (RinhaValue::Int(l), RinhaValue::Int(r)) => RinhaValue::Bool(l < r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Gt => match (lhs_eval, rhs_eval) {
                        (RinhaValue::Int(l), RinhaValue::Int(r)) => RinhaValue::Bool(l > r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Lte => match (lhs_eval, rhs_eval) {
                        (RinhaValue::Int(l), RinhaValue::Int(r)) => RinhaValue::Bool(l <= r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Gte => match (lhs_eval, rhs_eval) {
                        (RinhaValue::Int(l), RinhaValue::Int(r)) => RinhaValue::Bool(l >= r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::And => match (lhs_eval, rhs_eval) {
                        (RinhaValue::Bool(l), RinhaValue::Bool(r)) => RinhaValue::Bool(l && r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Or => match (lhs_eval, rhs_eval) {
                        (RinhaValue::Bool(l), RinhaValue::Bool(r)) => RinhaValue::Bool(l || r),
                        _ => panic!("invalid datatype"),
                    },
                }
            }
            Term::Var { text, .. } => {
                unimplemented!("var")
            }
            Term::Let {
                name, value, next, ..
            } => {
                unimplemented!("let")
            }
            Term::Function {
                parameters, value, ..
            } => {
                unimplemented!("function")
            }
            Term::Call {
                callee, arguments, ..
            } => {
                unimplemented!("call")
            }
        }
    }

    println!("eval of: {:?}", ast.name);
    let mut env: Vec<Env> = Vec::new();
    eval_rec(&*ast.expression, &env);
}
