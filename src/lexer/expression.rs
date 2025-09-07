use std::collections::HashMap;
use std::fmt::{self};

use crate::lexer::lexicon::{Atom, Operation, Paren, Token, Unary};
// use crate::lexer::{Lex, Lexer};

use crate::lexer::Lexer;
// use crate::lexer::{Lex as OldLex, Lexer as OldLexer};
#[derive(Debug, Clone)]
pub enum Expr {
	Atom(Atom),
	Operation {
		op: Operation,
		left: Box<Expr>,
		right: Box<Expr>,
	},
	Unary {
		op: Unary,
		expr: Box<Expr>,
	},
}
impl fmt::Display for Expr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Expr::Atom(i) => write!(f, "{}", i),
			Expr::Operation { op, left, right } => {
				write!(f, "({} {} {})", op, left, right)
			}
			Expr::Unary { op, expr } => {
				write!(f, "({}{})", op, expr)
			}
		}
	}
}

impl Expr {
	pub fn parse_stream(input: &str) -> Expr {
		let mut lexer = Lexer::new(input);
		Self::parse(&mut lexer, 0.0)
	}
	fn parse<'a>(lexer: &mut Lexer<'a>, min_bp: f32) -> Expr {
		let mut lhs = match lexer.next() {
			Some(Token::Atom(it)) => Expr::Atom(it),
			Some(Token::Op(Operation::Parentheses(Paren::Open))) => {
				// Subexpression in parentheses!
				let expr = Self::parse(lexer, 0.0);
				// once parentheses close return the expression
				match lexer.next() {
					Some(Token::Op(Operation::Parentheses(
						Paren::Close,
					))) => expr,
					_ => panic!("Unclosed Parentheses"),
				}
			}
			Some(Token::Op(op)) => {
				let ((), r_bp) = op.prefix_bp();
				let rhs = Self::parse(lexer, r_bp);
				Expr::Unary {
					op: op.to_unary(),
					expr: Box::new(rhs),
				}
			}
			t => panic!("Bad Token: {:?}", t),
		};
		loop {
			let op = match lexer.peek() {
				Some(Token::Op(Operation::Parentheses(
					Paren::Close,
				))) => break,

				Some(Token::Op(op)) => op,

				Some(Token::Atom(a)) => {
					panic!("unexpected atom {:?}", a)
				}
				None => break,
			};
			let (l_bp, r_bp) = op.infix_bp();
			if l_bp < min_bp {
				break;
			}

			lexer.next();

			let rhs = Self::parse(lexer, r_bp);

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
					// handle implicit multiplication
					Operation::Parentheses(Paren::Open) => lhs * rhs,
					op => panic!("Bad Operator: {:?}", op),
				}
			}
			Expr::Unary { op, expr } => match op {
				Unary::Negate => -expr.eval(variables),
				Unary::Positive => expr.eval(variables),
			},
		}
	}
}


#[test]
fn test_expr2() {
	let vars: HashMap<char, f64> = HashMap::new();
	let s = Expr::parse_stream("1").eval(&vars);
	assert_eq!(s, 1.0);

	let s = Expr::parse_stream("1");
	assert_eq!(s.to_string(), "1");
}
#[test]
fn test_expr3() {
	let vars: HashMap<char, f64> = HashMap::new();

	let s = Expr::parse_stream("1 + 2 * 3").eval(&vars);
	assert_eq!(s, 7.0);
	let s = Expr::parse_stream("1 + 2 * 3");
	assert_eq!(s.to_string(), "(+ 1 (* 2 3))");
}
#[test]
fn test_expr4() {
	let vars: HashMap<char, f64> = HashMap::new();
	let s = Expr::parse_stream("1").eval(&vars);
	assert_eq!(s, 1.0);
	let s = Expr::parse_stream("a + b * c * d + e");
	assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");
}
#[test]
fn test_expr5() {
	let vars: HashMap<char, f64> = HashMap::new();

	let s = Expr::parse_stream("1").eval(&vars);
	assert_eq!(s, 1.0);
	let s = Expr::parse_stream("1");
	assert_eq!(s.to_string(), "1");
}
#[test]
fn test_expr6() {
	let vars: HashMap<char, f64> = HashMap::new();

	let s = Expr::parse_stream("--1 * 2").eval(&vars);
	assert_eq!(s, 2.0);
	let s = Expr::parse_stream("--1 * 2");
	assert_eq!(s.to_string(), "(* (-(-1)) 2)");
}
#[test]
fn test_expr7() {
	let mut vars: HashMap<char, f64> = HashMap::new();
	vars.insert('f', 1.0);
	vars.insert('g', 5.0);

	let expr = Expr::parse_stream("--f * g");
	assert_eq!(expr.to_string(), "(* (-(-f)) g)");

	let s = expr.eval(&vars);
	assert_eq!(s, 5.0);
}
#[test]
fn test_expr8() {}
#[test]
fn test_expr9() {}
#[test]
fn test_expr10() {}
#[test]
fn test_expr11() {}
// fn test_expr1() {}
// fn test_expr1() {}
// fn test_expr1() {}
// fn test_expr1() {}
// fn test_expr1() {}
