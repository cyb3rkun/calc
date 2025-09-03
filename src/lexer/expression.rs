use std::collections::HashMap;

use crate::lexer::lexicon::{Atom, Operation, Paren, Token, Unary};
// use crate::lexer::{Lex, Lexer};

use crate::lexer::Tokenizer;
// use crate::lexer::{Lex as OldLex, Lexer as OldLexer};
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
	pub fn parse_stream(input: &str) -> Expr {
		let mut lexer = Tokenizer::new(input);
		Self::from_stream(&mut lexer, 0.0)
	}
	fn from_stream<'a>(lexer: &mut Tokenizer<'a>, min_bp: f32) -> Expr {
		let mut lhs = match lexer.next() {
			Some(Token::Atom(it)) => Expr::Atom(it),
			Some(Token::Op(Operation::Parentheses(Paren::Open))) => {
				let expr = Self::from_stream(lexer, 0.0);
				match lexer.next() {
					Some(Token::Op(Operation::Parentheses(
						Paren::Close,
					))) => expr,
					_ => panic!("Unclosed Parentheses"),
				}
			}
			// Unary Add
			Some(Token::Op(Operation::Add)) => {
				let rhs = Self::from_stream(lexer, 100.0);
				// Expr::Atom(Atom::Number(rhs))
				Expr::Operation {
					op: Operation::Unary(Unary::Positive),
					left: Box::new(Expr::Atom(Atom::Number(0.0))),
					right: Box::new(rhs),
				}
			}
			//Unary Minus
			Some(Token::Op(Operation::Subtract)) => {
				let rhs = Self::from_stream(lexer, 100.0);
				Expr::Operation {
					op: Operation::Unary(Unary::Negate),
					left: Box::new(Expr::Atom(Atom::Number(0.0))),
					right: Box::new(rhs),
				}
			}
			Some(Token::Op(op)) => {
				panic!(
					"Expression can't start with operator! {:?}",
					op
				)
			}
			None => panic!("Unexpected end of input"),
		};
		loop {
			let implicit_mul = matches!(
				lhs,
				Expr::Atom(_)
					| Expr::Operation {
						op: Operation::Parentheses(Paren::Close),
						..
					} | Expr::Operation {
					op: Operation::Unary(_),
					..
				}
			);
			let op = match lexer.peek() {
				Some(Token::Op(Operation::Parentheses(
					Paren::Close,
				))) => break,

				Some(Token::Op(Operation::Parentheses(
					Paren::Open,
				))) if implicit_mul => Operation::Multiply,

				Some(Token::Op(operation))
					if !matches!(
						operation,
						Operation::Parentheses(_)
					) =>
				{
					operation
				}

				Some(Token::Atom(_)) if implicit_mul => {
					Operation::Multiply
				}

				Some(Token::Atom(a)) => {
					panic!("unexpected atom {:?}", a)
				}

				Some(Token::Op(Operation::Root)) => break,
				Some(Token::Op(_)) => break,

				None => break,
			};
			let (l_bp, r_bp) = op.get_binding_power();
			if l_bp < min_bp {
				break;
			}
			lexer.next(); // consume operator
			let rhs = Self::from_stream(lexer, r_bp);

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

				Atom::Var(var_name) => {
					*variables.get(var_name).unwrap_or_else(|| {
						panic!("Undefined variable {:?}", var_name)
					})
				}
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
					Operation::Root => rhs.powf(1.0 / lhs),
					Operation::Unary(Unary::Negate) => -rhs,
					Operation::Unary(Unary::Positive) => rhs,
					op => panic!("Bad Operator: {:?}", op),
				}
			}
		}
	}
}
