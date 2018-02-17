#[derive(Debug, Clone, PartialEq)]
pub enum Op {
	Pow,
	Mul,
	Div,
	Add,
	Sub,
	Open,
	Close
}

impl Op {
	pub fn precedence(&self) -> i32 {
		use Op::*;
		match *self {
			Pow => 4,
			Mul | Div => 3,
			Add | Sub=> 2,
			Open | Close => 1
		}
	}
	
	pub fn is_higher(&self, other: Op) -> bool {
		self.precedence() > other.precedence()
	}
	
	pub fn is_lower(&self, other: Op) -> bool {
		self.precedence() < other.precedence()
	}
	
	pub fn is_left_associative(&self) -> bool {
		use Op::*;
		match *self {
			Pow => false,
			_ => true
		}
	}
	
	pub fn to_string(&self) -> String {
		use Op::*;
		String::from(match *self {
			Pow => "^",
			Mul => "*",
			Div => "/",
			Add => "+",
			Sub => "-",
			Open => "(",
			Close => ")"
		})
	}
	
	pub fn is_paren(&self) -> bool {
		*self == Op::Open || *self == Op::Close
	}
}

use std::fmt;
impl fmt::Display for Op {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}