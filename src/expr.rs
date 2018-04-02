use std::fmt;

use op::*;
use opers::*;
use parse::*;
use errors::*;
use context::*;

use failure::Error;

/// The main representation of parsed equations. It is an operand that can contain an operation between
/// more of itself. This form is necessary for the equation to be evaluated.
#[derive(Debug)]
pub enum Term {
	/// A number
	Num(f64),
	/// An operation
	Operation(Box<Operate>),
	/// A function with the given arguments
	Function(String, Vec<Term>),
	/// A variable
	Var(String),
}

/// An enum that represents the equation as a token that can be several types of operands, or an operator.
/// This token has functions already parsed by their name and arguments, and has no parentheses, with
/// a Vec of tokens representing an expression within parentheses instead.
#[derive(Debug, Clone)]
enum Expr {
	/// A number
	Num(f64),
	/// An operator
	Op(Op),
	/// An expression within parentheses (a subexpression)
	Sub(Vec<Expr>),
	/// A variable
	Var(String),
	/// A function with these args
	Func(String, Vec<Vec<Expr>>),
}

impl Term {
	/// Evaluate the term with the given context
	pub fn eval(&self, ctx: &Context) -> Calculation {
		// Evaluate each possible term type
		match *self {
			Term::Num(num) => Ok(num),                   // Already evaluated
			Term::Operation(ref oper) => oper.eval(ctx), // Perform the operation with the given context
			Term::Function(ref name, ref args) => {
				// Execute the function if it exists
				if let Some(func) = ctx.funcs.get(name) {
					func.eval(args, ctx)
				} else {
					Err(MathError::UndefinedFunction { name: name.clone() })
				}
			}
			Term::Var(ref name) => {
				// Retrieve the value of the variable, if it exists
				if let Some(var) = ctx.vars.get(name) {
					var.eval(ctx)
				} else {
					Err(MathError::UndefinedVariable { name: name.clone() })
				}
			}
		}
	}
}

impl fmt::Display for Term {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "(term)")
	}
}

impl From<f64> for Term {
	fn from(t: f64) -> Self {
		Term::Num(t)
	}
}

impl From<Expression> for Term {
	fn from(t: Expression) -> Term {
		t.into_term()
	}
}

/// The main Expression struct. Contains only a Term and a String representing the original equation
/// requesting to be parsed. Will contain intermediate representations in the future. To just compile
/// to a pure representation of an expression, without anything extra, use Term.
#[derive(Debug)]
pub struct Expression {
	string: String,
	term: Term,
}

impl Expression {
	/// Parse a string into an expression
	pub fn parse(raw: &str) -> Result<Self, Error> {
		let ctx = Context::new();
		Self::parse_ctx(raw, &ctx)
	}

	/// Parse a string into an expression with the given context
	pub fn parse_ctx(raw: &str, ctx: &Context) -> Result<Self, Error> {
		let raw = raw.trim();
		debug!("Parsing '{}'", raw);
		let paren_tokens = get_tokens(raw)?;
		debug!("Paren tokens: {:?}", paren_tokens);
		let exprs = Self::paren_to_exprs(paren_tokens, ctx)?;
		debug!("Expressions: {:?}", exprs);
		let exprs = Self::insert_operators(exprs);
		debug!("Expressions: {:?}", exprs);
		let postfix = Self::tokenexprs_to_postfix(exprs);
		debug!("Postfix: {:?}", postfix);
		let term = Self::postfix_to_term(postfix)?;
		debug!("Term: {:?}", term);

		Ok(Self {
			string: raw.to_string(),
			term,
		})
	}

	/// Evaluate the expression
	pub fn eval(&self) -> Calculation {
		let ctx = Context::new();
		self.eval_ctx(&ctx)
	}

	/// Evaluate the expression with the given context
	pub fn eval_ctx(&self, ctx: &Context) -> Calculation {
		self.term.eval(ctx)
	}

