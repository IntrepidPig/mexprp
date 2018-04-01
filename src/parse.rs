use failure::Error;

use op::*;
use errors::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub(crate) enum TokenType {
	Op,
	Paren,
	Num,
	Name,
	Comma,
}

#[derive(Debug, Clone)]
pub(crate) enum Token {
	Paren(Paren),
	Op(Op),
	Name(String),
	Num(f64),
	Comma,
}

#[derive(Debug, Clone)]
pub(crate) enum ParenToken {
	Op(Op),
	Num(f64),
	Name(String),
	Sub(Vec<ParenToken>),
	Comma,
}

fn parse_token(raw: &str, token_type: TokenType) -> Result<Token, Error> {
	Ok(match token_type {
		TokenType::Op => match raw {
			"+" => Token::Op(Op::Add),
			"-" => Token::Op(Op::Sub),
			"*" => Token::Op(Op::Mul),
			"/" => Token::Op(Op::Div),
			"^" => Token::Op(Op::Pow),
			_ => return Err(UnexpectedToken(raw.to_string()).into()),
		},
		TokenType::Paren => match raw {
			"(" => Token::Paren(Paren::Open),
			")" => Token::Paren(Paren::Close),
			_ => return Err(UnexpectedToken(raw.to_string()).into()),
		},
		TokenType::Num => {
			if raw == "-" {
				Token::Num(-1.0)
			} else {
				Token::Num(raw.parse()?)
			}
		}
		TokenType::Name => Token::Name(raw.to_string()),
		TokenType::Comma => match raw {
			"," => Token::Comma,
			_ => return Err(UnexpectedToken(raw.to_string()).into()),
		},
	})
}

