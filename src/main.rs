#![allow(dead_code, unused_variables)]

use std::process::Command;
use std::{collections::HashMap, io::Write};

use crate::lexer::expression::Expr;

mod lexer;

fn main() {
	let mut variables: HashMap<char, f64> = HashMap::new();
	loop {
		let inpt = input(">> ");
		let trimmed = inpt.trim();
		match trimmed.trim() {
			"stop" => break,
			"exit" => break,
			"clear" => {
				if cfg!(target_os = "windows") {
					Command::new("cmd")
						.args(["/C", "cls"])
						.status()
						.unwrap();
				} else {
					Command::new("clear").status().unwrap();
				}
				continue;
			}
			"help" => {
				println!("Help menu coming soon");
				continue;
			}
			_ => {}
		}

		let expr = Expr::parse_stream(trimmed);
		// store variables if operation is an assignment
		if let Some((var_name, lhs)) = expr.is_assign() {
			let value = lhs.eval(&variables);
			variables.insert(var_name, value);
			continue;
		};

		println!("{}", expr);
		println!("{:#?}", expr);
		let value = expr.eval(&variables);

		println!("Result: {}", value);
	}
}

fn input(prompt: &str) -> String {
	print!("{}", prompt);
	std::io::stdout().flush().unwrap();
	let mut buf = String::new();
	std::io::stdin().read_line(&mut buf).unwrap();
	buf
}


