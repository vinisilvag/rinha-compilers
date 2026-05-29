use crate::expr::{Ast, BinaryOp, Env, RinhaVal, Term};

pub fn eval(ast: Ast) {
    fn eval_rec(term: &Term, env: &mut Env) -> RinhaVal {
        match term {
            // Print
            Term::Print { value, .. } => {
                let val = eval_rec(&*value, env);
                match val {
                    RinhaVal::String(s) => println!("{s}"),
                    RinhaVal::Int(i) => println!("{i}"),
                    RinhaVal::Bool(b) => println!("{b}"),
                    RinhaVal::Void => panic!("should not print void (I guess)"),
                    _ => unimplemented!("a print method for {:?} is not implemented yet", val),
                }
                RinhaVal::Void
            }

            // Types
            Term::Str { value, .. } => RinhaVal::String(value.to_owned()),
            Term::Int { value, .. } => RinhaVal::Int(*value),
            Term::Bool { value, .. } => RinhaVal::Bool(*value),
            Term::Tuple { first, second, .. } => RinhaVal::Tuple((
                Box::new(eval_rec(&*first, env)),
                Box::new(eval_rec(&*second, env)),
            )),

            // Tuple functions
            Term::First { value, .. } => {
                let val = eval_rec(&*value, env);
                match val {
                    RinhaVal::Tuple(t) => *t.0,
                    _ => panic!("first called without a tuple"),
                }
            }
            Term::Second { value, .. } => {
                let val = eval_rec(&*value, env);
                match val {
                    RinhaVal::Tuple(t) => *t.1,
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
                    RinhaVal::Bool(b) => {
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
                        (RinhaVal::Int(l), RinhaVal::Int(r)) => RinhaVal::Int(l + r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Sub => match (lhs_eval, rhs_eval) {
                        (RinhaVal::Int(l), RinhaVal::Int(r)) => RinhaVal::Int(l - r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Mul => match (lhs_eval, rhs_eval) {
                        (RinhaVal::Int(l), RinhaVal::Int(r)) => RinhaVal::Int(l * r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Div => match (lhs_eval, rhs_eval) {
                        (RinhaVal::Int(l), RinhaVal::Int(r)) => RinhaVal::Int(l / r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Rem => match (lhs_eval, rhs_eval) {
                        (RinhaVal::Int(l), RinhaVal::Int(r)) => RinhaVal::Int(l % r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Eq => match (lhs_eval, rhs_eval) {
                        (RinhaVal::Int(l), RinhaVal::Int(r)) => RinhaVal::Bool(l == r),
                        (RinhaVal::String(l), RinhaVal::String(r)) => RinhaVal::Bool(l == r),
                        (RinhaVal::Bool(l), RinhaVal::Bool(r)) => RinhaVal::Bool(l == r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Neq => match (lhs_eval, rhs_eval) {
                        (RinhaVal::Int(l), RinhaVal::Int(r)) => RinhaVal::Bool(l != r),
                        (RinhaVal::String(l), RinhaVal::String(r)) => RinhaVal::Bool(l != r),
                        (RinhaVal::Bool(l), RinhaVal::Bool(r)) => RinhaVal::Bool(l != r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Lt => match (lhs_eval, rhs_eval) {
                        (RinhaVal::Int(l), RinhaVal::Int(r)) => RinhaVal::Bool(l < r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Gt => match (lhs_eval, rhs_eval) {
                        (RinhaVal::Int(l), RinhaVal::Int(r)) => RinhaVal::Bool(l > r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Lte => match (lhs_eval, rhs_eval) {
                        (RinhaVal::Int(l), RinhaVal::Int(r)) => RinhaVal::Bool(l <= r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Gte => match (lhs_eval, rhs_eval) {
                        (RinhaVal::Int(l), RinhaVal::Int(r)) => RinhaVal::Bool(l >= r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::And => match (lhs_eval, rhs_eval) {
                        (RinhaVal::Bool(l), RinhaVal::Bool(r)) => RinhaVal::Bool(l && r),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Or => match (lhs_eval, rhs_eval) {
                        (RinhaVal::Bool(l), RinhaVal::Bool(r)) => RinhaVal::Bool(l || r),
                        _ => panic!("invalid datatype"),
                    },
                }
            }
            Term::Var { text, .. } => env.lookup(text.clone()),
            Term::Let {
                name, value, next, ..
            } => {
                let val = eval_rec(value, env);
                env.insert(name.text.clone(), val);
                eval_rec(next, env)
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
    let mut env: Env = Env::new();
    eval_rec(&*ast.expression, &mut env);
}