fn token_type(raw: char, expected: Expected) -> Result<TokenType, UnexpectedToken> {
	let ops = ['+', '-', '*', '/', '^'];
	let parens = ['(', ')'];
	if ops.contains(&raw) && expected.op {
		Ok(TokenType::Op)
	} else if raw == ',' && expected.comma {
		Ok(TokenType::Comma)
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
	comma: bool,
}

impl Expected {
	pub fn none() -> Self {
		Expected {
			op: false,
			paren: false,
			literal: false,
			name: false,
			comma: false,
		}
	}

	pub fn all() -> Self {
		Expected {
			op: true,
			paren: true,
			literal: true,
			name: true,
			comma: true,
		}
	}
}

/// Get the next token of a string based on what's expected. Returns either a                //  ↓ never delete this
fn next_token(raw: &str, expected: Expected) -> Result<Result<(Token, &str), &str>, Error> {
	// TODO clean
	let c = raw.chars().next().expect("Empty");

	// Skip whitespace and return the substring after this whitespace character
	if c.is_whitespace() {
		return Ok(Err(&raw[c.len_utf8()..raw.len()]));
	}

	let char_token_type = token_type(c, expected)?;
	let mut token_end: usize = 0;
	for next_c in raw.chars() {
		let diff = match token_type(next_c, expected) {
			Ok(next_token_type) => {
				if char_token_type == TokenType::Paren || char_token_type == TokenType::Op || char_token_type == TokenType::Comma {
					// Only allows one character operators
					token_end += next_c.len_utf8();
					true
				} else {
					next_token_type != char_token_type
				}
			}
			Err(_) => true,
		};
		if diff {
			return Ok(Ok((
				parse_token(&raw[0..token_end], char_token_type)?,
				&raw[token_end..raw.len()],
			)));
		}
		token_end += next_c.len_utf8();
	}

	Ok(Ok((
		parse_token(&raw[0..token_end], char_token_type)?,
		&raw[token_end..raw.len()],
	)))
}

fn to_tokens(raw: &str) -> Result<Vec<Token>, Error> {
	// The data left to be parsed
	let mut raw: &str = raw;
	// The final token list
	let mut tokens: Vec<Token> = Vec::new();
	// The next expected token
	let mut expected = Expected {
		name: true,
		literal: true,
		paren: true,
		op: false,    // First token can't be an operator
		comma: false, // First token can't be a comma
	};

	while raw.len() > 0 {
		match next_token(raw, expected)? {
			Ok((token, new_raw)) => {
				raw = new_raw;
				tokens.push(token);
			}
			Err(new_raw) => {
				raw = new_raw;
			}
		}

		let mut new_expected = Expected::none();
		match tokens.last() {
			Some(&Token::Paren(Paren::Open)) => {
				// Any token can come after a open parentheses (except a comma)
				new_expected.paren = true;
				new_expected.name = true;
				new_expected.op = true;
				new_expected.literal = true;
			}
			Some(&Token::Paren(Paren::Close)) => {
				// Any token can come after a close parentheses
				new_expected = Expected::all();
			}
			Some(&Token::Op(_)) => {
				// An operator can't come after another operator (no unary ops)
				new_expected.name = true;
				new_expected.literal = true;
				new_expected.paren = true;
			}
			Some(&Token::Num(_)) => {
				// A number can't come after another number
				new_expected.paren = true;
				new_expected.name = true;
				new_expected.op = true;
				new_expected.comma = true;
			}
			Some(&Token::Name(_)) => {
				// Anything can come after a name
				new_expected = Expected::all();
			}
			Some(&Token::Comma) => {
				// Only operands can come after a comma
				new_expected.paren = true;
				new_expected.name = true;
				new_expected.literal = true;
			}
			None => {
				// An operator or comma can't be the first token
				new_expected.name = true;
				new_expected.literal = true;
				new_expected.paren = true;
			}
		}

		expected = new_expected;
	}

	Ok(tokens)
}

fn to_paren_tokens(raw: Vec<Token>) -> Result<Vec<ParenToken>, Error> {
	trace!("Converting raw tokens to paren tokens");
	fn recurse(raw: &[Token]) -> Result<Vec<ParenToken>, Error> {
		let mut parentokens = Vec::new();

		let mut start = 0;
		let mut paren_count = 0;
		let mut counting = false;

		for (i, token) in raw.iter().enumerate() {
			match *token {
				Token::Num(num) => {
					if !counting {
						parentokens.push(ParenToken::Num(num)); // Only push the number if it's not part of a subexpression
					}
				}
				Token::Op(ref op) => {
					if !counting {
						parentokens.push(ParenToken::Op(op.clone())); // Only push the op if it's not part of a subexpression
					}
				}
				Token::Paren(Paren::Open) => {
					if !counting {
						start = i; // If we aren't already in a subexpression, start counting here
					}
					counting = true; // Say we are counting
					paren_count += 1; // Up the open parentheses count
				}
				Token::Paren(Paren::Close) => {
					paren_count -= 1; // Lower the open parentheses count

					if paren_count < 0 {
						// Ensure we haven't gone below the amount of parentheses
						return Err(MismatchedParenthesis.into());
					}

					if paren_count == 0 {
						// If we have reached the matching end parentheses
						counting = false; // Say we are not in a subexpression anymore
						parentokens.push(ParenToken::Sub(recurse(&raw[start + 1..i])?)); // Just push the subexpression
					}
				}
				Token::Name(ref name) => {
					if !counting {
						parentokens.push(ParenToken::Name(name.clone())); // Only push the var if it's not part of the subexpression
					}
				}
				Token::Comma => {
					if !counting {
						parentokens.push(ParenToken::Comma); // Only push the comma if it's not part of the subexpression
					}
				}
			}
		}

		Ok(parentokens)
	}

	recurse(&raw)
}

pub(crate) fn get_tokens(raw: &str) -> Result<Vec<ParenToken>, Error> {
	let raw_tokens = to_tokens(raw)?;
	debug!("Raw tokens: {:?}", raw_tokens);
	let paren_tokens = to_paren_tokens(raw_tokens)?;
	debug!("Paren tokens: {:?}", paren_tokens);

	Ok(paren_tokens)
}