	/// Convert ParenTokens to exprs. This function accomplishes two things at once. First, it decides
	/// if names are functions or variables depending on their context. Second, it splits the arguments
	/// of a function up by their commas, removing the need for a comma in the token representation.
	fn paren_to_exprs(raw: Vec<ParenToken>, ctx: &Context) -> Result<Vec<Expr>, Error> {
		trace!("Converting paren tokens to exprs");
		let mut mtokens = Vec::new();
		// Names that have yet to be decided
		let mut pending_name = None;

		for rt in raw {
			trace!("Have paren token: {:?}", rt);
			match rt {
				ParenToken::Num(num) => {
					// Names followed by numbers aren't functions
					if let Some(pending_name) = pending_name.take() {
						mtokens.push(Expr::Var(pending_name));
					}
					mtokens.push(Expr::Num(num));
				}
				ParenToken::Op(op) => {
					// Names followed by operators aren't functions
					if let Some(pending_name) = pending_name.take() {
						mtokens.push(Expr::Var(pending_name));
					}
					mtokens.push(Expr::Op(op));
				}
				ParenToken::Sub(sub) => {
					if let Some(name) = pending_name.take() {
						// If there was a name before this subexpression
						if ctx.funcs.contains_key(&name) {
							// If there's a function with the name
							mtokens.push(Expr::Func(name, Self::tokens_to_args(sub, ctx)?)); // Push as a function, with the args parsed
						} else {
							mtokens.push(Expr::Var(name)); // It's a variable
							mtokens.push(Expr::Sub(Self::paren_to_exprs(sub, ctx)?)); // Push the subexpression
						}
					} else {
						// Just push the subexpression
						mtokens.push(Expr::Sub(Self::paren_to_exprs(sub, ctx)?));
					}
				}
				ParenToken::Name(name) => {
					// Names followed by names aren't functions
					if let Some(pending_name) = pending_name.take() {
						mtokens.push(Expr::Var(pending_name));
					}
					pending_name = Some(name);
				}
				// There should be no commas here, they should have been removed during the Self::tokens_to_args calls
				// that happen when pushing a function.
				ParenToken::Comma => {
					return Err(UnexpectedToken {
						token: String::from(","),
					}.into())
				}
			}
		}

		if let Some(pending_name) = pending_name.take() {
			// Push a leftover pending name
			mtokens.push(Expr::Var(pending_name));
		}

		Ok(mtokens)
	}

	/// Converts a Vec of ParenTokens into a Vec of a Vec of Exprs, splitting them by commas and
	/// then parsing them into Exprs.
	fn tokens_to_args(raw: Vec<ParenToken>, ctx: &Context) -> Result<Vec<Vec<Expr>>, Error> {
		let args: Vec<&[ParenToken]> = raw.split(|ptoken| match *ptoken {
			ParenToken::Comma => true,
			_ => false,
		}).collect();

		debug!("Split args into: '{:?}'", args);

		let mut new = Vec::new();
		for arg in args {
			if arg.is_empty() {
				continue; // Ignore empty arguments (occurs when no arguments where passed to the function)
			}
			let arg = arg.to_vec();
			new.push(Self::paren_to_exprs(arg, ctx)?)
		}
		Ok(new)
	}

	/// Insert multiplication operations in between operands that are right next to each other
	#[cfg_attr(feature = "cargo-clippy", allow(redundant_closure))]
	fn insert_operators(mut raw: Vec<Expr>) -> Vec<Expr> {
		let mut i = 0;

		if raw.is_empty() {
			// Don't panic on empty input
			return Vec::new();
		}

		while i < raw.len() - 1 {
			if raw[i].is_operand() && raw[i + 1].is_operand() {
				raw.insert(i + 1, Expr::Op(Op::Mul));
			} else {
				i += 1;
			}
		}

		let mut new = Vec::new();
		for texpr in raw {
			match texpr {
				Expr::Sub(texprs) => new.push(Expr::Sub(Self::insert_operators(texprs))),
				Expr::Func(name, args) => new.push(Expr::Func(
					name,
					args.into_iter()
						.map(|texprs| Self::insert_operators(texprs))
						.collect(),
				)),
				t => new.push(t),
			}
		}

		new
	}

