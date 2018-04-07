//! # MEXPRP
//! A math expression parsing and evaluating library
//!
//! ## Features
//! - `f64` precision
//! - multiple/arbitrary precision (very incomplete)
//! - low dependencies
//! - custom variable contexts
//! - custom function contexts
//! - builtin constants and functions (eg pi, sin, max)
//! - implicit multiplication
//! - utf8-ready
//! - support for multiple solutions
//! - complex numbers (very incomplete)
//!
//! ## Usage
//! There are three different ways to parse and evaluate an equation.
//!
//!  #### 1. With `eval()`
//! This function parses and evaluates a string all at once with the default context. There's also
//! an `eval_ctx()` function which takes a reference to a `Context` as well that will be used instead of
//! the default `Context`. The type parameter can be anything that implements the [`Num`](num::Num)
//! trait. Some `Num` types support more operations than others. More info about `Num`s can be found
//! in the [`Num`](num) module.
//!
//! ```rust
//! # let res =
//! mexprp::eval::<f64>("10 / (2 + 3)"); // Ok(Answer::Single(2.0))
//! # assert_eq!(res.unwrap(), mexprp::Answer::Single(2.0));
//! ```
//!
//! #### 2. With `Expression`
//! `Expression::parse()` parses a string into a tree representation (a `Term`). It can also be parsed
//! with a context with `parse_ctx()`, and it will store that context within it for future evaluations.
//! It can also be evaluated with a reference to any other context with `eval_ctx`. It's important to
//! ensure that the custom context contains any definitions the `Expression` depends on.
//!
//! ```rust
//! # use mexprp::{Expression, Answer};
//! let expr: Expression<f64> = Expression::parse("3 ^ 4 / 9").unwrap();
//! let res = expr.eval(); // Ok(Answer::Single(9.0))
//! # assert_eq!(res.unwrap(), Answer::Single(9.0));
//! ```
//!
//! #### 3. With `Term`
//! A `Term` is an `Expression`, but without the extra overhead of a context or the original string
//! representation stored with it. It is literally a tree representing the equation by it's operations.
//!
//! ```rust
//! # use mexprp::{Term, Answer};
//! let term: Term<f64> = Term::parse("10 ^ -3").unwrap();
//! let res = term.eval(); // Ok(Answer::Single(0.001))
//! # assert_eq!(res.unwrap(), Answer::Single(0.001));
//! ```
//!
//! ### Answer Types
//! Evaluating an expression will return an [`Answer`](answer::Answer) enum. An answer represents either
//! a single value, or multiple. The most notable example of an operation that results in multiple
//! answers is `sqrt()` which returns a positive and negative answer. Another obvious example is the
//! '±' operator. When implementing functions, it's important to handle each answer type when evaluating
//! the arguments. More info about that and helper methods for it can be found in the documentation
//! for the `Answer` enum.
//!
//! ### Multiple Precisions
//! MEXPRP supports evaluating expressions with different precisions with the [`Num`](num::Num) trait.
//! Currently supported number types are
//! - f64
//! - [`ComplexFloat`](num::ComplexFloat)
//! - [`ComplexRugRat`](num::ComplexRugRat) (using the rug crate)
//! - [`Rational`](::rug::Rational) (from the rug crate)
//!
//! However, the implementation for certain types is incomplete. Only the `f64` type fully implements
//! all of the operations. The others only implement a (small) subset of the functionality of the `Num`
//! trait, and return a `MathError::Unimplemented` when an unsupported operation is attempted. It is
//! hopeful that more functions will be implemented in the future, but some are extremely difficult
//! to implement for arbitrary precision numbers. `rug::Float` will probably become a `Num` soon to
//! allow for arbitrary precision numbers with more available operations, as soon as the design decision
//! of how to choose a precision is made. Most likely a configuration field will be added to the `Context`
//! struct which will include fields like the precision to use and whether to enable implicit multiplication
//! or not.
//!
//! To use another number type, change the type annotation(s) for your MEXPRP types.
//! ```rust
//! extern crate rug;
//! # extern crate mexprp;
//! use rug::Rational;
//! # use mexprp;
//! mexprp::eval::<Rational>("10/15"); // 2/3
//!```
//!
//! ```rust
//! # use mexprp::Expression;
//! # use mexprp::num::ComplexFloat;
//! let expr: Expression<ComplexFloat> = Expression::parse("(3 + 4i) × (6 - 3i)").unwrap();
//! let res = expr.eval(); // 30 + 15i
//! ```
//!
//! In case you don't want a dependency on `rug`, compile the MEXPRP without the `"rug"` feature.
//!
//! ### Using Contexts
//! You can evaluate expressions with custom variable and function definition's by defining a context.
//! When defining custom functions, it's important to remember to parse the expression with the custom
//! context, or else the parser will recognize your functions as variables instead. `Expression`s will
//! store the context you parse them with, but you have to evaluate `Term`s with a reference to a context
//! using `Term::eval_ctx`. For more info see the [`Context`](context::Context) struct.

#![deny(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(collapsible_if))]

#[macro_use]
extern crate failure;
#[cfg(feature = "rug")]
extern crate rug;

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
/// Number representation(s)
pub mod num;
/// Answer enum
pub mod answer;
#[cfg(test)]
mod tests;

pub use func::Func;
pub use expr::{Expression, Term};
pub use context::Context;
pub use errors::{EvalError, MathError, ParseError};
pub use num::Num;
pub use opers::Calculation;
pub use answer::Answer;

/// Parse and evaluate a string
pub fn eval<N: Num + 'static>(expr: &str) -> Result<Answer<N>, EvalError> {
	Ok(Term::parse(expr)?.eval()?)
}

/// Parse and evaluate a string with the given context
pub fn eval_ctx<N: Num + 'static>(expr: &str, ctx: &Context<N>) -> Result<Answer<N>, EvalError> {
	Ok(Term::parse_ctx(expr, ctx)?.eval_ctx(ctx)?)
}
