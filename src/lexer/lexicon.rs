use std::fmt::{self};

#[derive(Debug, Clone)]
pub enum Paren {
	Open,
	Close,
}
impl fmt::Display for Paren {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Paren::Open => write!(f, "("),
			Paren::Close => write!(f, ")"),
		}
	}
}

#[derive(Debug, Clone)]
pub enum Unary {
	Negate,
	Positive,
}
impl fmt::Display for Unary {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Unary::Negate => write!(f, "-"),
			Unary::Positive => write!(f, "+"),
		}
	}
}
#[derive(Debug, Clone)]
pub enum Operation {
	Add,
	Subtract,
	Multiply,
	Divide,
	Parentheses(Paren),
	Pow,
	Root,
	Assign,
	// Unary(Unary),
}
impl Operation {
	pub fn infix_bp(&self) -> (f32, f32) {
		match self {
			Operation::Assign => (0.2, 0.1),
			Operation::Add | Operation::Subtract => (1.0, 1.1),
			Operation::Multiply | Operation::Divide => (2.0, 2.1),
			// Operation::Unary(_) => (4.1, 4.0),
			Operation::Pow => (3.0, 3.1),
			Operation::Root => (3.0, 3.1),
			Operation::Parentheses(Paren::Open) => (0.0, 0.0),
			Operation::Parentheses(Paren::Close) => (0.0, 0.0),
		}
	}
	pub fn prefix_bp(&self) -> ((), f32) {
		match self {
			Operation::Subtract | Operation::Add => ((), 5.0),
			op => panic!("bad op: {:?}", op),
		}
	}
	pub fn from_char(input: char) -> Result<Operation, String> {
		match input {
			'=' => Ok(Operation::Assign),
			'+' => Ok(Operation::Add),
			'-' => Ok(Operation::Subtract),
			'*' => Ok(Operation::Multiply),
			'/' => Ok(Operation::Divide),
			'^' => Ok(Operation::Pow),
			'(' => Ok(Operation::Parentheses(Paren::Open)),
			')' => Ok(Operation::Parentheses(Paren::Close)),
			'√' => Ok(Operation::Root),
			other => Err(format!("Unknown operator: {}", other)),
		}
	}
	pub fn to_unary(&self) -> Unary {
		match self {
			Operation::Subtract => Unary::Negate,
			Operation::Add => Unary::Positive,
			t => panic!("Bad token for unary conversion: {:?}", t),
		}
	}
}
impl fmt::Display for Operation {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Operation::Add => write!(f, "+"),
			Operation::Subtract => write!(f, "-"),
			Operation::Multiply => write!(f, "*"),
			Operation::Divide => write!(f, "/"),
			Operation::Parentheses(p) => write!(f, "{}", p),
			Operation::Pow => write!(f, "^"),
			Operation::Root => write!(f, "√"),
			Operation::Assign => write!(f, "="),
		}
	}
}

#[derive(Debug, Clone)]
pub enum Atom {
	Number(f64),
	Var(char),
}
impl Atom {
	pub fn num_from_str(input: &str) -> Result<Atom, String> {
		match input.parse::<f64>() {
			Ok(value) => Ok(Atom::Number(value)),
			Err(e) => Err(format!("Invalid number '{}': {}", input, e)),
		}
	}
	pub fn var_from_char(c: char) -> Atom {
		Atom::Var(c)
	}
}
impl fmt::Display for Atom {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Atom::Number(num) => write!(f, "{}", num),
			Atom::Var(var) => write!(f, "{}", var),
		}
	}
}

#[derive(Debug, Clone)]
pub enum Token {
	Op(Operation),
	Atom(Atom),
}
