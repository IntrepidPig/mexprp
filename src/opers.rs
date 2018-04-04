use std::fmt::Debug;

use expr::Term;
use context::Context;
use errors::MathError;

/// A trait for operations
pub trait Operate: Debug {
	/// Evalute the operation or return an error
	fn eval(&self, ctx: &Context) -> Result<f64, MathError>;
	/// Convert the operation to a string representation
	fn to_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub(crate) struct Add {
	pub a: Term,
	pub b: Term,
}

impl Operate for Add {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		Ok(self.a.eval_ctx(ctx)? + self.b.eval_ctx(ctx)?)
	}

	fn to_string(&self) -> String {
		format!("({} + {})", self.a, self.b)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Sub {
	pub a: Term,
	pub b: Term,
}

impl Operate for Sub {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		Ok(self.a.eval_ctx(ctx)? - self.b.eval_ctx(ctx)?)
	}

	fn to_string(&self) -> String {
		format!("({} - {})", self.a, self.b)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Mul {
	pub a: Term,
	pub b: Term,
}

impl Operate for Mul {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		Ok(self.a.eval_ctx(ctx)? * self.b.eval_ctx(ctx)?)
	}

	fn to_string(&self) -> String {
		format!("({} × {})", self.a, self.b)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Div {
	pub a: Term,
	pub b: Term,
}

impl Operate for Div {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		let b = self.b.eval_ctx(ctx)?;
		if b == 0.0 {
			return Err(MathError::DivideByZero);
		}
		Ok(self.a.eval_ctx(ctx)? / b)
	}

	fn to_string(&self) -> String {
		format!("({} ÷ {})", self.a, self.b)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Pow {
	pub a: Term,
	pub b: Term,
}

impl Operate for Pow {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		Ok(self.a.eval_ctx(ctx)?.powf(self.b.eval_ctx(ctx)?))
	}

	fn to_string(&self) -> String {
		format!("({} ^ {})", self.a, self.b)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Neg {
	pub a: Term,
}

impl Operate for Neg {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		Ok(-self.a.eval_ctx(ctx)?)
	}
	
	fn to_string(&self) -> String {
		format!("(-{})", self.a)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Pos {
	pub a: Term,
}

impl Operate for Pos {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		Ok(self.a.eval_ctx(ctx)?)
	}
	
	fn to_string(&self) -> String {
		format!("(+{})", self.a)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Fact {
	pub a: Term,
}

impl Operate for Fact {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		let a = self.a.eval_ctx(ctx)?.round() as i64;
		
		let mut sum = 1i64;
		for i in 1..=a {
			sum = sum * i
		}
		
		Ok(sum as f64)
	}
	
	fn to_string(&self) -> String {
		format!("({}!)", self.a)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Percent {
	pub a: Term,
}

impl Operate for Percent {
	fn eval(&self, ctx: &Context) -> Result<f64, MathError> {
		let a = self.a.eval_ctx(ctx)?;
		
		Ok(a / 100.0)
	}
	
	fn to_string(&self) -> String {
		format!("({}%)", self.a)
	}
}

