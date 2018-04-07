use std::fmt;
use std::rc::Rc;

use op::*;
use opers::*;
use parse::*;
use errors::*;
use context::*;
use num::*;
use answer::*;
use expr::*;

/// The main representation of parsed equations. It is an operand that can contain an operation between
/// more of itself. This form is the only one that can be directly evaluated. Does not include it's own
/// context.
#[derive(Debug, Clone)]
pub enum Term<N: Num> {
	/// A number
	Num(Answer<N>),
	/// An operation
	Operation(Rc<Operate<N>>),
	/// A function with the given arguments
	Function(String, Vec<Term<N>>),
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

impl<N: Num + 'static> Term<N> {
	/// Parse a string into an expression
	pub fn parse(raw: &str) -> Result<Self, ParseError> {
		let ctx = Context::new();
		Self::parse_ctx(raw, &ctx)
	}
	
	/// Parse a string into an expression with the given context
	pub fn parse_ctx(raw: &str, ctx: &Context<N>) -> Result<Self, ParseError> {
		let raw = raw.trim();
		let paren_tokens = get_tokens(raw)?;
		let exprs = paren_to_exprs(paren_tokens, ctx)?;
		let exprs = if ctx.cfg.implicit_multiplication {
			insert_operators(exprs)
		} else {
			exprs
		};
		let postfix = tokenexprs_to_postfix(exprs);
		let term = postfix_to_term(postfix, ctx)?;
		
		Ok(term)
	}
	
	/// Evaluate the term with the default context
	pub fn eval(&self) -> Calculation<N> {
		let ctx = Context::new();
		self.eval_ctx(&ctx)
	}
	
	/// Evaluate the term with the given context
	pub fn eval_ctx(&self, ctx: &Context<N>) -> Calculation<N> {
		// Evaluate each possible term type
		match *self {
			Term::Num(ref num) => Ok(num.clone()),                   // Already evaluated
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
					var.eval_ctx(ctx)
				} else {
					Err(MathError::UndefinedVariable { name: name.clone() })
				}
			}
		}
	}
	
	/// Express this term as a string
	pub fn to_string(&self) -> String {
		match *self {
			Term::Num(ref num) => format!("{}", num),
			Term::Operation(ref op) => format!("{}", op.to_string()),
			Term::Function(ref name, ref args) => format!("{}({})", name, {
				let mut buf = String::new();
				for (i, arg) in args.iter().enumerate() {
					buf.push_str(&arg.to_string());
					if i + 1 < args.len() {
						buf.push_str(", ");
					}
				}
				buf
			}),
			Term::Var(ref name) => format!("{}", name),
		}
	}
}

impl<N: Num + 'static> fmt::Display for Term<N> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl<N: Num> From<Expression<N>> for Term<N> {
	fn from(t: Expression<N>) -> Term<N> {
		t.term
	}
}

impl<N: Num> From<N> for Term<N> {
	fn from(t: N) -> Term<N> {
		Term::Num(Answer::Single(t))
	}
}

impl<N: Num> From<Answer<N>> for Term<N> {
	fn from(t: Answer<N>) -> Term<N> {
		Term::Num(t)
	}
}

