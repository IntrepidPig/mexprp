use {eval, Calculation, Context, Expression, Term};

#[test]
fn basic() {
	let raw = "(3 * (17.8 - 4) ^ 2) / 7";
	println!("\nParsing {}", raw);
	let expr = Expression::parse(raw).unwrap();
	println!("\nDone parsing {}", raw);
	assert!((expr.eval().unwrap() - 81.61714285714285).abs() < 0.001);
}

#[test]
fn var_context() {
	let expr = Expression::parse("3 - x ^ (0-3 + 0.22)").unwrap();
	let mut ctx = Context::new();
	ctx.set_var("x", 7.0);
	ctx.set_var("y", 0.22);
	assert!((expr.eval_ctx(&ctx).unwrap() - 2.995526705934608).abs() < 0.001);
}

#[test]
fn expr_context() {
	let expr = Expression::parse("3 * something").unwrap();
	let mut ctx = Context::new();
	ctx.set_var("something", Expression::parse("(0-8) ^ 2").unwrap());
	assert_eq!(expr.eval_ctx(&ctx).unwrap(), 192.0);
}

#[test]
fn funky() {
	let expr = Expression::parse("3(x * -(3 + 1))").unwrap();
	let mut ctx = Context::new();
	ctx.set_var("x", 2.0);
	assert_eq!(expr.eval_ctx(&ctx).unwrap(), -24.0);
}

#[test]
fn sin() {
	let expr = Expression::parse("2 + sin(3.1415926)").unwrap();
	assert!((expr.eval().unwrap() - 2.0) < 0.005);
}

#[test]
fn funcs() {
	assert!(eq(eval("max(sin(2), 5000000, -4)").unwrap(), 5000000.0));
	assert!(eq(eval("min(2 / -3 * 3 * 3, 5000000, -4)").unwrap(), -6.0));
	let mut context = Context::new();
	context.set_func("sum", |args: &[Term], ctx: &Context| -> Calculation {
		let mut x = 0.0;
		for arg in args {
			x += arg.eval_ctx(ctx)?;
		}
		Ok(x)
	});
	let expr = Expression::parse_ctx("sum(4, 5, 6) / 3", &context).unwrap();
	assert!(eq(expr.eval_ctx(&context).unwrap(), 5.0));
}

fn eq(x: f64, y: f64) -> bool {
	(x - y).abs() < 0.00001
}
