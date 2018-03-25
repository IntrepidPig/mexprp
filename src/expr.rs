use std::collections::HashMap;

use op::Op;
use opers::*;
use parse::*;

use failure::Error;
use failure::err_msg;

#[derive(Debug)]
pub enum Expr {
	Value(f64),
	Expr(Box<Operation>),
	Var(String),
}

#[derive(Debug, Fail)]
#[fail(display = "Got unexpected token")]
pub struct UnexpectedToken(pub String);

#[derive(Debug, Fail)]
#[fail(display = "Parenthesis didn't match")]
pub struct MismatchedParenthesis;

#[derive(Debug, Fail)]
#[fail(display = "Variable {} wasn't set to a value", name)]
pub struct UninitializedVar {
	name: String,
}

#[derive(Debug, Fail)]
pub enum EvalError {
	#[fail(display = "{}", error)]
	UninitializedVar {
		error: UninitializedVar
	},
}

pub struct Context {
	data: HashMap<String, Expr>,
}

impl Context {
	pub fn new() -> Self {
		Context {
			data: HashMap::new()
		}
	}
	
	pub fn get(&self, name: &str) -> Option<&Expr> {
		self.data.get(name)
	}
	
	pub fn add<I: Into<Expr>>(&mut self, name: &str, val: I) {
		self.data.insert(name.to_string(), val.into());
	}
	
	pub fn inner(&mut self) -> &mut HashMap<String, Expr> {
		&mut self.data
	}
}

fn to_postfix(raw: &str) -> Result<Vec<Token>, Error> {
	let mut ops: Vec<Op> = Vec::new();
	let mut tokens: Vec<Token> = Vec::new();
	let raw = to_tokens(raw.trim())?;
	for token in raw {
		match token {
			Token::Num(val) => tokens.push(Token::Num(val)),
			Token::Name(name) => tokens.push(Token::Name(name)),
			Token::Op(op) => {
				match op {
					Op::Open => {
						ops.push(op);
					},
					Op::Close => {
						while ops.last().unwrap() != &Op::Open {
							tokens.push(Token::Op(ops.pop().unwrap()));
						}
						ops.pop();
					},
					op => {
						while let Some(op2) = ops.last().cloned() {
							let cond1 = op2.precedence() > op.precedence();
							let cond2 = op2.precedence() == op.precedence() && op.is_left_associative();
							let cond3 = op2 != Op::Open;
							if cond1 || cond2 && cond3 {
								tokens.push(Token::Op(ops.pop().unwrap()))
							} else {
								break
							}
						}
						ops.push(op);
					},
				}
			},
		}
	}
	while !ops.is_empty() {
		tokens.push(Token::Op(ops.pop().unwrap()));
	}
	
	Ok(tokens)
}

impl Expr {
	pub fn from(raw: &str) -> Result<Expr, Error> {
		let tokens = to_postfix(raw)?;
		let mut stack: Vec<Expr> = Vec::new();
		for token in tokens {
			match token {
				Token::Num(val) => {
					stack.push(Expr::Value(val))
				},
				Token::Op(op) => {
					let b = match stack.pop() {
						Some(v) => v,
						None => return Err(err_msg("Expected an expression").into())
					};
					let a = match stack.pop() {
						Some(v) => v,
						None => return Err(err_msg("Expected an expression").into())
					};
					let oper: Box<Operation> = match op {
						Op::Add => Box::new(Add { a, b }),
						Op::Sub => Box::new(Sub { a, b }),
						Op::Mul => Box::new(Mul { a, b }),
						Op::Div => Box::new(Div { a, b }),
						Op::Pow => Box::new(Pow { a, b }),
						_ => { return Err(MismatchedParenthesis.into()) }
					};
					stack.push(Expr::Expr(oper));
				},
				Token::Name(name) => {
					stack.push(Expr::Var(name))
				}
			}
		};
		if stack.len() > 1 { return Err(err_msg("Expected another operator").into()) }
		Ok(stack.into_iter().next().unwrap())
	}
	
	pub fn to_string(&self) -> String {
		match *self {
			Expr::Value(val) => val.to_string(),
			Expr::Expr(ref oper) => oper.to_string(),
			Expr::Var(ref name) => name.clone(),
		}
	}
	
	pub fn eval(&self) -> Result<f64, EvalError> {
		match *self {
			Expr::Value(val) => Ok(val),
			Expr::Expr(ref oper) => oper.eval(None),
			Expr::Var(ref name) => {
				Err(EvalError::UninitializedVar { error: UninitializedVar { name: name.clone() } }.into())
			}
		}
	}
	
	pub fn eval_ctx(&self, ctx: &Context) -> Result<f64, EvalError> {
		match *self {
			Expr::Value(val) => Ok(val),
			Expr::Expr(ref oper) => oper.eval(Some(ctx)),
			Expr::Var(ref name) => {
				if let Some(expr) = ctx.get(&name) {
					Ok(expr.eval_ctx(ctx)?)
				} else {
					Err(EvalError::UninitializedVar { error: UninitializedVar { name: name.clone() } }.into())
				}
			}
		}
	}
}

impl From<f64> for Expr {
	fn from(v: f64) -> Expr {
		Expr::Value(v)
	}
}

use std::fmt;
impl fmt::Display for Expr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}