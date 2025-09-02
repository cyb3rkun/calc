pub mod lexicon;
pub mod expression;

use crate::lexer::lexicon::{Atom, Operation, Token};
pub struct Tokenizer<'a> {
	input: &'a str,
	chars: std::str::Chars<'a>,
	peeked: Option<Token>,
}

impl<'a> Tokenizer<'a> {
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
	pub fn next(&mut self) -> Option<Token> {
		if let Some(tok) = self.peeked.take() {
			return Some(tok);
		}
		self.next_token()
	}
}
