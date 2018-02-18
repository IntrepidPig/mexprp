use std::fmt::Debug;

use expr::Expr;

pub trait Operation: Debug {
	fn eval(&self) -> f64;
	fn to_string(&self) -> String {
		String::from("({})")
	}
}

#[derive(Debug)]
pub struct Add {
	pub a: Expr,
	pub b: Expr
}

impl Operation for Add {
	fn eval(&self) -> f64 {
		self.a.eval() + self.b.eval()
	}
	
	fn to_string(&self) -> String {
		format!("({} + {})", self.a, self.b)
	}
}

#[derive(Debug)]
pub struct Sub {
	pub a: Expr,
	pub b: Expr
}

impl Operation for Sub {
	fn eval(&self) -> f64 {
		self.a.eval() - self.b.eval()
	}
	
	fn to_string(&self) -> String {
		format!("({} - {})", self.a, self.b)
	}
}

#[derive(Debug)]
pub struct Mul {
	pub a: Expr,
	pub b: Expr
}

impl Operation for Mul {
	fn eval(&self) -> f64 {
		self.a.eval() * self.b.eval()
	}
	
	fn to_string(&self) -> String {
		format!("({} * {})", self.a, self.b)
	}
}

#[derive(Debug)]
pub struct Div {
	pub a: Expr,
	pub b: Expr
}

impl Operation for Div {
	fn eval(&self) -> f64 {
		self.a.eval() / self.b.eval()
	}
	
	fn to_string(&self) -> String {
		format!("({} / {})", self.a, self.b)
	}
}

#[derive(Debug)]
pub struct Pow {
	pub a: Expr,
	pub b: Expr
}

impl Operation for Pow {
	fn eval(&self) -> f64 {
		self.a.eval().powf(self.b.eval())
	}
	
	fn to_string(&self) -> String {
		format!("({} ^ {})", self.a, self.b)
	}
}