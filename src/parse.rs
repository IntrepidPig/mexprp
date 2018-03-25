use failure::Error;

use expr::UnexpectedToken;
use op::Op;

#[derive(PartialEq, Copy, Clone, Debug)]
pub(crate) enum TokenType {
	Op,
	Paren,
	Num,
	Name,
}

#[derive(Debug, Clone)]
pub(crate) enum Token {
	Op(Op),
	Name(String),
	Num(f64),
}

pub(crate) fn parse_token(raw: &str, token_type: TokenType) -> Result<Token, Error> {
	Ok(match token_type {
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
		TokenType::Num => {
			Token::Num(raw.parse()?)
		},
		TokenType::Name => {
			Token::Name(raw.to_string())
		},
	})
}

fn token_type(raw: char, expected: Expected) -> Result<TokenType, UnexpectedToken> {
	let ops = ['+', '-', '*', '/', '^'];
	let parens = ['(', ')'];
	if ops.contains(&raw) && expected.op {
		Ok(TokenType::Op)
	} else if parens.contains(&raw) && expected.paren {
		Ok(TokenType::Paren)
	} else if raw.is_ascii_digit() || raw == '.' || (raw == '-' && !expected.op) && expected.literal {
		Ok(TokenType::Num)
	} else if raw.is_alphabetic() && expected.name {
		Ok(TokenType::Name)
	} else {
		Err(UnexpectedToken(raw.to_string()))
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
	
	/*pub fn contains(&self, token_type: TokenType) -> bool {
		match token_type {
			TokenType::Op => self.op,
			TokenType::Paren => self.paren,
			TokenType::Literal => self.literal,
			TokenType::Name => self.name,
		}
	}*/
}

fn next_token(raw: &str, expected: Option<Expected>) -> Result<Result<(Token, &str), &str>, Error> {
	let expected = expected.unwrap_or(Expected {
		op: true,
		paren: true,
		literal: true,
		name: true,
	});
	let c = raw.chars().next().expect("Empty");
	// Skip whitespace
	if c.is_whitespace() {
		return Ok(Err(&raw[c.len_utf8()..raw.len()]));
	}
	
	let char_token_type = token_type(c, expected)?;
	let mut token_end: usize = 0;
	for next_c in raw.chars() {
		let diff = match token_type(next_c, expected) {
			Ok(next_token_type) => {
				if char_token_type == TokenType::Paren {
					token_end += next_c.len_utf8();
					true
				} else {
					next_token_type != char_token_type
				}
			},
			Err(_) => true,
		};
		if diff {
			return Ok(Ok((parse_token(&raw[0..token_end], char_token_type)?, &raw[token_end..raw.len()])));
		}
		token_end += next_c.len_utf8();
	}
	
	Err(UnexpectedToken(raw.to_string()).into())
}

pub(crate) fn to_tokens(raw: &str) -> Result<Vec<Token>, Error> {
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
				tokens.push(token);
			},
			Err(new_raw) => {
				raw = new_raw;
			}
		}
		
		let mut new_expected = Expected::none();
		match tokens.last() {
			Some(&Token::Op(Op::Open)) | Some(&Token::Op(Op::Close)) => {
				new_expected = Expected::all();
			},
			Some(&Token::Op(_)) => {
				new_expected.name = true;
				new_expected.literal = true;
				new_expected.paren = true;
			},
			Some(&Token::Num(_)) => {
				new_expected.paren = true;
				new_expected.name = true;
				new_expected.op = true;
			},
			Some(&Token::Name(_)) => {
				new_expected = Expected::all();
			},
			None => {
				new_expected = Expected::all();
			}
		}
		
		expected = Some(new_expected);
		
	}
	
	Ok(tokens)
}