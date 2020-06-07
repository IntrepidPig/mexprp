use std::fmt;



use crate::opers::*;

use crate::errors::*;
use crate::context::*;
use crate::num::*;

use crate::term::*;

/// The main Expression struct. Contains the string that was originally requested to be parsed, the
/// context the Expression was parsed with, and the Term the raw form was parsed as. For just the
/// parsed version of the expression, use the Term enum.
#[derive(Debug)]
pub struct Expression<N: Num> {
	/// The original string passed into this expression
	pub string: String,
	/// Context the expression was parsed with
	pub ctx: Context<N>,
	/// The term this string has been parsed as
	pub term: Term<N>,
}

impl<N: Num + 'static> Expression<N> {
	/// Parse a string into an expression
	pub fn parse(raw: &str) -> Result<Self, ParseError> {
		let ctx = Context::new();
		Self::parse_ctx(raw, ctx)
	}

	/// Parse a string into an expression with the given context
	pub fn parse_ctx(raw: &str, ctx: Context<N>) -> Result<Self, ParseError> {
		let raw = raw.trim();
		let term = Term::parse_ctx(raw, &ctx)?;

		Ok(Self {
			string: raw.to_string(),
			ctx,
			term,
		})
	}

	/// Evaluate the expression
	pub fn eval(&self) -> Calculation<N> {
		self.eval_ctx(&self.ctx)
	}

	/// Evaluate the expression with the given context
	pub fn eval_ctx(&self, ctx: &Context<N>) -> Calculation<N> {
		self.term.eval_ctx(ctx)
	}
}

impl<N: Num> fmt::Display for Expression<N> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.string)
	}
}
