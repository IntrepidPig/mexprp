use std::collections::HashMap;
use std::f64::consts;

use expr::Term;
use func::Func;

/// A context holds values for variables and functions to be used in expressions. It is useful for both
/// parsing and evaluation expressions. During parsing, all names will be treated as variables unless
/// present in the Context the expression is being parsed with as functions at the time.
///
/// Internally, a context is just two HashMaps, one for variables and one for functions. The only thing
/// truly special about it (for now at least) is the defualt value when calling new(). This Context contains
/// all basic functions and values expected to be included such as sin(), cos(), sqrt(), pi, etc. This
/// is the Context all expressions are parsed and evaluated with if no other one is present.
pub struct Context {
	/// HashMap of variables
	pub vars: HashMap<String, Term>,
	/// HashMap of functions
	pub funcs: HashMap<String, Box<Func>>,
}

impl Context {
	/// Returns a default Context
	pub fn new() -> Self {
		use self::funcs::*;

		let mut ctx = Context {
			vars: HashMap::new(),
			funcs: HashMap::new(),
		};

		ctx.set_var("pi", consts::PI);
		ctx.set_var("e", consts::E);

		ctx.funcs.insert("sin".to_string(), Box::new(Sin));
		ctx.funcs.insert("cos".to_string(), Box::new(Cos));
		ctx.funcs.insert("max".to_string(), Box::new(Max));
		ctx.funcs.insert("min".to_string(), Box::new(Min));
		ctx.funcs.insert("sqrt".to_string(), Box::new(Sqrt));

		ctx
	}

	/// Add a variable definition to the context, replacing any existing one with the same name
	pub fn set_var<T: Into<Term>>(&mut self, name: &str, val: T) {
		self.vars.insert(name.to_string(), val.into());
	}

	/// Add a function definition to the context, replacing any existing one with the same name
	pub fn set_func<F: Func + 'static>(&mut self, name: &str, func: F) {
		self.funcs.insert(name.to_string(), Box::new(func));
	}
}

impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}

pub(in context) mod funcs {
	use std::cmp::{Ordering, PartialOrd};

	use context::Context;
	use expr::{Calculation, Term};
	use errors::MathError;
	use func::Func;

	pub struct Sin;
	impl Func for Sin {
		fn eval(&self, args: &[Term], ctx: &Context) -> Calculation {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}
			Ok(args[0].eval(ctx)?.sin())
		}
	}

	pub struct Cos;
	impl Func for Cos {
		fn eval(&self, args: &[Term], ctx: &Context) -> Calculation {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}
			Ok(args[0].eval(ctx)?.cos())
		}
	}

	pub struct Max;
	impl Func for Max {
		fn eval(&self, args: &[Term], ctx: &Context) -> Calculation {
			if args.is_empty() {
				return Err(MathError::IncorrectArguments);
			}
			let mut max = args[0].eval(ctx)?;
			for arg in &args[1..args.len()] {
				let arg = arg.eval(ctx)?;
				if float_cmp(arg, max)? == Ordering::Greater {
					max = arg;
				}
			}
			Ok(max)
		}
	}

	pub struct Min;
	impl Func for Min {
		fn eval(&self, args: &[Term], ctx: &Context) -> Calculation {
			if args.is_empty() {
				return Err(MathError::IncorrectArguments);
			}
			let mut max = args[0].eval(ctx)?;
			for arg in &args[1..args.len()] {
				let arg = arg.eval(ctx)?;
				if float_cmp(arg, max)? == Ordering::Less {
					max = arg;
				}
			}
			Ok(max)
		}
	}

	pub struct Sqrt;
	impl Func for Sqrt {
		fn eval(&self, args: &[Term], ctx: &Context) -> Calculation {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}

			Ok(args[0].eval(ctx)?.sqrt())
		}
	}

	/// Compares two floats. Errors if either is NaN. Infinity is greater than anything except equal
	/// to infinity. Negative infinity is less than anything except equal to negative infinity.
	fn float_cmp(a: f64, b: f64) -> Result<Ordering, MathError> {
		if a.is_nan() || b.is_nan() {
			return Err(MathError::NaN);
		}
		if a.is_infinite() {
			if a.is_sign_positive() {
				if b.is_infinite() && b.is_sign_positive() {
					Ok(Ordering::Equal)
				} else {
					Ok(Ordering::Greater)
				}
			} else {
				if b.is_infinite() && b.is_sign_negative() {
					Ok(Ordering::Equal)
				} else {
					Ok(Ordering::Less)
				}
			}
		} else if b.is_infinite() {
			Ok(float_cmp(b, a)?.reverse())
		} else {
			Ok(a.partial_cmp(&b).unwrap())
		}
	}
}
