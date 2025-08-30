#![allow(dead_code, unused_variables)]

use std::{collections::HashMap, io::Write};

use crate::lexer::expression::Expr;

mod lexer;

fn main() {
	let mut variables: HashMap<char, f64> = HashMap::new();
	loop {
		print!(">> ");
		std::io::stdout().flush().unwrap();
		let input = {
			let mut buf = String::new();
			std::io::stdin().read_line(&mut buf).unwrap();
			buf
		};
		match input.trim() {
			"stop" => break,
			"exit" => break,
			_ => {}
		}

		let expr = Expr::parse(&input);
		if let Some((var_name, lhs)) = expr.is_assign() {
			let value = lhs.eval(&variables);
			variables.insert(var_name, value);
			continue;
		};
		let value = expr.eval(&variables);

		println!("{}", value);
	}
}
