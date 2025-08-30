#[derive(Debug, Clone)]
pub enum Paren {
	Open,
	Close,
}

#[derive(Debug, Clone)]
pub enum Operation {
	Add,
	Subtract,
	Multiply,
	Divide,
	Parentheses(Paren),
	Pow,
	Assign,
}
impl Operation {
	pub fn get_binding_power(&self) -> (f32, f32) {
		match self {
			Operation::Assign => (0.2, 0.1),
			Operation::Add | Operation::Subtract => (1.0, 1.1),
			Operation::Multiply | Operation::Divide => (2.0, 2.1),
			Operation::Pow => (3.0,3.1),
			Operation::Parentheses(Paren::Close) => (0.0,0.0),
			Operation::Parentheses(Paren::Open) => (0.0,0.0),
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
			other => Err(format!("Unknown operator: {}", other)),
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

#[derive(Debug, Clone)]
pub enum Token {
	Op(Operation),
	Atom(Atom),
}
