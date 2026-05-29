use crate::{
    ast::expr::{Ast, BinaryOp, Expr},
    error::RuntimeError,
    runtime::{environment::Env, value::Value},
};

pub fn eval(ast: Ast) -> Result<Value, RuntimeError> {
    fn eval_rec(term: &Expr, env: &mut Env) -> Result<Value, RuntimeError> {
        match term {
            // Print
            Expr::Print { value, .. } => {
                let val = eval_rec(value, env)?;
                match val {
                    Value::String(s) => println!("{s}"),
                    Value::Int(i) => println!("{i}"),
                    Value::Bool(b) => println!("{b}"),
                    _ => {
                        return Err(RuntimeError::UnsupportedType(val.val_type()));
                    }
                }
                Ok(Value::Void)
            }

            // Types
            Expr::Str { value, .. } => Ok(Value::String(value.to_owned())),
            Expr::Int { value, .. } => Ok(Value::Int(*value)),
            Expr::Bool { value, .. } => Ok(Value::Bool(*value)),
            Expr::Tuple { first, second, .. } => Ok(Value::Tuple(
                Box::new(eval_rec(first, env)?),
                Box::new(eval_rec(second, env)?),
            )),

            // Tuple functions
            Expr::First { value, .. } => {
                let val = eval_rec(value, env)?;
                match val {
                    Value::Tuple(el, _) => Ok(*el),
                    _ => Err(RuntimeError::TupleMethodOnInvalidType(
                        "first".to_owned(),
                        val.val_type(),
                    )),
                }
            }
            Expr::Second { value, .. } => {
                let val = eval_rec(value, env)?;
                match val {
                    Value::Tuple(_, el) => Ok(*el),
                    _ => Err(RuntimeError::TupleMethodOnInvalidType(
                        "second".to_owned(),
                        val.val_type(),
                    )),
                }
            }

            // Expressions
            Expr::If {
                condition,
                then,
                otherwise,
                ..
            } => {
                let cond = eval_rec(condition, env)?;
                match cond {
                    Value::Bool(b) => {
                        if b {
                            eval_rec(then, env)
                        } else {
                            eval_rec(otherwise, env)
                        }
                    }
                    _ => Err(RuntimeError::ConditionNotBoolean()),
                }
            }
            // TODO: handle errors later
            Expr::Binary { lhs, op, rhs, .. } => {
                let lhs = eval_rec(lhs, env)?;
                let rhs = eval_rec(rhs, env)?;
                match op {
                    BinaryOp::Add => match (lhs, rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l + r)),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Sub => match (lhs, rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l - r)),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Mul => match (lhs, rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l * r)),
                        _ => panic!("invalid datatype"),
                    },

                    BinaryOp::Div => match (lhs, rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l / r)),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Rem => match (lhs, rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l % r)),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Eq => match (lhs, rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l == r)),
                        (Value::String(l), Value::String(r)) => Ok(Value::Bool(l == r)),
                        (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l == r)),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Neq => match (lhs, rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l != r)),
                        (Value::String(l), Value::String(r)) => Ok(Value::Bool(l != r)),
                        (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l != r)),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Lt => match (lhs, rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l < r)),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Gt => match (lhs, rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l > r)),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Lte => match (lhs, rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l <= r)),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Gte => match (lhs, rhs) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l >= r)),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::And => match (lhs, rhs) {
                        (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l && r)),
                        _ => panic!("invalid datatype"),
                    },
                    BinaryOp::Or => match (lhs, rhs) {
                        (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l || r)),
                        _ => panic!("invalid datatype"),
                    },
                }
            }
            Expr::Var { text, .. } => Ok(env.lookup(text.clone())?),
            Expr::Let {
                name, value, next, ..
            } => {
                let mut val = eval_rec(value, env)?;
                if let Value::Closure(ref mut self_name, _, _, _) = val {
                    *self_name = Some(name.text.clone());
                }
                env.insert(name.text.clone(), val);
                eval_rec(next, env)
            }
            Expr::Function {
                parameters, value, ..
            } => Ok(Value::Closure(
                None,
                parameters.iter().map(|p| p.text.clone()).collect(),
                value.clone(),
                env.clone(),
            )),
            Expr::Call {
                callee, arguments, ..
            } => match eval_rec(callee, env)? {
                Value::Closure(self_name, parameters, body, captured_env) => {
                    if parameters.len() != arguments.len() {
                        return Err(RuntimeError::MissingParameters(
                            parameters.len(),
                            arguments.len(),
                        ));
                    }
                    let mut new_env = captured_env.clone();
                    // Enable recursion
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
                        new_env.insert(param, eval_rec(arg, env)?);
                    }
                    eval_rec(&body, &mut new_env)
                }
                _ => Err(RuntimeError::ExpectedClosure),
            },
        }
    }

    let mut env: Env = Env::new();
    eval_rec(&ast.expression, &mut env)
}
