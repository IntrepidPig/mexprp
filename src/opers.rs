use std::fmt::Debug;

use expr::{Term};
use context::Context;
use errors::MathError;

pub trait Operate: Debug {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError>;
	fn to_string(&self) -> String {
		String::from("({})")
	}
}

#[derive(Debug)]
pub struct Add {
	pub a: Term,
	pub b: Term
}

impl Operate for Add {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		Ok(self.a.eval(ctx)? + self.b.eval(ctx)?)
	}
	
	fn to_string(&self) -> String {
		format!("({} + {})", self.a, self.b)
	}
}

#[derive(Debug)]
pub struct Sub {
	pub a: Term,
	pub b: Term
}

impl Operate for Sub {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		Ok(self.a.eval(ctx)? - self.b.eval(ctx)?)
	}
	
	fn to_string(&self) -> String {
		format!("({} - {})", self.a, self.b)
	}
}

#[derive(Debug)]
pub struct Mul {
	pub a: Term,
	pub b: Term
}

impl Operate for Mul {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		Ok(self.a.eval(ctx)? * self.b.eval(ctx)?)
	}
	
	fn to_string(&self) -> String {
		format!("({} * {})", self.a, self.b)
	}
}

#[derive(Debug)]
pub struct Div {
	pub a: Term,
	pub b: Term
}

impl Operate for Div {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		let b = self.b.eval(ctx)?;
		if b == 0.0 {
			return Err(MathError::DivideByZero);
		}
		Ok(self.a.eval(ctx)? / b)
	}
	
	fn to_string(&self) -> String {
		format!("({} / {})", self.a, self.b)
	}
}

#[derive(Debug)]
pub struct Pow {
	pub a: Term,
	pub b: Term
}

impl Operate for Pow {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		Ok(self.a.eval(ctx)?.powf(self.b.eval(ctx)?))
	}
	
	fn to_string(&self) -> String {
		format!("({} ^ {})", self.a, self.b)
	}
}

pub struct Operation {
	inner: Box<Operate>,
}

impl Operation {

}

impl<T> From<T> for Operation where T: Operate + 'static {
	fn from(t: T) -> Self {
		Self {
			inner: Box::new(t)
		}
	}
}