pub mod expression;
pub mod lexicon;

use crate::lexer::lexicon::{Atom, Operation, Token};
pub struct Lexer<'a> {
	input: &'a str,
	chars: std::str::Chars<'a>,
	peeked: Option<Token>,
}

/* struct L {
	tokens: Vec<Token>,
}
impl L {
	pub fn tokenize(input: &str) -> Self {
		let mut buf = String::new();
		let mut tokens: Vec<Token> = Vec::new();
		for c in input.chars().filter(|c| !c.is_ascii_whitespace()) {
			match c {
				'0'..='9' | '.' => {
					buf.push(c);
				}
				'a'..='z' | 'A'..='Z' => {
					if !buf.is_empty() {
						if let Ok(num) = buf.parse::<f64>() {
							tokens.push(Token::Atom(
								Atom::Number(num),
							));
						}
						buf.clear();
					}
					tokens.push(Token::Atom(Atom::Var(c)));
				}
				_ => {
					if !buf.is_empty() {
						if let Ok(num) = buf.parse::<f64>() {
							tokens.push(Token::Atom(
								Atom::Number(num),
							));
						}
						buf.clear();
					}
				}
			}
		}
		if !buf.is_empty()
			&& let Ok(num) = buf.parse::<f64>()
		{
			tokens.push(Token::Atom(Atom::Number(num)));
		}
		L { tokens }
	}
} */
impl<'a> Lexer<'a> {
	pub fn new(input: &'a str) -> Self {
		Self {
			input,
			chars: input.chars(),
			peeked: None,
		}
	}

	fn read_num(&mut self, first: char) -> Token {
		let mut buf = String::new();
		buf.push(first);
		while let Some(c) = self.chars.as_str().chars().next() {
			if c.is_ascii_digit() || c == '.' {
				buf.push(c);
				self.chars.next();
			} else {
				break;
			}
		}
		let value = buf.parse::<f64>().expect("Invalid number");
		Token::Atom(Atom::Number(value))
	}

	fn next_token(&mut self) -> Option<Token> {
		while let Some(c) = self.chars.next() {
			if c.is_whitespace() {
				continue;
			}
			if c.is_ascii_digit() {
				return Some(self.read_num(c));
			}
			if c.is_alphabetic() {
				return Some(Token::Atom(Atom::Var(c)));
			}
			if let Ok(op) = Operation::from_char(c) {
				return Some(Token::Op(op));
			}
		}
		None
	}

	pub fn peek(&mut self) -> Option<Token> {
		if self.peeked.is_none() {
			self.peeked = self.next_token();
		}
		self.peeked.clone()
	}
	/// Return the next token if available, None if no token is available
	pub fn next(&mut self) -> Option<Token> {
		if let Some(tok) = self.peeked.take() {
			return Some(tok);
		}
		self.next_token()
	}
}
