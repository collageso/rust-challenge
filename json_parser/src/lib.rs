mod lex;
use lex::Lexer;

pub struct Json {}

impl Json {
    pub fn parse(input: &str) {
        let lexer = Lexer::new(input);
    }

    pub fn stringify() {}
}
