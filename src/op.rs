#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Op {
	Pow,
	Mul,
	Div,
	Add,
	Sub,
}

impl Op {
	pub fn precedence(&self) -> i32 {
		use self::Op::*;
		match *self {
			Pow => 4,
			Mul | Div => 3,
			Add | Sub => 2,
		}
	}

	pub fn is_left_associative(&self) -> bool {
		use self::Op::*;
		match *self {
			Pow => false,
			Mul => true,
			Div => true,
			Add => true,
			Sub => true,
		}
	}

	pub fn to_string(&self) -> String {
		use self::Op::*;
		String::from(match *self {
			Pow => "^",
			Mul => "*",
			Div => "/",
			Add => "+",
			Sub => "-",
		})
	}
}

use std::fmt;
impl fmt::Display for Op {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Paren {
	Open,
	Close,
}

impl Paren {
	pub fn to_str(&self) -> &str {
		match *self {
			Paren::Open => "(",
			Paren::Close => ")",
		}
	}
}

impl fmt::Display for Paren {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(self.to_str())
	}
}
