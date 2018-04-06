use std::collections::HashMap;
use std::f64::consts;
use std::rc::Rc;
use std::fmt;

use answer::Answer;
use expr::Term;
use func::Func;
use num::Num;

/// A context holds values for variables and functions to be used in expressions. It is useful for both
/// parsing and evaluation expressions. During parsing, all names will be treated as variables unless
/// present in the Context the expression is being parsed with as functions at the time.
///
/// Internally, a context is just two HashMaps, one for variables and one for functions. The only thing
/// truly special about it (for now at least) is the default value when calling new(). This Context contains
/// all basic functions and values expected to be included such as sin(), cos(), sqrt(), pi, etc. This
/// is the Context all expressions are parsed and evaluated with if no other one is present.
///
/// Contexts are used differently with `Term`s and `Expression`s. With `Term`s, more decisions are left
/// up to the user. `Term`s can be parsed with a reference to a context and evaluated with a reference
/// to a context. They never store any contextual information themselves. `Expression`s can be parsed
/// with an instance of a `Context` and will then store that `Context` within them. They can still be
/// evaluated with a reference to any other `Context`.
///
/// To define a custom variable, use `set_var`. It takes anything that implements `Into<Term>`, so you
/// can pass in just an `f64` if you want.
///
/// ```rust
/// # use mexprp::{Expression, Context};
/// let mut context = Context::new();
/// context.set_var("x", 4.0);
/// let expr = Expression::parse_ctx("4x", context).unwrap();
/// let res = expr.eval(); // Ok(16.0)
/// # assert_eq!(res.unwrap(), 16.0);
/// ```
///
/// A custom function is anything that implements the [`func::Func`](func::Func) trait. There's a
/// blanket impl of this trait allowing you to pass in any closure with the signature
/// `Fn(&[Term], &Context) -> Calculation`. You can also pass in a struct that implements `Func` manually
/// if you want more flexibility. The `Func` trait is just one method with the same signature previously
/// mentioned. Defining a custom function will most often look like this.
///
/// ```rust
/// # use mexprp::{Expression, Context, Term, Calculation, MathError};
/// let mut context = Context::new();
/// context.set_func("sum", |args: &[Term], ctx: &Context| -> Calculation {
///     if args.is_empty() { return Err(MathError::IncorrectArguments) }
///
///     let mut sum = 0.0;
///     for arg in args {
///         sum += arg.eval_ctx(ctx)?;
///     }
///     Ok(sum)
/// });
/// let expr = Expression::parse_ctx("sum(5, 6, 7, 8)", context).unwrap();
/// let res = expr.eval(); // Ok(26.0)
/// # assert_eq!(res.unwrap(), 26.0);
/// ```
///
/// The first argument of a custom function definition is a slice of `Term`s, which are the arguments
/// passed to the functions. The second argument is a reference to the `Context` the equation is being
/// evaluated with. It's important to remember to evaluate any arguments you receive with the reference
/// to the `Context` you received with `Term::eval_ctx()`. If the function is given arguments in an
/// incorrect way, return a `MathError::IncorrectArguments`. If any errors occur during evaluation, you
/// can try to find a `MathError` variant that fits or return `MathError::Other`.
#[derive(Clone)]
pub struct Context<N: Num> {
	/// HashMap of variables
	pub vars: HashMap<String, Term<N>>,
	/// HashMap of functions
	pub funcs: HashMap<String, Rc<Func<N>>>,
}

impl<N: Num + 'static> Context<N> {
	/// Returns a default Context
	pub fn new() -> Self {
		use self::funcs::*;

		let mut ctx: Context<N> = Context {
			vars: HashMap::new(),
			funcs: HashMap::new(),
		};

		ctx.set_var("pi", N::from_f64(consts::PI).unwrap());
		ctx.set_var("e", N::from_f64(consts::E).unwrap());
		ctx.set_var("i", N::from_f64_complex((0.0, 1.0)).unwrap());

		ctx.funcs.insert("sin".to_string(), Rc::new(Sin));
		ctx.funcs.insert("cos".to_string(), Rc::new(Cos));
		ctx.funcs.insert("max".to_string(), Rc::new(Max));
		ctx.funcs.insert("min".to_string(), Rc::new(Min));
		ctx.funcs.insert("sqrt".to_string(), Rc::new(Sqrt));

		ctx
	}

	/// Add a variable definition to the context, replacing any existing one with the same name
	pub fn set_var<T: Into<Term<N>>>(&mut self, name: &str, val: T) {
		self.vars.insert(name.to_string(), val.into());
	}

	/// Add a function definition to the context, replacing any existing one with the same name
	pub fn set_func<F: Func<N> + 'static>(&mut self, name: &str, func: F) {
		self.funcs.insert(name.to_string(), Rc::new(func));
	}
}

