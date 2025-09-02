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
// TODO: Implement stream parser as it would probably be easier to add features!
impl Lexer for Lex {
	// tokenize input string and return as Lexer (Vec<Token>)
	fn parse_string(input: &str) -> Result<Lex, String> {
		let mut digit_buff = String::new();
		let mut prev_token: Option<Token> = None;
		let mut tokens = Vec::new();

		for c in input.chars().filter(|c| !c.is_whitespace()) {
			if c.is_ascii_digit() {
				digit_buff.push(c);
				// skip rest of loop and start new iteration
				continue;
			}
			if !digit_buff.is_empty() {
				let value = Atom::num_from_str(&digit_buff)?;
				let atom = Token::Atom(value);
				tokens.push(atom.clone());
				prev_token = Some(atom);
				digit_buff.clear();
			}
			if c.is_alphabetic() {
				let var = Atom::var_from_char(c);
				let atom = Token::Atom(var);

				if matches!(
					prev_token,
					Some(Token::Atom(_))
						| Some(Token::Op(Operation::Parentheses(
							Paren::Close
						)))
				) {
					tokens.push(Token::Op(Operation::Multiply));
				}
				tokens.push(atom.clone());
				prev_token = Some(atom);
				continue;
			}
			if c == '('
				&& matches!(
					prev_token,
					Some(Token::Atom(_))
						| Some(Token::Op(Operation::Parentheses(
							Paren::Close
						)))
				) && !matches!(
				prev_token,
				Some(Token::Op(Operation::Pow))
					| Some(Token::Op(Operation::Root))
			) {
				tokens.push(Token::Op(Operation::Multiply));
			}
			let op = Operation::from_char(c)?;
			let token = Token::Op(op);
			tokens.push(token.clone());
			prev_token = Some(token)
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
