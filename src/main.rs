extern crate failure;
#[macro_use] extern crate failure_derive;

mod op;
mod expr;
mod opers;

use op::Op;
use expr::{Expr, Context};

fn main() {
	let raw_expr = std::env::args().nth(1).unwrap();
	let expr = Expr::from(&raw_expr).unwrap();
	let mut ctx = Context::new();
	ctx.add("x", 3.453523);
	ctx.add("e", Expr::from("3 + 2").unwrap());
	println!("{} = {}", expr, expr.eval_ctx(&ctx).unwrap());
}