	/// Convert a vector of infix exprs to a postfix representations (shunting yard)
	fn tokenexprs_to_postfix(raw: Vec<Expr>) -> Vec<Expr> {
		fn recurse(raw: &[Expr]) -> Vec<Expr> {
			let mut stack = Vec::new();
			let mut ops: Vec<Op> = Vec::new();
			for texpr in raw {
				match *texpr {
					Expr::Num(num) => stack.push(Expr::Num(num)), // Push number onto the stack
					Expr::Op(ref op) => {
						while let Some(top_op) = ops.pop() {
							// Pop all operators with high enough precedence
							if (top_op.precedence() > op.precedence()) || (top_op.precedence() == op.precedence() && top_op.is_left_associative()) {
								stack.push(Expr::Op(top_op));
							} else {
								ops.push(top_op); // Put it back (not high enough precedence)
								break;
							}
						}
						ops.push(op.clone()); // Put the op on the stack
					}
					Expr::Var(ref name) => stack.push(Expr::Var(name.clone())), // Put the var on the stack
					Expr::Func(ref name, ref texprs_args) => stack.push(Expr::Func(name.clone(), {
						// Put the function on the stack
						let mut new_texprs_args = Vec::new();
						for texprs in texprs_args {
							new_texprs_args.push(recurse(texprs)); // Do shunting yard for all of it's arguments
						}
						new_texprs_args
					})),
					Expr::Sub(ref texprs) => stack.push(Expr::Sub(recurse(texprs))), // Push the subexpression onto the stack
				}
			}
			while let Some(op) = ops.pop() {
				// Push leftover operators onto stack
				stack.push(Expr::Op(op));
			}
			stack
		}

		recurse(&raw)
	}

	/// Parse a postfix token stream into a single term
	fn postfix_to_term(raw: Vec<Expr>) -> Result<Term, Expected> {
		let mut stack = Vec::new();
		for texpr in raw {
			match texpr {
				Expr::Num(num) => stack.push(Term::Num(num)), // Put num on the stack
				Expr::Op(op) => {
					// Push the operation with the last two operands on the stack
					macro_rules! pop {
						() => {
							match stack.pop() {
								Some(v) => v,
								None => return Err(Expected::Expression)
							}
						}
					}

					let oper: Box<Operate> = match op {
						Op::Add => Box::new(Add {
							b: pop!(),
							a: pop!(),
						}),
						Op::Sub => Box::new(Sub {
							b: pop!(),
							a: pop!(),
						}),
						Op::Mul => Box::new(Mul {
							b: pop!(),
							a: pop!(),
						}),
						Op::Div => Box::new(Div {
							b: pop!(),
							a: pop!(),
						}),
						Op::Pow => Box::new(Pow {
							b: pop!(),
							a: pop!(),
						}),
					};
					stack.push(Term::Operation(oper));
				}
				Expr::Sub(texprs) => {
					// Put subexpression on the stack
					stack.push(Self::postfix_to_term(texprs)?);
				}
				Expr::Var(name) => stack.push(Term::Var(name)), // Put var on the stack
				Expr::Func(name, args) => {
					// Put function with args converted to terms on the stack
					stack.push(Term::Function(name, {
						let mut new = Vec::new();
						for texprs in args {
							new.push(Self::postfix_to_term(texprs)?);
						}
						new
					}));
				}
			}
		}
		if stack.len() > 1 {
			// If there's leftovers on the stack, oops
			return Err(Expected::Operator);
		}

		if let Some(term) = stack.pop() {
			Ok(term)
		} else {
			Err(Expected::Expression)
		}
	}

	/// Convert the Expression into it's pure Term representation
	pub fn into_term(self) -> Term {
		self.term
	}
}

impl fmt::Display for Expression {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.string)
	}
}

impl Expr {
	/// Returns true if this expr is an operand (not an operator)
	fn is_operand(&self) -> bool {
		use self::Expr::*;
		match *self {
			Num(_) | Var(_) | Func(_, _) | Sub(_) => true,
			Op(_) => false,
		}
	}
}

/// The result of an evaluation (Result<f64, MathError>)
pub type Calculation = Result<f64, MathError>;
