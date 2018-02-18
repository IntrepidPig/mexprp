use op::Op;
use opers::*;

use failure::Error;
use failure::err_msg;

#[derive(Debug)]
pub enum Expr {
	Value(f64),
	Expr(Box<Operation>),
}

#[derive(Debug, Fail)]
#[fail(display = "Got unexpected token")]
pub struct UnexpectedToken(String);

#[derive(Debug, Fail)]
#[fail(display = "Parenthesis didn't match")]
pub struct MismatchedParenthesis;

fn parse_token(raw: &str, ttype: TokenType) -> Result<Option<Token>, Error> {
	Ok(match ttype {
		TokenType::Op => {
			Some(match raw {
				"+" => Token::Op(Op::Add),
				"-" => Token::Op(Op::Sub),
				"*" => Token::Op(Op::Mul),
				"/" => Token::Op(Op::Div),
				"^" => Token::Op(Op::Pow),
				"(" => Token::Op(Op::Open),
				")" => Token::Op(Op::Close),
				_ => {
					return Err(UnexpectedToken(raw.to_string()).into())
				}
			})
		},
		TokenType::Lit => {
			Some(Token::Value(raw.parse()?))
		},
		TokenType::None => {
			None
		}
	})
}

fn tokentype(raw: char) -> Result<TokenType, UnexpectedToken> {
	match raw {
		'+' | '-' | '*' | '/' | '^' | '(' | ')' => Ok(TokenType::Op),
		_ => {
			if raw.is_ascii_digit() || raw == '.' {
				Ok(TokenType::Lit)
			} else if raw.is_whitespace() {
				Ok(TokenType::None)
			} else {
				Err(UnexpectedToken(raw.to_string()))
			}
		}
	}
}

fn to_tokens(raw: &str) -> Result<Vec<Token>, Error> {
	let mut prevttype = TokenType::None;
	let mut tbuf = String::new();
	let mut tokens = Vec::new();
	
	for c in raw.chars() {
		let ttype = tokentype(c)?;
		if ttype == prevttype {
			tbuf.push(c);
		} else {
			if prevttype != TokenType::None {
				tokens.push(parse_token(&tbuf, prevttype)?.unwrap());
			}
			tbuf.clear();
			tbuf.push(c);
			prevttype = ttype;
		}
	}
	
	if prevttype != TokenType::None {
		tokens.push(parse_token(&tbuf, prevttype)?.unwrap());
	}
	
	Ok(tokens)
}

fn to_postfix(raw: &str) -> Result<Vec<Token>, Error> {
	let mut index = 0;
	let mut ops: Vec<Op> = Vec::new();
	let mut nums: Vec<f64> = Vec::new();
	let mut tokens: Vec<Token> = Vec::new();
	let mut raw = to_tokens(raw.trim())?;
	for token in raw {
		println!("Got token {:?}", token);
		match token {
			Token::Value(val) => tokens.push(Token::Value(val)),
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
			}
		}
		println!("Ops: {:?}", ops);
		println!("Nums: {:?}", nums);
		println!("Tokens: {:?}\n", tokens);
	}
	while !ops.is_empty() {
		tokens.push(Token::Op(ops.pop().unwrap()));
	}
	
	Ok(tokens)
}

impl Expr {
	pub fn from(raw: &str) -> Result<Expr, Error> {
		let mut tokens = to_postfix(raw)?;
		println!("\nTokens: {:?}", tokens);
		let mut expr = Expr::Value(0.0);
		let mut stack: Vec<Expr> = Vec::new();
		for token in tokens {
			println!("Stack is {:?}", stack);
			match token {
				Token::Value(val) => {
					stack.push(Expr::Value(val))
				},
				Token::Op(op) => {
					let b = stack.pop().unwrap();
					let a = stack.pop().unwrap();
					let oper: Box<Operation> = match op {
						Op::Add => Box::new(Add { a, b }),
						Op::Sub => Box::new(Sub { a, b }),
						Op::Mul => Box::new(Mul { a, b }),
						Op::Div => Box::new(Div { a, b }),
						Op::Pow => Box::new(Pow { a, b }),
						_ => { return Err(MismatchedParenthesis.into()) }
					};
					stack.push(Expr::Expr(oper));
				}
			}
		};
		if stack.len() > 1 { return Err(err_msg("Too many... things").into()) }
		Ok(stack.into_iter().next().unwrap())
	}
	
	pub fn to_string(&self) -> String {
		match *self {
			Expr::Value(val) => val.to_string(),
			Expr::Expr(ref oper) => { oper.to_string() }
		}
	}
	
	pub fn eval(&self) -> f64 {
		match *self {
			Expr::Value(val) => val,
			Expr::Expr(ref oper) => oper.eval()
		}
	}
}

use std::fmt;
impl fmt::Display for Expr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}

#[derive(PartialEq)]
enum TokenType {
	Op,
	Lit,
	None,
}

#[derive(Debug, Clone)]
pub enum Token {
	Op(Op),
	Value(f64)
}