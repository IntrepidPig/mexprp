//! # MEXPRP
//! A math expression parsing and evaluating library
//!
//! ## Usage
//! The recommended way to use it is with the Expression struct, but it can also be used for one-off
//! usages with the `eval()` and `eval_ctx()` functions in the root. Expressions can be parsed without
//! a context and will use the default one, but in order to use custom constants and functions, Expressions
//! need to be parsed AND evaluated with the context you wish to use.
//!
//!  ### Evaluating an expression
//! ```rust
//! # let res =
//! mexprp::eval("10 / (2 + 3)"); // Ok(2.0)
//! # assert_eq!(res.unwrap(), 2.0);
//! ```
//!
//! ### Compiling an expression
//!
//! ```rust
//! # use mexprp::Expression;
//! let expr = Expression::parse("3 ^ 4 / 9").unwrap();
//! let res = expr.eval(); // Ok(9.0)
//! # assert_eq!(res.unwrap(), 9.0);
//! ```

#![deny(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(collapsible_if))]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

/// Contains Function trait
pub mod func;
/// Contains methods for parsing equations into token representations
pub mod parse;
/// Contains definitions for Operations
pub mod op;
/// Contains expressions and terms
pub mod expr;
/// Contains implementations for operations
pub mod opers;
/// All the errors
pub mod errors;
/// Context struct
pub mod context;
#[cfg(test)]
mod tests;

pub use func::Func;
pub use expr::{Calculation, Expression, Term};
pub use context::Context;
pub use errors::{EvalError, MathError, ParseError};

/// Parse and evaluate a string
pub fn eval(expr: &str) -> Result<f64, EvalError> {
	Ok(Expression::parse(expr)?.eval()?)
}

/// Parse and evaluate a string with the given context
pub fn eval_ctx(expr: &str, ctx: &Context) -> Result<f64, EvalError> {
	Ok(Expression::parse_ctx(expr, ctx)?.eval_ctx(ctx)?)
}
