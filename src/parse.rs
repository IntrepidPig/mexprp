use failure::Error;

use expr::UnexpectedToken;
use op::Op;

#[derive(PartialEq, Copy, Clone, Debug)]
pub(crate) enum TokenType {
	Op,
	Paren,
	Literal,
	Name,
}

#[derive(Debug, Clone)]
pub(crate) enum Token {
	Op(Op),
	Func(String),
	Var(String),
	Num(f64),
}

impl Token {
	pub(crate) fn tokentype(&self) -> TokenType {
		match *self {
			Token::Op(_) => TokenType::Op,
			Token::Num(_) => TokenType::Literal,
			Token::Func(_) | Token::Var(_) => TokenType::Name,
		}
	}
}

pub(crate) fn parse_token(raw: &str, ttype: TokenType) -> Result<Token, Error> {
	Ok(match ttype {
		TokenType::Op => {
			match raw {
				"+" => Token::Op(Op::Add),
				"-" => Token::Op(Op::Sub),
				"*" => Token::Op(Op::Mul),
				"/" => Token::Op(Op::Div),
				"^" => Token::Op(Op::Pow),
				_ => {
					return Err(UnexpectedToken(raw.to_string()).into())
				}
			}
		},
		TokenType::Paren => {
			match raw {
				"(" => Token::Op(Op::Open),
				")" => Token::Op(Op::Close),
				_ => {
					return Err(UnexpectedToken(raw.to_string()).into())
				}
			}
		}
		TokenType::Literal => {
			Token::Num(raw.parse()?)
		},
		TokenType::Name => {
			Token::Var(raw.to_string())
		},
	})
}

fn tokentype(raw: char) -> Result<TokenType, UnexpectedToken> {
	match raw {
		'+' | '-' | '*' | '/' | '^' => Ok(TokenType::Op),
		'(' | ')' => Ok(TokenType::Paren),
		_ => {
			if raw.is_ascii_digit() || raw == '.' {
				Ok(TokenType::Literal)
			} else if raw.is_alphabetic() {
				Ok(TokenType::Name)
			} else {
				Err(UnexpectedToken(raw.to_string()))
			}
		}
	}
}

#[derive(Debug, Copy, Clone)]
struct Expected {
	op: bool,
	paren: bool,
	literal: bool,
	name: bool,
}

impl Expected {
	pub fn list(self) -> Vec<TokenType> {
		let mut expected = Vec::new();
		if self.op {
			expected.push(TokenType::Op);
		}
		if self.paren {
			expected.push(TokenType::Paren);
		}
		if self.literal {
			expected.push(TokenType::Literal);
		}
		if self.name {
			expected.push(TokenType::Name);
		}
		expected
	}
	
	pub fn from_type(ttype: TokenType) -> Self {
		let mut e = Self::none();
		match ttype {
			TokenType::Paren => { e.paren = true; e },
			TokenType::Op => { e.op = true; e },
			TokenType::Literal => { e.literal = true; e },
			TokenType::Name => { e.name = true; e },
		}
	}
	
	pub fn none() -> Self {
		Expected {
			op: false,
			paren: false,
			literal: false,
			name: false,
		}
	}
	
	pub fn all() -> Self {
		Expected {
			op: true,
			paren: true,
			literal: true,
			name: true,
		}
	}
}

fn next_token(raw: &str, expected: Option<Expected>) -> Result<Result<(Token, &str), &str>, Error> {
	let expected = expected.unwrap_or(Expected {
		op: true,
		paren: true,
		literal: true,
		name: true,
	});
	let c = raw.chars().next().expect("Empty");
	if c.is_whitespace() {
		return Ok(Err(&raw[c.len_utf8()..raw.len()]));
	}
	let ttype = tokentype(c)?;
	for e in expected.list() {
		if ttype == e {
			let mut end: usize = 0;
			for c in raw.chars() {
				let diff = match tokentype(c) {
					Ok(testtype) => {
						if ttype == TokenType::Paren {
							end += c.len_utf8();
							true
						} else {
							testtype != ttype
						}
					},
					Err(e) => true,
				};
				if diff {
					return Ok(Ok((parse_token(&raw[0..end], ttype)?, &raw[end..raw.len()])));
				}
				end += c.len_utf8();
			}
		}
	}
	
	Err(UnexpectedToken(raw.to_string()).into())
}

pub(crate) fn to_tokens(raw: &str) -> Result<Vec<Token>, Error> {
	// The type of the token that is being built
	let mut ttype: Option<TokenType> = None;
	// The start of the token that is being built
	let mut tokenstart: usize = 0;
	// The data left to be parsed
	let mut raw: &str = raw;
	// The final token list
	let mut tokens: Vec<Token> = Vec::new();
	// The next expected token
	let mut expected = None;
	
	while raw.len() > 0 {
		match next_token(raw, expected)? {
			Ok((token, new_raw)) => {
				raw = new_raw;
				let mut new_expected = Expected::none();
				match token {
					Token::Op(_) => {
						new_expected.name = true;
						new_expected.literal = true;
						new_expected.paren = true;
					},
					Token::Func(_) => {
						new_expected = Expected::all();
					},
					Token::Num(_) => {
						new_expected.paren = true;
						new_expected.name = true;
						new_expected.op = true;
					},
					Token::Var(_) => {
						new_expected.paren = true;
						new_expected.op = true;
					}
				}
				expected = Some(new_expected);
				tokens.push(token);
			},
			Err(new_raw) => {
				raw = new_raw;
			}
		}
	}
	
	Ok(tokens)
}