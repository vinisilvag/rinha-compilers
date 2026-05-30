use std::rc::Rc;

use crate::{
    ast::expr::{Ast, BinaryOp, Expr},
    error::RuntimeError,
    runtime::{environment::Env, value::Value},
};

pub fn eval(ast: Ast) -> Result<Value, RuntimeError> {
    fn eval_rec(term: &Expr, env: Rc<Env>) -> Result<Value, RuntimeError> {
        match term {
            Expr::Print { value, .. } => {
                let val = eval_rec(value, env)?;
                match val {
                    Value::String(s) => println!("{s}"),
                    Value::Int(i) => println!("{i}"),
                    Value::Bool(b) => println!("{b}"),
                    Value::Tuple(e0, e1) => println!("({e0}, {e1})"),
                    Value::Closure(_, _, _, _) => println!("<#closure>"),
                    _ => {
                        return Err(RuntimeError::NonPrintableValue(val));
                    }
                }
                Ok(Value::Void)
            }

            Expr::Str { value, .. } => Ok(Value::String(value.to_owned())),
            Expr::Int { value, .. } => Ok(Value::Int(*value)),
            Expr::Bool { value, .. } => Ok(Value::Bool(*value)),
            Expr::Tuple { first, second, .. } => Ok(Value::Tuple(
                Box::new(eval_rec(first, Rc::clone(&env))?),
                Box::new(eval_rec(second, Rc::clone(&env))?),
            )),
            Expr::First { value, .. } => {
                let val = eval_rec(value, env)?;
                match val {
                    Value::Tuple(el, _) => Ok(*el),
                    _ => Err(RuntimeError::InvalidTupleAccess {
                        method: "first".to_owned(),
                        found: val,
                    }),
                }
            }
            Expr::Second { value, .. } => {
                let val = eval_rec(value, env)?;
                match val {
                    Value::Tuple(_, el) => Ok(*el),
                    _ => Err(RuntimeError::InvalidTupleAccess {
                        method: "second".to_owned(),
                        found: val,
                    }),
                }
            }

            Expr::If {
                condition,
                then,
                otherwise,
                ..
            } => {
                let cond = eval_rec(condition, Rc::clone(&env))?;
                match cond {
                    Value::Bool(b) => {
                        if b {
                            eval_rec(then, Rc::clone(&env))
                        } else {
                            eval_rec(otherwise, Rc::clone(&env))
                        }
                    }
                    _ => Err(RuntimeError::InvalidConditionType(cond)),
                }
            }
            Expr::Binary { lhs, op, rhs, .. } => {
                let lhs = eval_rec(lhs, Rc::clone(&env))?;
                let rhs = eval_rec(rhs, Rc::clone(&env))?;
                match op {
                    BinaryOp::Add => match (&lhs, &rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l + r)),
                        // Syntax sugar
                        (Value::String(l), Value::Int(r)) => {
                            Ok(Value::String(l.to_owned() + &r.to_string()))
                        }
                        (Value::Int(l), Value::String(r)) => Ok(Value::String(l.to_string() + r)),
                        // String concatenation
                        (Value::String(l), Value::String(r)) => Ok(Value::String(l.to_owned() + r)),
                        _ => Err(RuntimeError::InvalidBinaryOperands {
                            op: op.clone(),
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }),
                    },
                    BinaryOp::Sub => match (&lhs, &rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l - r)),
                        _ => Err(RuntimeError::InvalidBinaryOperands {
                            op: op.clone(),
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }),
                    },
                    BinaryOp::Mul => match (&lhs, &rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l * r)),
                        _ => Err(RuntimeError::InvalidBinaryOperands {
                            op: op.clone(),
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }),
                    },

                    BinaryOp::Div => match (&lhs, &rhs) {
                        (Value::Int(_), Value::Int(0)) => Err(RuntimeError::DivisionByZero),
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l / r)),
                        _ => Err(RuntimeError::InvalidBinaryOperands {
                            op: op.clone(),
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }),
                    },
                    BinaryOp::Rem => match (&lhs, &rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l % r)),
                        _ => Err(RuntimeError::InvalidBinaryOperands {
                            op: op.clone(),
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }),
                    },
                    BinaryOp::Eq => match (&lhs, &rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l == r)),
                        (Value::String(l), Value::String(r)) => Ok(Value::Bool(l == r)),
                        (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l == r)),
                        _ => Err(RuntimeError::InvalidBinaryOperands {
                            op: op.clone(),
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }),
                    },
                    BinaryOp::Neq => match (&lhs, &rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l != r)),
                        (Value::String(l), Value::String(r)) => Ok(Value::Bool(l != r)),
                        (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l != r)),
                        _ => Err(RuntimeError::InvalidBinaryOperands {
                            op: op.clone(),
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }),
                    },
                    BinaryOp::Lt => match (&lhs, &rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l < r)),
                        _ => Err(RuntimeError::InvalidBinaryOperands {
                            op: op.clone(),
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }),
                    },
                    BinaryOp::Gt => match (&lhs, &rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l > r)),
                        _ => Err(RuntimeError::InvalidBinaryOperands {
                            op: op.clone(),
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }),
                    },
                    BinaryOp::Lte => match (&lhs, &rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l <= r)),
                        _ => Err(RuntimeError::InvalidBinaryOperands {
                            op: op.clone(),
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }),
                    },
                    BinaryOp::Gte => match (&lhs, &rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l >= r)),
                        _ => Err(RuntimeError::InvalidBinaryOperands {
                            op: op.clone(),
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }),
                    },
                    BinaryOp::And => match (&lhs, &rhs) {
                        (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(*l && *r)),
                        _ => Err(RuntimeError::InvalidBinaryOperands {
                            op: op.clone(),
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }),
                    },
                    BinaryOp::Or => match (&lhs, &rhs) {
                        (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(*l || *r)),
                        _ => Err(RuntimeError::InvalidBinaryOperands {
                            op: op.clone(),
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }),
                    },
                }
            }
            Expr::Var { text, .. } => Ok(env.lookup(text)?),
            Expr::Let {
                name, value, next, ..
            } => {
                let mut val = eval_rec(value, Rc::clone(&env))?;
                if let Value::Closure(ref mut self_name, _, _, _) = val {
                    *self_name = Some(name.text.clone());
                }
                let mut new_env = Env::extend(Rc::clone(&env));
                new_env.insert(name.text.clone(), val);
                eval_rec(next, Rc::new(new_env))
            }
            Expr::Function {
                parameters, value, ..
            } => Ok(Value::Closure(
                None,
                parameters.iter().map(|p| p.text.clone()).collect(),
                value.clone(),
                Rc::clone(&env),
            )),
            Expr::Call {
                callee, arguments, ..
            } => {
                let callee_val = eval_rec(callee, Rc::clone(&env))?;
                match callee_val {
                    Value::Closure(self_name, parameters, body, captured_env) => {
                        if parameters.len() != arguments.len() {
                            return Err(RuntimeError::InvalidArgumentCount {
                                expected: parameters.len(),
                                found: arguments.len(),
                            });
                        }
                        let mut new_env = Env::extend(Rc::clone(&captured_env));
                        if let Some(name) = self_name.clone() {
                            let self_closure = Value::Closure(
                                self_name,
                                parameters.clone(),
                                body.clone(),
                                captured_env,
                            );
                            new_env.insert(name, self_closure);
                        }
                        for (param, arg) in parameters.into_iter().zip(arguments) {
                            let arg_val = eval_rec(arg, Rc::clone(&env))?;
                            new_env.insert(param, arg_val);
                        }
                        eval_rec(&body, Rc::new(new_env))
                    }
                    _ => Err(RuntimeError::NonCallableValue(callee_val)),
                }
            }
        }
    }

    eval_rec(&ast.expression, Rc::new(Env::new()))
}
