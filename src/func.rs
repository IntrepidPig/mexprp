use expr::{Calculation, Term};
use context::Context;

/// Implemented by functions defined in a context
pub trait Func {
	/// Evaluate the function in this context with the given arguments. When implementing,
	/// simply evaluate the arguments with the context and return an Err(MathError::IncorrectArguments)
	/// if there are too many or too few.
	fn eval(&self, args: &[Term], ctx: &Context) -> Calculation;
}

/// Blanket impl for closures
impl<T> Func for T
where
	T: Fn(&[Term], &Context) -> Calculation,
{
	fn eval(&self, args: &[Term], ctx: &Context) -> Calculation {
		self(args, ctx)
	}
}
