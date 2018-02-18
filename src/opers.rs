use std::fmt::Debug;

use expr::{Expr, EvalError, Context};

pub trait Operation: Debug {
	fn eval(&self, ctx: Option<&Context>) -> Result<f64, EvalError>;
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
	fn eval(&self, ctx: Option<&Context>) -> Result<f64, EvalError> {
		if let Some(ctx) = ctx {
			Ok(self.a.eval_ctx(ctx)? + self.b.eval_ctx(ctx)?)
		} else {
			Ok(self.a.eval()? + self.b.eval()?)
		}
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
	fn eval(&self, ctx: Option<&Context>) -> Result<f64, EvalError> {
		if let Some(ctx) = ctx {
			Ok(self.a.eval_ctx(ctx)? - self.b.eval_ctx(ctx)?)
		} else {
			Ok(self.a.eval()? - self.b.eval()?)
		}
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
	fn eval(&self, ctx: Option<&Context>) -> Result<f64, EvalError> {
		if let Some(ctx) = ctx {
			Ok(self.a.eval_ctx(ctx)? * self.b.eval_ctx(ctx)?)
		} else {
			Ok(self.a.eval()? * self.b.eval()?)
		}
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
	fn eval(&self, ctx: Option<&Context>) -> Result<f64, EvalError> {
		if let Some(ctx) = ctx {
			Ok(self.a.eval_ctx(ctx)? / self.b.eval_ctx(ctx)?)
		} else {
			Ok(self.a.eval()? / self.b.eval()?)
		}
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
	fn eval(&self, ctx: Option<&Context>) -> Result<f64, EvalError> {
		if let Some(ctx) = ctx {
			Ok(self.a.eval_ctx(ctx)?.powf(self.b.eval_ctx(ctx)?))
		} else {
			Ok(self.a.eval()?.powf(self.b.eval()?))
		}
	}
	
	fn to_string(&self) -> String {
		format!("({} ^ {})", self.a, self.b)
	}
}