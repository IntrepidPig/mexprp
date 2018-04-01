use std::fmt::Debug;

use expr::Term;
use context::Context;
use errors::MathError;

/// A trait for operations
pub trait Operate: Debug {
	/// Evalute the operation or return an error
	fn eval(&self, ctx: &Context) -> Result<f64, MathError>;
	/// Convert the operation to a string representation
	fn to_string(&self) -> String {
		String::from("({})")
	}
}

#[derive(Debug)]
pub(crate) struct Add {
	pub a: Term,
	pub b: Term,
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
pub(crate) struct Sub {
	pub a: Term,
	pub b: Term,
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
pub(crate) struct Mul {
	pub a: Term,
	pub b: Term,
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
pub(crate) struct Div {
	pub a: Term,
	pub b: Term,
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
pub(crate) struct Pow {
	pub a: Term,
	pub b: Term,
}

impl Operate for Pow {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		Ok(self.a.eval(ctx)?.powf(self.b.eval(ctx)?))
	}

	fn to_string(&self) -> String {
		format!("({} ^ {})", self.a, self.b)
	}
}
