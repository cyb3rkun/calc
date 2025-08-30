use std::collections::HashMap;

use crate::lexer::*;
#[derive(Debug, Clone)]
pub enum Expr {
	Atom(Atom),
	Operation {
		op: Operation,
		left: Box<Expr>,
		right: Box<Expr>,
	},
}

impl Expr {
	pub fn parse(input: &str) -> Expr {
		match Lex::parse_string(input) {
			Ok(mut lexer) => Self::from_lex(&mut lexer, 0.0),
			Err(e) => panic!("Error: Lexer Error: {:?}", e),
		}
	}
	pub fn from_lex(lexer: &mut Lex, min_bp: f32) -> Expr {
		let mut lhs = match lexer.next() {
			Some(Token::Atom(it)) => Expr::Atom(it),
			Some(Token::Op(Operation::Parentheses(Paren::Open))) => {
				let expr = Self::from_lex(lexer, 0.0);
				match lexer.next() {
					// consume closing parenthesis so caller
					// can continue
					Some(Token::Op(Operation::Parentheses(
						Paren::Close,
					))) => expr,
					_ => panic!("Expected closing parenthesis"),
				}
			}
			Some(Token::Op(_)) => {
				panic!("Expression cannot start with operator")
			}
			None => panic!("Unexpected end of input"),
		};
		loop {
			let op = match lexer.peek() {
				Some(Token::Op(Operation::Parentheses(
					Paren::Close,
				))) => {
					break;
				}
				Some(Token::Op(operation))
					if !matches!(
						operation,
						Operation::Parentheses(_)
					) =>
				{
					operation
				}
				Some(Token::Op(_)) => break,
				Some(Token::Atom(a)) => {
					panic!("Unexpected atom {:?}", a)
				}
				None => break,
			};
			let (l_bp, r_bp) = op.get_binding_power();
			if l_bp < min_bp {
				break;
			}

			lexer.next();
			let rhs = Self::from_lex(lexer, r_bp);

			lhs = Expr::Operation {
				op,
				left: Box::new(lhs),
				right: Box::new(rhs),
			}
		}
		lhs
	}
	pub fn is_assign(&self) -> Option<(char, &Expr)> {
		match self {
			Expr::Operation {
				op: Operation::Assign,
				left,
				right,
			} => {
				if let Expr::Atom(Atom::Var(c)) = &**left {
					Some((*c, &**right))
				} else {
					None
				}
			}
			_ => None,
		}
	}
	pub fn eval(&self, variables: &HashMap<char, f64>) -> f64 {
		match self {
			Expr::Atom(c) => match c {
				Atom::Number(value) => *value,

				Atom::Var(var_name) => *variables
					.get(var_name)
					.unwrap_or_else(|| panic!("Undefined variable {:?}", var_name)),
			},
			Expr::Operation { op, left, right } => {
				let lhs = left.eval(variables);
				let rhs = right.eval(variables);
				match op {
					Operation::Add => lhs + rhs,
					Operation::Subtract => lhs - rhs,
					Operation::Multiply => lhs * rhs,
					Operation::Divide => lhs / rhs,
					Operation::Pow => lhs.powf(rhs),
					op => panic!("Bad Operator: {:?}", op),
				}
			}
		}
	}
}
