# MEXPRP

[![Crates.io](https://img.shields.io/crates/v/mexprp.svg)](https://crates.io/crates/mexprp)
[![Docs.rs](https://docs.rs/mexprp/badge.svg)](https://docs.rs/mexprp)
![License](https://img.shields.io/crates/l/mexprp.svg)


A math expression parsing and evaluation library for Rust

[API docs here](https://docs.rs/mexprp). Also see the `examples/` directory.

### Motivation 

The main reason I wrote MEXPRP was for a 3D equation grapher I've been working on ([vgraph](https://github.com/intrepidpig/vgraph)). I can't really say why I didn't choose any existing libraries other than because I wanted a learning experience, and because I wanted flexibility. I'm glad to say I learned a lot from this project, and it's also quite flexible.

## Features
- `f64` precision
- multiple/arbitrary precision (somewhat incomplete)
- low dependencies
- custom variable contexts
- custom function contexts
- builtin constants and functions (eg pi, sin, max)
- implicit multiplication
- utf8-ready
- support for multiple answers
- complex numbers (somewhat incomplete)

## Usage
There are several different ways to parse and evaluate an equation.

 #### With `eval()`
This function parses and evaluates a string all at once with the default context. There's also an `eval_ctx()` function which takes a reference to a `Context` as well that will be used instead of the default `Context`. The type parameter can be anything that implements the `Num` trait. Some `Num` types support more operations than others. More info about `Num`s can be found in the `Num` module.

```rust
mexprp::eval::<f64>("10 / (2 + 3)"); // Ok(Answer::Single(2.0))
```

#### With `Expression`
`Expression::parse()` parses a string into a tree representation (a `Term`). It can also be parsed with a context with `parse_ctx()`, and it will store that context within it for future evaluations. It can also be evaluated with a reference to any other context with `eval_ctx`. It's important to ensure that the custom context contains any definitions the `Expression` depends on.

```rust
let expr: Expression<f64> = Expression::parse("3 ^ 4 / 9").unwrap();
let res = expr.eval(); // Ok(Answer::Single(9.0))
```

#### Using Contexts
You can evaluate expressions with custom variable and function definition's by defining a context. When defining custom functions, it's important to remember to parse the expression with the custom context, or else the parser will recognize your functions as variables instead. One way to bypass this is by disabling implicit multiplication in the context used for parsing, which will then parse all names followed by parentheses as functions, regardless of whether they are defined in the `Context`.

A `Context` also holds configuration values that define how MEXPRP parses and evaluates equations. These configuration values include enabling/disabling implicit multiplication, the precision to use for types that support selecting precisions (just `Complex` for now), and the behaviour of the `sqrt()` function. More info can be found in the API docs (check the `context` module).

```rust
let mut context: Context<f64> = Context::new();
context.set_var("x", 4.0);
let expr = Expression::parse_ctx("4x", context).unwrap();
let res = expr.eval(); // Ok(Answer::Single(16.0))
```

For a list of builtin functions/constants in `Context`s, see the API docs for the `Context` struct. 

### Multiple Precisions
MEXPRP supports evaluating expressions with different precisions and complex numbers with the [`Num`](num::Num) trait. Currently supported number types are
- `f64`
- [`ComplexFloat`](num::ComplexFloat)
- [`ComplexRugRat`](num::ComplexRugRat) (using the rug crate)
- [`Rational`](::rug::Rational) (from the rug crate)
- [`Complex`](::rug::Complex) (from the rug crate)

However, the implementation for certain types is incomplete. Only the `f64` type fully implements all of the operations. `Complex` is the next best, but even it is still missing some. The others only implement a (small) subset of the functionality of the `Num` trait, and return a `MathError::Unimplemented` when an unsupported operation is attempted. It is hopeful that more functions will be implemented in the future, but some are very difficult to implement for arbitrary precision or rational numbers.

For more info on the types, see the documentation for the `num` module. To see progress on implementations for numbers, see GitHub [issues](https://github.com/IntrepidPig/mexprp/issues?q=is%3Aopen+is%3Aissue+label%3Anumber) with the `number` label.

To use another number type, change the type annotation(s) for your MEXPRP types.
```rust
extern crate rug;
use rug::Rational;
mexprp::eval::<Rational>("10/15"); // 2/3
```

```rust
extern crate rug;
use rug::Complex;
mexprp::eval::<Complex>("(2+3i)(2-3i)"); // 23 + 2i
```

### Multiple Answers
Any evaluation of an expression in MEXPRP returns an `Answer`. An answer is a simple enum that is either `Single(N)` or `Multiple(Vec<N>)` where N is the type of number this expression is using. This represents answers to operations that possibly yield multiple values such as `sqrt()` or the `±` operator. If you know the result of an expression will be just one answer, you can use the `unwrap_single()` method of answer to get that one answer.

Be sure to check the [API docs](https://docs.rs/mexprp) for more in depth explanations of usage.

### License

[MPL-2.0](https://choosealicense.com/licenses/mpl-2.0/)
