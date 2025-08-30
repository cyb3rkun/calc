pub mod lexicon;
use crate::lexer::lexicon::*;
pub mod expression;

#[derive(Debug, Clone)]
pub struct Lex {
	tokens: Vec<Token>,
}
impl Lex {
	fn next(&mut self) -> Option<Token> {
		self.tokens.pop()
	}
	fn peek(&mut self) -> Option<Token> {
		self.tokens.last().cloned()
	}
}

pub trait Lexer {
	fn parse_string(input: &str) -> Result<Lex, String>;
}
impl Lexer for Lex {
	// tokenize input string and return as Lexer (Vec<Token>)
	fn parse_string(input: &str) -> Result<Lex, String> {
		let mut digit_buff = String::new();
		let mut tokens = Vec::new();

		for c in input.chars().filter(|c| !c.is_whitespace()) {
			if c.is_ascii_digit() {
				digit_buff.push(c);
				// skip rest of loop and start new iteration
				continue;
			}
			if !digit_buff.is_empty() {
				let value = Atom::num_from_str(&digit_buff)?;
				tokens.push(Token::Atom(value));
				digit_buff.clear();
			}
			if c.is_alphabetic() {
				let var = Atom::var_from_char(c);
				tokens.push(Token::Atom(var));
				// skip rest of loop and start new iteration
				continue;
			}
			let op = Operation::from_char(c)?;
			tokens.push(Token::Op(op));
		}
		if !digit_buff.is_empty() {
			let value: f64 = digit_buff.parse().unwrap();
			tokens.push(Token::Atom(Atom::Number(value)));
			digit_buff.clear();
		}

		tokens.reverse();
		Ok(Lex { tokens })
	}
}
