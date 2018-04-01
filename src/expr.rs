use std::fmt;

use op::*;
use opers::*;
use parse::*;
use errors::*;
use context::*;

use failure::Error;

#[derive(Debug)]
pub enum Term {
	Num(f64),
	Operation(Box<Operate>),
	Function(String, Vec<Term>),
	Var(String),
}

#[derive(Debug, Clone)]
enum Expr {
	Num(f64),
	Op(Op),
	Sub(Vec<Expr>),
	Var(String),
	Func(String, Vec<Vec<Expr>>),
}

impl Term {
	pub fn eval(&self, ctx: &Context) -> Calculation {
		match *self {
			Term::Num(num) => Ok(num),
			Term::Operation(ref oper) => oper.eval(ctx),
			Term::Function(ref name, ref args) => {
				if let Some(func) = ctx.funcs.get(name) {
					func.eval(args, ctx)
				} else {
					Err(MathError::UndefinedFunction { name: name.clone() })
				}
			}
			Term::Var(ref name) => {
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

#[derive(Debug)]
pub struct Expression {
	string: String,
	term: Term,
}

impl Expression {
	pub fn parse(raw: &str) -> Result<Self, Error> {
		let ctx = Context::new();
		Self::parse_ctx(raw, &ctx)
	}

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

	pub fn eval(&self) -> Calculation {
		let ctx = Context::new();
		self.eval_ctx(&ctx)
	}

	pub fn eval_ctx(&self, ctx: &Context) -> Calculation {
		self.term.eval(ctx)
	}

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
						if ctx.funcs.contains_key(&name) {
							mtokens.push(Expr::Func(name, Self::tokens_to_args(sub, ctx)?));
						} else {
							mtokens.push(Expr::Var(name));
							mtokens.push(Expr::Sub(Self::paren_to_exprs(sub, ctx)?));
						}
					} else {
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
				ParenToken::Comma => return Err(UnexpectedToken(String::from(",")).into()),
			}
		}

		if let Some(pending_name) = pending_name.take() {
			// Push a leftover pending name
			mtokens.push(Expr::Var(pending_name));
		}

		Ok(mtokens)
	}

	fn tokens_to_args(raw: Vec<ParenToken>, ctx: &Context) -> Result<Vec<Vec<Expr>>, Error> {
		let args: Vec<&[ParenToken]> = raw.split(|ptoken| match ptoken {
			&ParenToken::Comma => true,
			_ => false,
		}).collect();

		debug!("Split args into: '{:?}'", args);

		let mut new = Vec::new();
		for arg in args {
			let arg = arg.to_vec();
			new.push(Self::paren_to_exprs(arg, ctx)?)
		}
		Ok(new)
	}

	/// Insert multiplication operations in between operands that are right next to each other
	fn insert_operators(mut raw: Vec<Expr>) -> Vec<Expr> {
		let mut i = 0;
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

	/// Convert a vector of infix tokenexprs to a postfix representations
	fn tokenexprs_to_postfix(raw: Vec<Expr>) -> Vec<Expr> {
		fn recurse(raw: &[Expr]) -> Vec<Expr> {
			let mut stack = Vec::new();
			let mut ops: Vec<Op> = Vec::new();
			for texpr in raw {
				match *texpr {
					Expr::Num(num) => stack.push(Expr::Num(num)),
					Expr::Op(ref op) => {
						while let Some(top_op) = ops.pop() {
							if top_op.precedence() > op.precedence() {
								stack.push(Expr::Op(top_op));
							} else if top_op.precedence() == op.precedence() && top_op.is_left_associative() {
								stack.push(Expr::Op(top_op));
							} else {
								ops.push(top_op); // Put it back
								break;
							}
						}
						ops.push(op.clone());
					}
					Expr::Var(ref name) => stack.push(Expr::Var(name.clone())),
					Expr::Func(ref name, ref texprs_args) => stack.push(Expr::Func(name.clone(), {
						let mut new_texprs_args = Vec::new();
						for texprs in texprs_args {
							new_texprs_args.push(recurse(texprs));
						}
						new_texprs_args
					})),
					Expr::Sub(ref texprs) => stack.push(Expr::Sub(recurse(texprs))),
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

	fn postfix_to_term(raw: Vec<Expr>) -> Result<Term, Expected> {
		let mut stack = Vec::new();
		for texpr in raw {
			match texpr {
				Expr::Num(num) => stack.push(Term::Num(num)),
				Expr::Op(op) => {
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
					stack.push(Self::postfix_to_term(texprs)?);
				}
				Expr::Var(name) => stack.push(Term::Var(name)),
				Expr::Func(name, args) => {
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
			return Err(Expected::Operator);
		}
		Ok(stack.pop().unwrap_or(Term::Num(0.0))) // hmmm....
	}

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
	pub fn is_operand(&self) -> bool {
		use self::Expr::*;
		match *self {
			Num(_) | Var(_) | Func(_, _) | Sub(_) => true,
			Op(_) => false,
		}
	}
}

pub type Calculation = Result<f64, MathError>;
