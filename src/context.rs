use std::collections::HashMap;
use std::f64::consts;
use std::rc::Rc;
use std::fmt;

use term::Term;
use func::Func;
use num::Num;

/// A context holds values for variables and functions to be used in expressions. It is useful for both
/// parsing and evaluation expressions. During parsing, all names will be treated as variables unless
/// present in the Context the expression is being parsed with as functions at the time. The default
/// context (created with `new()`) contains basic functions and constants such as `sin`, `pi`, etc,
/// as well as the default configuration.
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
/// # use mexprp::{Expression, Context, Answer};
/// let mut context: Context<f64> = Context::new();
/// context.set_var("x", 4.0);
/// let expr = Expression::parse_ctx("4x", context).unwrap();
/// let res = expr.eval(); // Ok(Answer::Single(16.0))
/// # assert_eq!(res.unwrap(), Answer::Single(16.0));
/// ```
///
/// A custom function is anything that implements the [`func::Func`](::func::Func) trait. There's a
/// blanket impl of this trait allowing you to pass in any closure with the signature
/// `Fn(&[Term<Num>], &Context<Num>) -> Calculation<Num>`. You can also pass in a struct that implements `Func` manually
/// if you want more flexibility. The `Func` trait is just one method with the same signature previously
/// mentioned. Defining a custom function will most often look like this.
///
/// ```rust
/// # use mexprp::{Expression, Context, Term, Calculation, MathError, Answer};
/// let mut context: Context<f64> = Context::new();
/// context.set_func("sum", |args: &[Term<f64>], ctx: &Context<f64>| -> Calculation<f64> {
///     if args.is_empty() { return Err(MathError::IncorrectArguments) }
///
///     let mut sum = 0.0;
///     for arg in args {
///         let a = arg.eval_ctx(ctx)?;
///         match a {
///             Answer::Single(n) => sum += n,
///             Answer::Multiple(ns) => {
///                 for n in ns {
///                     sum += n;
///                 }
///             }
///         }
///     }
///     Ok(Answer::Single(sum))
/// });
/// let expr = Expression::parse_ctx("sum(5, 6, 7, 8)", context).unwrap();
/// let res = expr.eval(); // Ok(Answer::Single(26.0))
/// # assert_eq!(res.unwrap(), Answer::Single(26.0));
/// ```
///
/// The first argument of a custom function definition is a slice of `Term`s, which are the arguments
/// passed to the functions. The second argument is a reference to the `Context` the equation is being
/// evaluated with. It's important to remember to evaluate any arguments you receive with the reference
/// to the `Context` you received with `Term::eval_ctx()`. If the function is given arguments in an
/// incorrect way, return a `MathError::IncorrectArguments`. If any errors occur during evaluation, you
/// can try to find a `MathError` variant that fits or return `MathError::Other`.
///
/// ## Builtin
/// ### Constants
/// - pi
/// - e
/// - i
///
/// ### Functions
/// - sin
/// - cos
/// - tan
/// - asin
/// - acos
/// - atan
/// - atant (atan2)
/// - floor
/// - ceil
/// - round
/// - sqrt
/// - max
/// - min
#[derive(Clone)]
pub struct Context<N: Num> {
	/// HashMap of variables
	pub vars: HashMap<String, Term<N>>,
	/// HashMap of functions
	pub funcs: HashMap<String, Rc<Func<N>>>,
	/// The configuration used when evaluating expressions
	pub cfg: Config,
}

/// Struct that holds configuration values used when evaluating expressions
#[derive(Debug, Clone)]
pub struct Config {
	/// Whether or not to automatically insert multiplication signs between two operands (default = true)
	pub implicit_multiplication: bool,
	/// The precision to be used for arbitrary precision floating point numbers (default = 53)
	pub precision: u32,
	/// Whether or not sqrt should return the positive and negative values
	pub sqrt_both: bool,
}

