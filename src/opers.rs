use std::fmt::Debug;

use expr::Term;
use context::Context;
use errors::MathError;
use num::*;

/// The result of an evaluation
pub type Calculation<N> = Result<N, MathError>;

/// A trait for operations
pub trait Operate<N: Num>: Debug {
	/// Evalute the operation or return an error
	fn eval(&self, ctx: &Context<N>) -> Calculation<N>;
	/// Convert the operation to a string representation
	fn to_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub(crate) struct Add<N: Num> {
	pub a: Term<N>,
	pub b: Term<N>,
}

impl<N: Num + 'static> Operate<N> for Add<N> {
	fn eval(&self, ctx: &Context<N>) -> Calculation<N> {
		let a = self.a.eval_ctx(ctx)?;
		let b = self.b.eval_ctx(ctx)?;
		
		a.add(&b)
	}

	fn to_string(&self) -> String {
		format!("({} + {})", self.a, self.b)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Sub<N: Num> {
	pub a: Term<N>,
	pub b: Term<N>,
}

impl<N: Num + 'static> Operate<N> for Sub<N> {
	fn eval(&self, ctx: &Context<N>) -> Calculation<N> {
		let a = self.a.eval_ctx(ctx)?;
		let b = self.b.eval_ctx(ctx)?;
		
		a.sub(&b)
	}

	fn to_string(&self) -> String {
		format!("({} - {})", self.a, self.b)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Mul<N: Num> {
	pub a: Term<N>,
	pub b: Term<N>,
}

impl<N: Num + 'static> Operate<N> for Mul<N> {
	fn eval(&self, ctx: &Context<N>) -> Calculation<N> {
		let a = self.a.eval_ctx(ctx)?;
		let b = self.b.eval_ctx(ctx)?;
		
		a.mul(&b)
	}

	fn to_string(&self) -> String {
		format!("({} × {})", self.a, self.b)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Div<N: Num> {
	pub a: Term<N>,
	pub b: Term<N>,
}

impl<N: Num + 'static> Operate<N> for Div<N> {
	fn eval(&self, ctx: &Context<N>) -> Calculation<N> {
		let a = self.a.eval_ctx(ctx)?;
		let b = self.b.eval_ctx(ctx)?;
		
		a.div(&b)
	}

	fn to_string(&self) -> String {
		format!("({} ÷ {})", self.a, self.b)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Pow<N: Num> {
	pub a: Term<N>,
	pub b: Term<N>,
}

impl<N: Num + 'static> Operate<N> for Pow<N> {
	fn eval(&self, ctx: &Context<N>) -> Calculation<N> {
		let a = self.a.eval_ctx(ctx)?;
		let b = self.b.eval_ctx(ctx)?;
		
		a.pow(&b)
	}

	fn to_string(&self) -> String {
		format!("({} ^ {})", self.a, self.b)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Neg<N: Num> {
	pub a: Term<N>,
}

impl<N: Num + 'static> Operate<N> for Neg<N> {
	fn eval(&self, ctx: &Context<N>) -> Calculation<N> {
		let a = self.a.eval_ctx(ctx)?;
		
		a.mul(&N::from_f64(-1.0)?)
	}

	fn to_string(&self) -> String {
		format!("(-{})", self.a)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Pos<N: Num> {
	pub a: Term<N>,
}

impl<N: Num + 'static> Operate<N> for Pos<N> {
	fn eval(&self, ctx: &Context<N>) -> Calculation<N> {
		let a = self.a.eval_ctx(ctx)?;
		
		Ok(a)
	}

	fn to_string(&self) -> String {
		format!("(+{})", self.a)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Fact<N: Num> {
	pub a: Term<N>,
}

impl<N: Num + 'static> Operate<N> for Fact<N> {
	fn eval(&self, ctx: &Context<N>) -> Calculation<N> {
		unimplemented!()
	}

	fn to_string(&self) -> String {
		format!("({}!)", self.a)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Percent<N: Num> {
	pub a: Term<N>,
}

impl<N: Num + 'static> Operate<N> for Percent<N> {
	fn eval(&self, ctx: &Context<N>) -> Calculation<N> {
		let a = self.a.eval_ctx(ctx)?;
		
		a.mul(&N::from_f64(0.01)?)
	}

	fn to_string(&self) -> String {
		format!("({}%)", self.a)
	}
}
