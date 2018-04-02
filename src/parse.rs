use op::*;
use errors::*;

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

/// Get a number at the beginning of a string
fn next_num(raw: &str) -> Option<(Token, &str)> {
	let mut buf = "";
	let mut dot = false;

	for c in raw.chars() {
		if c.is_digit(10) {
			buf = &raw[0..buf.len() + c.len_utf8()];
		} else if c == '-' {
			if !buf.is_empty() {
				if buf == "-" {
					return Some((Token::Num(-1.0), &raw[buf.len()..raw.len()]));
				} else {
					return Some((
						Token::Num(match buf.parse() {
							Ok(v) => v,
							Err(_e) => {
								return None;
							}
						}),
						&raw[buf.len()..raw.len()],
					));
				}
			} else {
				buf = &raw[0..buf.len() + c.len_utf8()];
			}
		} else if c == '.' {
			if !dot {
				dot = true;
				buf = &raw[0..buf.len() + c.len_utf8()];
			} else {
				return None;
			}
		} else {
			if buf.is_empty() {
				return None;
			} else if buf == "-" {
				return Some((Token::Num(-1.0), &raw[buf.len()..raw.len()]));
			} else {
				return Some((
					Token::Num(match buf.parse() {
						Ok(v) => v,
						Err(_e) => {
							return None;
						}
					}),
					&raw[buf.len()..raw.len()],
				));
			}
		}
	}

	if buf.is_empty() {
		None
	} else if buf == "-" {
		Some((Token::Num(-1.0), &raw[buf.len()..raw.len()]))
	} else {
		Some((
			Token::Num(match buf.parse() {
				Ok(v) => v,
				Err(_e) => {
					return None;
				}
			}),
			&raw[buf.len()..raw.len()],
		))
	}
}

/// Function that can be used to retrieve a token
type TokenFn = fn(&str) -> Option<(Token, &str)>;

/// Get the parentheses at the beginning of a string
fn next_paren(raw: &str) -> Option<(Token, &str)> {
	if let Some(c) = raw.chars().next() {
		match c {
			'(' => Some((Token::Paren(Paren::Open), &raw[c.len_utf8()..raw.len()])),
			')' => Some((Token::Paren(Paren::Close), &raw[c.len_utf8()..raw.len()])),
			_ => None,
		}
	} else {
		None
	}
}

/// Get the operator at the beginning of a string
fn next_op(raw: &str) -> Option<(Token, &str)> {
	if let Some(c) = raw.chars().next() {
		match c {
			'+' => Some((Token::Op(Op::Add), &raw[c.len_utf8()..raw.len()])),
			'-' => Some((Token::Op(Op::Sub), &raw[c.len_utf8()..raw.len()])),
			'*' => Some((Token::Op(Op::Mul), &raw[c.len_utf8()..raw.len()])),
			'/' => Some((Token::Op(Op::Div), &raw[c.len_utf8()..raw.len()])),
			'^' => Some((Token::Op(Op::Pow), &raw[c.len_utf8()..raw.len()])),
			_ => None,
		}
	} else {
		None
	}
}

/// Get the name at the beginning of a string
fn next_name(raw: &str) -> Option<(Token, &str)> {
	let mut name = "";
	for c in raw.chars() {
		if c.is_alphabetic() || c == '_' {
			name = &raw[0..name.len() + c.len_utf8()];
		} else {
			if name.is_empty() {
				return None;
			} else {
				return Some((Token::Name(name.to_string()), &raw[name.len()..raw.len()]));
			}
		}
	}

	if name.is_empty() {
		None
	} else {
		Some((Token::Name(name.to_string()), &raw[name.len()..raw.len()]))
	}
}

/// Get the comma at the beginning of a string
fn next_comma(raw: &str) -> Option<(Token, &str)> {
	if let Some(c) = raw.chars().next() {
		match c {
			',' => Some((Token::Comma, &raw[c.len_utf8()..raw.len()])),
			_ => None,
		}
	} else {
		None
	}
}

/// Return a list of functions to use (in order) to try and parse the next token based on the last token
/// that was parsed.
fn get_parse_order(last: Option<&Token>) -> &[TokenFn] {
	match last {
		Some(&Token::Paren(Paren::Open)) => &[next_paren, next_name, next_num],
		Some(&Token::Paren(Paren::Close)) => &[next_paren, next_comma, next_op, next_name, next_num],
		Some(&Token::Op(_)) => &[next_paren, next_name, next_num],
		Some(&Token::Num(_)) => &[next_paren, next_comma, next_op, next_name],
		Some(&Token::Name(_)) => &[next_paren, next_comma, next_op, next_name, next_num],
		Some(&Token::Comma) => &[next_paren, next_name, next_num],
		None => &[next_paren, next_name, next_num],
	}
}

/// Get the next token of a string based on the last token. Returns either a Token and the rest of the
/// string or an error
fn next_token<'a>(raw: &'a str, last: Option<&Token>) -> Result<(Token, &'a str), ParseError> {
	let parseorder = get_parse_order(last);

	let mut tok_start = 0;
	for c in raw.chars() {
		if c.is_whitespace() {
			tok_start += c.len_utf8();
		} else {
			break;
		}
	}
	let raw = &raw[tok_start..raw.len()];

	for next_func in parseorder {
		if let Some(new) = (*next_func)(raw) {
			return Ok(new);
		}
	}

	Err(ParseError::UnexpectedToken {
		token: raw.chars().next().unwrap().to_string(),
	})
}

/// Convert a string to a list of tokens
fn to_tokens(mut raw: &str) -> Result<Vec<Token>, ParseError> {
	let mut tokens = Vec::new();
	while !raw.is_empty() {
		let (tok, new_raw) = next_token(raw, tokens.last())?;
		tokens.push(tok);
		raw = new_raw;
	}
	Ok(tokens)
}

/// Convert tokens to a tree based on expression within parentheses
fn to_paren_tokens(raw: Vec<Token>) -> Result<Vec<ParenToken>, ParseError> {
	fn recurse(raw: &[Token]) -> Result<Vec<ParenToken>, ParseError> {
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
						return Err(ParseError::MismatchedParentheses);
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

/// Get ParenTokens from a string
pub(crate) fn get_tokens(raw: &str) -> Result<Vec<ParenToken>, ParseError> {
	let raw_tokens = to_tokens(raw)?;
	let paren_tokens = to_paren_tokens(raw_tokens)?;

	Ok(paren_tokens)
}
