use std::collections::HashMap;

use expr::Term;
use func::Func;
use errors::NameInUse;

pub struct Context {
	pub vars: HashMap<String, Term>,
	pub funcs: HashMap<String, Box<Func>>,
}

impl Context {
	pub fn new() -> Self {
		use self::funcs::*;

		let mut ctx = Context {
			vars: HashMap::new(),
			funcs: HashMap::new(),
		};

		ctx.funcs.insert("sin".to_string(), Box::new(Sin));
		ctx.funcs.insert("cos".to_string(), Box::new(Cos));
		ctx.funcs.insert("max".to_string(), Box::new(Max));
		ctx.funcs.insert("min".to_string(), Box::new(Min));

		ctx
	}

	pub fn add_var<T: Into<Term>>(&mut self, name: &str, val: T) -> Result<(), NameInUse> {
		if self.funcs.contains_key(name) {
			return Err(NameInUse {
				name: name.to_string(),
			});
		}

		self.vars.insert(name.to_string(), val.into());
		Ok(())
	}

	pub fn add_func<F: Func + 'static>(&mut self, name: &str, func: F) -> Result<(), NameInUse> {
		if self.vars.contains_key(name) {
			return Err(NameInUse {
				name: name.to_string(),
			});
		}

		self.funcs.insert(name.to_string(), Box::new(func));
		Ok(())
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
			if !(args.len() > 0) {
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
			if !(args.len() > 0) {
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
