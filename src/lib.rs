//! # MEXPRP
//! A math expression parsing and evaluating library
//!
//! ## Usage
//! There are three different ways to parse and evaluate an equation.
//!
//!  #### With eval()
//! This function parses and evaluates a string all at once with the default context. There's also
//! an `eval_ctx()` function which takes a reference to a `Context` as well that will be used instead of
//! the default `Context`.
//!
//! ```rust
//! # let res =
//! mexprp::eval("10 / (2 + 3)"); // Ok(2.0)
//! # assert_eq!(res.unwrap(), 2.0);
//! ```
//!
//! #### With Expression
//! `Expression::parse()` parses a string into a tree representation (a `Term`). It can also be parsed
//! with a context with `parse_ctx`, and it will store that context within it for future evaluations.
//! It can also be evaluated with a reference to any other context with `eval_ctx`. It's important to
//! ensure that the custom context contains any definitions the `Expression` depends on.
//!
//! ```rust
//! # use mexprp::Expression;
//! let expr = Expression::parse("3 ^ 4 / 9").unwrap();
//! let res = expr.eval(); // Ok(9.0)
//! # assert_eq!(res.unwrap(), 9.0);
//! ```
//!
//! #### With Term
//! A `Term` is an `Expression`, but without any extra overhead.
//!
//! ```rust
//! # use mexprp::Term;
//! let term = Term::parse("10 ^ -3").unwrap();
//! let res = term.eval(); // Ok(0.001)
//! # assert_eq!(res.unwrap(), 0.001);
//! ```
//!
//! ### Using Contexts
//! You can evaluate expressions with custom variable and function definition's by defining a context.
//! When defining custom functions, it's important to remember to parse the expression with the custom
//! context, or else the parser will recognize your functions as variables instead. `Expression`s will
//! store the context you parse them with, but you have to evaluate `Term`s with a reference to a context
//! using `Term::eval_ctx`. For more info see the [`context`](context) module.

#![deny(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(collapsible_if))]

#[macro_use]
extern crate failure;

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
	Ok(Term::parse(expr)?.eval()?)
}

/// Parse and evaluate a string with the given context
pub fn eval_ctx(expr: &str, ctx: &Context) -> Result<f64, EvalError> {
	Ok(Term::parse_ctx(expr, ctx)?.eval_ctx(ctx)?)
}
