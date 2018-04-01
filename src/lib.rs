#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

use failure::Error;

pub mod func;
pub mod parse;
pub mod op;
pub mod expr;
pub mod opers;
pub mod errors;
pub mod context;
#[cfg(test)]
mod tests;

pub use func::Func;
pub use expr::{Calculation, Expression, Term};
pub use context::Context;

/// Parse and evaluate a string
pub fn eval(expr: &str) -> Result<f64, Error> {
	Ok(Expression::parse(expr)?.eval()?)
}

/// Parse and evaluate a string with the given context
pub fn eval_ctx(expr: &str, ctx: &Context) -> Result<f64, Error> {
	Ok(Expression::parse_ctx(expr, ctx)?.eval_ctx(ctx)?)
}
