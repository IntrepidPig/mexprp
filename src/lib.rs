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

pub fn eval(expr: &str) -> Result<f64, Error> {
	Ok(Expression::parse(expr)?.eval()?)
}
