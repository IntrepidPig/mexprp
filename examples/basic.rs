//! This example shows a simple command line calculator written with this library, with some basic
//! error handling.

extern crate mexprp;

use std::io::{self, Write};

use mexprp::Expression;

fn main() {
	println!("MEXPRP Test Calculator\n---------------------");
	loop {
		let mut buf = String::new();
		print!("> ");
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut buf).unwrap();
		
		// Parse the expression (with the default context)
		let expr = match Expression::parse(&buf) {
			Ok(expr) => expr,
			Err(e) => {
				println!("Failed to parse expression: {}", e);
				continue;
			}
		};
		
		// Evaluate the expression or print the error if there was one
		match expr.eval() {
			Ok(val) => println!("\t= {}", val),
			Err(e) => println!("Failed to evaluate the expression: {}", e)
		}
	}
}