# MEXPRP

[![Crates.io](https://img.shields.io/crates/v/mexprp.svg)](https://crates.io/crates/mexprp)
[![Docs.rs](https://docs.rs/mexprp/badge.svg)](https://docs.rs/mexprp)
![License](https://img.shields.io/crates/l/mexprp.svg)


A math expression parsing and evaluation library for Rust with the goal of being simple to use, yet powerful.

[API docs here](https://docs.rs/mexprp). Also see the `examples/` directory.

### Motivation 

The main reason I wrote MEXPRP was for a 3D equation grapher I've been working on ([vgraph](https://github.com/intrepidpig/vgraph)). Of course, I could've used an existing Rust expression parser, but the only other one was a `0.1.0` library that had what I considered I somewhat strange API. Also, for some reason I've been kind of interested in keeping the dependencies down for the 3D equation grapher I was talking about. Anyway, now there's [at least two](https://xkcd.com/927/) `0.x` Rust math expression libraries with API's that can be considered strange.

### Features

- `f64` precision
- Custom variable contexts
- Custom function contexts
- Builtin constants and functions
- Implicit multiplication
- Low dependencies (just failure)
- Easy to use
- UTF-8 ready

### Usage

The simplest, but not always the most efficient, way to use MEXPRP is with `mexprp::eval()`. This function takes a `&str` as an argument, parses it, and evaluates it. E.g. like this:

```rust
mexprp::eval("(2 + 7) / 3")? // 3.0
```

MEXPRP also supports some functions and constants.

```rust
mexprp::eval("sin(max(2, 3, pi))")? // 0.0
```

A "better" way to evaluate an expression is to compile it first, with the `Expression` struct. This has the advantage of retaining a completely parsed and organized instance of an expression for faster evaluation.

```rust
let expr = Expression::parse("3 ^ (4 - 1)")?;
expr.eval()? // 27.0
```

But what's the point of being able to evaluate the same expression over and over again? Well, you can evaluate expressions with variables set to a specific value by evaluating it with a context.

```rust
let expr = Expression::parse("3x / 6")?;
let mut ctx = Context::new();
ctx.set_var("x", 8.0);
expr.eval_ctx(&ctx)?; // 4.0
ctx.set_var("x", 10.0);
expr.eval_ctx(&ctx)?; // 5.0
```

It's also possible to define custom functions. When defining custom functions, you should be aware of a minor drawback. Expressions need to be parsed with the custom context containing the custom function definitions in order for them to recognized as functions instead of variables during parsing. The reason for this is that without a list of functions present at parse time, the parser has no way to know if a name in the expression is a variable or a function. For example, in `foo(3 + 5)`, `foo` could be a function called with one argument, or a variable multiplied by `(3 + 5)`. The parser will assume the latter.

In order to bypass this, simply create a context before parsing the expression, then use `Expression::parse_ctx()` to parse the expression. The expression will store the context and calling eval on the expression will use it. You can modify the context by accessing the `ctx` field of the epxression. You can also evaluate the expression with other contexts with the `eval_ctx()` function. If you don't wan't to store the context, use a `Term` instead.

There are two ways to define a function. A function is anything that implements the `func::Func` trait. There is a blanket `impl` of this trait for all `Fn(&[Term], &Context) -> Calculation`, allowing you to pass in a closure. You can also pass in an empty struct that you implement `Func` for manually, which is no harder then writing it as a closure, but can be more flexible. The `Func` trait consists of one method, with the signature `fn(args: &[Term], ctx: &Context) -> Calculation`. (A `Term` is just an `Expression` without the metadata.) In case the arguments given were not properly formatted (e.g. there was an incorrect amount given), you can just return `Err(MathError::IncorrectArguments)`.

```rust
let mut ctx = Context::new();
ctx.set_func("funca", |args: &[Term], ctx: &Context| -> Calculation {
	if args.len() != 2 { return Err(MathError::IncorrectArguments) }
	
	let a = args[0].eval_ctx(ctx)?;
	let b = args[1].eval_ctx(ctx)?;
	Ok(a + b)
});

struct FuncB;
impl Func for FuncB {
	fn eval(&self, args: &[Term], ctx: &Context) -> Calculation {
		if args.is_empty() { return Err(MathError::IncorrectArguments) }
		
		let mut sum = 0.0;
		for arg in args {
			sum += arg.eval_ctx(ctx)?;
		}
		Ok(sum)
	}
}
ctx.set_func("funcb", FuncB);

let mut expr = Expression::parse_ctx("funca(5, funcb(3, 4, 3))", ctx)?;
expr.eval()?; // 15.0

expr.ctx.set_func("funca", |args: &[Term], ctx: &Context| -> Calculation {
	if args.len() != 2 { return Err(MathError::IncorrectArguments) }
	
	let a = args[0].eval_ctx(ctx)?;
	let b = args[1].eval_ctx(ctx)?;
	Ok(a - b)
});

expr.eval()?; // -5.0

Ok(())
```

### License

[MPL-2.0](https://choosealicense.com/licenses/mpl-2.0/)
