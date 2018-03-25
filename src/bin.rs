extern crate mexprp;

use mexprp::Expr;

fn main() {
	let raw = " (3 *(   (15/ (4-1)))  ";
	let expr = Expr::from(raw).unwrap();
	assert!((expr.eval().unwrap() - 15f64).abs() < 0.001f64);
}