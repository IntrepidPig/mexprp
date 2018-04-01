use expr::{Term, Calculation};
use context::Context;

pub trait Func {
	fn eval(&self, args: &[Term], ctx: &Context) -> Calculation;
}

impl<T> Func for T where T: Fn(&[Term], &Context) -> Calculation {
	fn eval(&self, args: &[Term], ctx: &Context) -> Calculation {
		self(args, ctx)
	}
}
