#![allow(dead_code, unused_variables)]

use std::{collections::HashMap, io::Write};

use crate::lexer::{expression::Expr, Lex, Lexer};

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

		let mut lex = match Lex::parse_string(&input) {
			Ok(l) => l,
			Err(e) => panic!("Error: {}", e),
		};
		println!("Tokens:\n{:?}", lex);
		let expr = Expr::from_lex(&mut lex, 0.0);
		println!("Expression Tree:\n{:#?}", expr);
		if let Some((var_name, lhs)) = expr.is_assign() {
			let value = lhs.eval(&variables);
			variables.insert(var_name, value);
			continue;
		};
		let value = expr.eval(&variables);

		println!("Result: {}", value);
	}

}