/// Convert ParenTokens to exprs. This function accomplishes two things at once. First, it decides
/// if names are functions or variables depending on their context. Second, it splits the arguments
/// of a function up by their commas, removing the need for a comma in the token representation.
fn paren_to_exprs<N: Num + 'static>(raw: Vec<ParenToken>, ctx: &Context<N>) -> Result<Vec<Expr>, ParseError> {
	let mut mtokens = Vec::new();
	// Names that have yet to be decided
	let mut pending_name = None;
	
	for rt in raw {
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
				// If there was a name before this subexpression
				if let Some(name) = pending_name.take() {
					// If we allow implicit multiplication it might be a variable
					if ctx.cfg.implicit_multiplication {
						if ctx.funcs.contains_key(&name) {
							// If there's a function with the name
							mtokens.push(Expr::Func(name, tokens_to_args(sub, ctx)?)); // Push as a function, with the args parsed
						} else {
							mtokens.push(Expr::Var(name)); // It's a variable
							mtokens.push(Expr::Sub(paren_to_exprs(sub, ctx)?)); // Push the subexpression
						}
					} else { // If not then it's definitely a function
						mtokens.push(Expr::Func(name, tokens_to_args(sub, ctx)?)); // Push as a function, with the args parsed
					}
				} else {
					// Just push the subexpression
					mtokens.push(Expr::Sub(paren_to_exprs(sub, ctx)?));
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
				return Err(ParseError::UnexpectedToken {
					token: String::from(","),
				})
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
fn tokens_to_args<N: Num + 'static>(raw: Vec<ParenToken>, ctx: &Context<N>) -> Result<Vec<Vec<Expr>>, ParseError> {
	let args: Vec<&[ParenToken]> = raw.split(|ptoken| match *ptoken {
		ParenToken::Comma => true,
		_ => false,
	}).collect();
	
	let mut new = Vec::new();
	for arg in args {
		if arg.is_empty() {
			continue; // Ignore empty arguments (occurs when no arguments where passed to the function)
		}
		let arg = arg.to_vec();
		new.push(paren_to_exprs(arg, ctx)?)
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
			raw.insert(i + 1, Expr::Op(Op::In(In::Mul)));
		} else {
			match raw[i] {
				Expr::Op(Op::Post(_)) => {
					if raw[i + 1].is_operand() {
						raw.insert(i + 1, Expr::Op(Op::In(In::Mul)));
					}
				}
				_ => {}
			}
			i += 1;
		}
	}
	
	let mut new = Vec::new();
	for texpr in raw {
		match texpr {
			Expr::Sub(texprs) => new.push(Expr::Sub(insert_operators(texprs))),
			Expr::Func(name, args) => new.push(Expr::Func(
				name,
				args.into_iter()
						.map(|texprs| insert_operators(texprs))
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
						if op.should_shunt(&top_op.clone()) {
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
fn postfix_to_term<N: Num + 'static>(raw: Vec<Expr>, ctx: &Context<N>) -> Result<Term<N>, ParseError> {
	let mut stack = Vec::new();
	for texpr in raw {
		match texpr {
			Expr::Num(num) => stack.push(Term::Num(N::from_f64(num, ctx).unwrap())), // Put num on the stack
			Expr::Op(op) => {
				// Push the operation with the last two operands on the stack
				macro_rules! pop {
						() => {
							match stack.pop() {
								Some(v) => v,
								None => return Err(ParseError::Expected {
									expected: Expected::Expression
								}),
							}
						}
					}
				
				let oper: Rc<Operate<N>> = match op {
					Op::In(op) => match op {
						In::Add => Rc::new(Add {
							b: pop!(),
							a: pop!(),
						}),
						In::Sub => Rc::new(Sub {
							b: pop!(),
							a: pop!(),
						}),
						In::Mul => Rc::new(Mul {
							b: pop!(),
							a: pop!(),
						}),
						In::Div => Rc::new(Div {
							b: pop!(),
							a: pop!(),
						}),
						In::Pow => Rc::new(Pow {
							b: pop!(),
							a: pop!(),
						}),
					},
					Op::Pre(op) => match op {
						Pre::Neg => Rc::new(Neg { a: pop!() }),
						Pre::Pos => Rc::new(Pos { a: pop!() }),
					},
					Op::Post(op) => match op {
						Post::Fact => Rc::new(Fact { a: pop!() }),
						Post::Percent => Rc::new(Percent { a: pop!() }),
					},
				};
				stack.push(Term::Operation(oper));
			}
			Expr::Sub(texprs) => {
				// Put subexpression on the stack
				stack.push(postfix_to_term(texprs, ctx)?);
			}
			Expr::Var(name) => stack.push(Term::Var(name)), // Put var on the stack
			Expr::Func(name, args) => {
				// Put function with args converted to terms on the stack
				stack.push(Term::Function(name, {
					let mut new = Vec::new();
					for texprs in args {
						new.push(postfix_to_term(texprs, ctx)?);
					}
					new
				}));
			}
		}
	}
	if stack.len() > 1 {
		// If there's leftovers on the stack, oops
		return Err(ParseError::Expected {
			expected: Expected::Operator,
		});
	}
	
	if let Some(term) = stack.pop() {
		Ok(term)
	} else {
		Err(ParseError::Expected {
			expected: Expected::Expression,
		})
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