impl<N: Num + 'static> Context<N> {
	/// Returns a default Context
	pub fn new() -> Self {
		use self::funcs::*;

		let mut ctx: Context<N> = Context::empty();
		
		let empty = Context::empty();

		ctx.set_var("pi", N::from_f64(consts::PI, &empty).unwrap());
		ctx.set_var("e", N::from_f64(consts::E, &empty).unwrap());
		ctx.set_var("i", N::from_f64_complex((0.0, 1.0), &empty).unwrap());

		ctx.funcs.insert("sin".to_string(), Rc::new(Sin));
		ctx.funcs.insert("cos".to_string(), Rc::new(Cos));
		ctx.funcs.insert("max".to_string(), Rc::new(Max));
		ctx.funcs.insert("min".to_string(), Rc::new(Min));
		ctx.funcs.insert("sqrt".to_string(), Rc::new(Sqrt));
		ctx.funcs.insert("nrt".to_string(), Rc::new(Nrt));
		ctx.funcs.insert("tan".to_string(), Rc::new(Tan));
		ctx.funcs.insert("abs".to_string(), Rc::new(Abs));
		ctx.funcs.insert("asin".to_string(), Rc::new(Asin));
		ctx.funcs.insert("acos".to_string(), Rc::new(Acos));
		ctx.funcs.insert("atan".to_string(), Rc::new(Atan));
		ctx.funcs.insert("atant".to_string(), Rc::new(Atan2));
		ctx.funcs.insert("floor".to_string(), Rc::new(Floor));
		ctx.funcs.insert("round".to_string(), Rc::new(Round));
		ctx.funcs.insert("log".to_string(), Rc::new(Log));

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
	
	/// Creates an empty `Context` with the default config
	pub fn empty() -> Self {
		Context {
			vars: HashMap::new(),
			funcs: HashMap::new(),
			cfg: Config::new(),
		}
	}
}

impl Config {
	/// Create a new config with the default values
	pub fn new() -> Self {
		Config {
			implicit_multiplication: true,
			precision: 53,
			sqrt_both: true,
		}
	}
}

impl Default for Config {
	fn default() -> Self {
		Self::new()
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
	use std::cmp::Ordering;

	use context::Context;
	use term::Term;
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
			
			a.unop(|a| Num::sin(a, ctx))
		}
	}
	
	pub struct Cos;
	impl<N: Num + 'static> Func<N> for Cos {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;
			
			a.unop(|a| Num::cos(a, ctx))
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
				if Num::tryord(arg, &max, ctx)? == Ordering::Greater {
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
				if Num::tryord(arg, &min, ctx)? == Ordering::Less {
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

			a.unop(|a| Num::sqrt(a, ctx))
		}
	}
	
	pub struct Nrt;
	impl<N: Num + 'static> Func<N> for Nrt {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 2 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;
			let b = args[1].eval_ctx(ctx)?;
			
			a.op(&b, |a, b| Num::nrt(a, b, ctx))
		}
	}
	
	pub struct Abs;
	impl<N: Num + 'static> Func<N> for Abs {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;
			
			a.unop(|a| Num::abs(a, ctx))
		}
	}
	
	pub struct Tan;
	impl<N: Num + 'static> Func<N> for Tan {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;
			
			a.unop(|a| Num::tan(a, ctx))
		}
	}
	
	pub struct Asin;
	impl<N: Num + 'static> Func<N> for Asin {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;
			
			a.unop(|a| Num::asin(a, ctx))
		}
	}
	
	pub struct Acos;
	impl<N: Num + 'static> Func<N> for Acos {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;
			
			a.unop(|a| Num::acos(a, ctx))
		}
	}
	
	pub struct Atan;
	impl<N: Num + 'static> Func<N> for Atan {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;
			
			a.unop(|a| Num::atan(a, ctx))
		}
	}
	
	pub struct Atan2;
	impl<N: Num + 'static> Func<N> for Atan2 {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 2 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;
			let b = args[1].eval_ctx(ctx)?;
			
			a.op(&b, |a, b| Num::atan2(a, b, ctx))
		}
	}
	
	pub struct Floor;
	impl<N: Num + 'static> Func<N> for Floor {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;
			
			a.unop(|a| Num::floor(a, ctx))
		}
	}
	
	pub struct Ceil;
	impl<N: Num + 'static> Func<N> for Ceil {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;
			
			a.unop(|a| Num::ceil(a, ctx))
		}
	}
	
	pub struct Round;
	impl<N: Num + 'static> Func<N> for Round {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 1 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;
			
			a.unop(|a| Num::round(a, ctx))
		}
	}
	
	pub struct Log;
	impl<N: Num + 'static> Func<N> for Log {
		fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
			if args.len() != 2 {
				return Err(MathError::IncorrectArguments);
			}
			
			let a = args[0].eval_ctx(ctx)?;
			let b = args[1].eval_ctx(ctx)?;
			
			a.op(&b, |a, b| Num::log(a, b, ctx))
		}
	}
}
