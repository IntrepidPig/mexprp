use {Expr, Context};

#[test]
fn basic() {
	let raw = "(3 * (17.8 - 4) ^ 2) / 7";
	println!("\nParsing {}", raw);
	let expr = Expr::from(raw).unwrap();
	println!("\nDone parsing {}", raw);
	assert!((expr.eval().unwrap() - 81.61714285714285).abs() < 0.001);
}

#[test]
fn var_context() {
	let expr = Expr::from("3 - x ^ (0-3 + 0.22)").unwrap();
	let mut ctx = Context::new();
	ctx.add("x", 7.0);
	ctx.add("y", 0.22);
	assert!((expr.eval_ctx(&ctx).unwrap() - 2.995526705934608).abs() < 0.001);
}

#[test]
fn expr_context() {
	let expr = Expr::from("3 * something").unwrap();
	let mut ctx = Context::new();
	ctx.add("something", Expr::from("(0-8) ^ 2").unwrap());
	assert_eq!(expr.eval_ctx(&ctx).unwrap(), 192.0);
}