extern crate failure;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate nom;

mod op;
mod expr;

use op::Op;
use expr::Expr;

fn main() {
	let raw_expr = std::env::args().nth(1).unwrap();
	let expr = Expr::from(&raw_expr).unwrap();
	println!("{} = {}", expr, expr.eval().unwrap());
}