impl<N: Num + 'static> Default for Context<N> {
	fn default() -> Self {
		Self::new()
	}
}

impl<N: Num> fmt::Debug for Context<N> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Context {{ vars: {:?}, funcs: {{{}}} }}", self.vars, {
			let mut output = String::new();
			for (i, key) in self.funcs.keys().enumerate() {
				output.push_str(key);
				if i + 1 < self.funcs.len() {
					output.push_str(", ");
				}
			}
			output
		})
	}
}

pub(in context) mod funcs {
	use std::cmp::{Ordering, PartialOrd};

	use context::Context;
	use expr::Term;
	use errors::MathError;
	use func::Func;
	use opers::Calculation;
	use num::Num;
	use answer::Answer;

	pub struct Sin;
	impl<N: Num + 'static> Func<N> for Sin {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;
			
			a.unop(|a| Num::sin(a))
		}
	}
	
	pub struct Cos;
	impl<N: Num + 'static> Func<N> for Cos {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;
			
			a.unop(|a| Num::cos(a))
		}
	}
	
	pub struct Max;
	impl<N: Num + 'static> Func<N> for Max {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.is_empty() {
				return Err(MathError::IncorrectArguments);
			}
			let mut extra = Vec::new();
			let mut max = match args[0].eval_ctx(ctx)? {
				Answer::Single(n) => n,
				Answer::Multiple(mut ns) => {
					let one = ns.pop().unwrap();
					extra = ns;
					one
				}
			};
			
			// Try to evaluate the arguments
			let args: Vec<Answer<N>> = args.iter().map(|term| term.eval_ctx(ctx)).collect::<Result<Vec<Answer<N>>, MathError>>()?;
			let mut new_args = Vec::new();
			// Push each answer of each argument to `new_args`
			for a in args {
				match a {
					Answer::Single(n) => new_args.push(n),
					Answer::Multiple(mut ns) => new_args.append(&mut ns),
				}
			}
			// For every argument as well as the extraneous solutions from the first one
			for arg in new_args[1..new_args.len()].iter().chain(extra.iter()) {
				if Num::tryord(arg, &max)? == Ordering::Greater {
					max = arg.clone();
				}
			}
			Ok(Answer::Single(max))
		}
	}
	
	pub struct Min;
	impl<N: Num + 'static> Func<N> for Min {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.is_empty() {
				return Err(MathError::IncorrectArguments);
			}
			let mut extra = Vec::new();
			let mut min = match args[0].eval_ctx(ctx)? {
				Answer::Single(n) => n,
				Answer::Multiple(mut ns) => {
					let one = ns.pop().unwrap();
					extra = ns;
					one
				}
			};
			
			// Try to evaluate the arguments
			let args: Vec<Answer<N>> = args.iter().map(|term| term.eval_ctx(ctx)).collect::<Result<Vec<Answer<N>>, MathError>>()?;
			let mut new_args = Vec::new();
			// Push each answer of each argument to `new_args`
			for a in args {
				match a {
					Answer::Single(n) => new_args.push(n),
					Answer::Multiple(mut ns) => new_args.append(&mut ns),
				}
			}
			// For every argument as well as the extraneous solutions from the first one
			for arg in new_args[1..new_args.len()].iter().chain(extra.iter()) {
				if Num::tryord(arg, &min)? == Ordering::Less {
					min = arg.clone();
				}
			}
			Ok(Answer::Single(min))
		}
	}

	pub struct Sqrt;
	impl<N: Num + 'static> Func<N> for Sqrt {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;

			a.unop(|a| Num::sqrt(a))
		}
	}
}
