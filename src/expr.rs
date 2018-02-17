use op::Op;

use failure::Error;
use failure::err_msg;

#[derive(Debug, Clone)]
pub enum Expr {
	Value(f64),
	Expr(Box<Oper>),
}

#[derive(Debug, Fail)]
#[fail(display = "Got unexpected token")]
pub struct UnexpectedToken(String);

impl Expr {
	fn parse_token(raw: &str) -> Result<Option<Token>, Error> {
		match raw {
			"+" => Token::Op(Op::Add),
			"-" => Token::Op(Op::Sub),
			"*" => Token::Op(Op::Mul),
			"/" => Token::Op(Op::Div),
			"^" => Token::Op(Op::Pow),
			"(" => Token::Op(Op::Open),
			")" => Token::Op(Op::Close),
			_ => {
				if
			}
		}
	}
	
	fn to_tokens(raw: &str, index: &mut usize) -> Result<Option<Token>, Error> {
		if raw.len() == 0 {
			return Ok(None);
		}
		#[derive(Debug, PartialEq, Copy, Clone)]
		enum TokenType {
			Num,
			Op,
		}
		let mut tokentype = None;
		let mut buf = String::with_capacity(4);
		
		fn get_ttype(c: char) -> Option<TokenType> {
			match c {
				'+' | '-' | '*' | '/' | '^' | '(' | ')' => Some(TokenType::Op),
				' ' => None,
				_ => {
					if c.is_numeric() || c == '.' {
						Some(TokenType::Num)
					} else {
						None
					}
				}
			}
		}
		
		fn str_to_token(raw: &str, ttype: TokenType) -> Result<Token, Error> {
			match ttype {
				TokenType::Num => {
					Ok(Token::Value(raw.parse::<f64>()?))
				},
				TokenType::Op => {
					use Op::*;
					Ok(Token::Op(match raw {
						"+" => Add,
						"-" => Sub,
						"*" => Mul,
						"/" => Div,
						"^" => Pow,
						"(" => Open,
						")" => Close,
						_ => return Err(err_msg(format!("Unexpected operator string \"{}\"", raw)).into())
					}))
				}
			}
		}
		
		for c in raw.chars() {
			println!("Char: {}\nRaw: \"{}\"\nOldType: {:?}\nNewType: {:?}\nBuf: {}\n", c, raw, tokentype, get_ttype(c), buf);
			if let Some(new_ttype) = get_ttype(c) {
				if new_ttype == TokenType::Op {
					*index += 1;
					return Ok(Some(str_to_token(&c.to_string(), new_ttype)?));
				} else if let Some(old_ttype) = tokentype {
					if new_ttype != old_ttype {
						return Ok(Some(str_to_token(&buf, old_ttype)?))
					} else {
						buf.push(c);
						*index += 1;
					}
				} else {
					tokentype = Some(new_ttype);
					buf.push(c);
					*index += 1;
				}
			} else {
				*index += 1;
			}
		}
		
		if let Some(ttype) = tokentype {
			Ok(Some(str_to_token(&buf, ttype)?))
		} else {
			Ok(None)
		}
	}
	
	fn to_postfix(raw: &str) -> Result<Vec<Token>, Error> {
		let mut index = 0;
		let mut ops: Vec<Op> = Vec::new();
		let mut nums: Vec<f64> = Vec::new();
		let mut tokens: Vec<Token> = Vec::new();
		let raw = raw.trim();
		while let Some(token) = Expr::next_token(&raw[index..raw.len()], &mut index)? {
			println!("Got token {:?}", token);
			println!("Ops: {:?}", ops);
			println!("Nums: {:?}", nums);
			println!("Tokens: {:?}\n", tokens);
			match token {
				Token::Value(val) => tokens.push(Token::Value(val)),
				Token::Op(op) => {
					if !op.is_paren() {
						while
								if !ops.is_empty() {
									(ops[ops.len() - 1].precedence() > op.precedence()) || (ops[ops.len() - 1].precedence() > op.precedence() && op.is_left_associative())
								} else {
									false
								}
										&& op != Op::Open {
							tokens.push(Token::Op(ops.pop().unwrap()));
						}
						ops.push(op);
					} else {
						match op {
							Op::Open => ops.push(op),
							Op::Close => {
								while ops[ops.len() - 1] != Op::Open {
									tokens.push(Token::Op(ops.pop().unwrap()));
								}
								if ops.pop().unwrap() != Op::Open { return Err(err_msg("Mismatched parentheses").into()) };
							}
							_ => unreachable!()
						}
					}
				}
			}
		}
		for op in ops.into_iter() {
			if op.is_paren() { return Err(err_msg("Mismatched parentheses").into()) };
			tokens.push(Token::Op(op));
		}
		
		Ok(tokens)
	}
	
	pub fn from(raw: &str) -> Result<Expr, Error> {
		let mut tokens = Expr::to_postfix(raw)?;
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
					let oper = Oper {
						a,
						op,
						b
					};
					stack.push(Expr::Expr(Box::new(oper)));
				}
			}
		};
		if stack.len() > 1 { return Err(err_msg("Too many... things").into()) }
		Ok(stack.into_iter().next().unwrap())
	}
	
	pub fn to_string(&self) -> String {
		match *self {
			Expr::Value(val) => val.to_string(),
			Expr::Expr(ref oper) => {
				format!("({} {} {})", oper.a, oper.op.to_string(), oper.b)
			}
		}
	}
	
	pub fn eval(&self) -> Result<f64, Error> {
		match *self {
			Expr::Value(val) => Ok(val),
			Expr::Expr(ref oper) => {
				let expr = oper.eval()?;
				return Ok(expr);
			}
		}
	}
}

use std::fmt;
impl fmt::Display for Expr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}

#[derive(Debug, Clone)]
pub struct Oper {
	a: Expr,
	op: Op,
	b: Expr
}

impl Oper {
	pub fn eval(&self) -> Result<f64, Error> {
		match self.op {
			Op::Add => Ok(self.a.eval()? + self.b.eval()?),
			Op::Sub => Ok(self.a.eval()? - self.b.eval()?),
			Op::Mul => Ok(self.a.eval()? * self.b.eval()?),
			Op::Div => Ok(self.a.eval()? / self.b.eval()?),
			Op::Pow => Ok(self.a.eval()?.powf(self.b.eval()?)),
			_ => Err(err_msg("Got unexpected operator").into())
		}
	}
}

#[derive(Debug, Clone)]
pub enum Token {
	Op(Op),
	Value(f64)
}