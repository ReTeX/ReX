#[derive(Clone, Copy, Debug, Default)]
pub struct Lexer<'a> {
    pub input: &'a str,
    pub pos: usize,         // cursor position of lexer
    pub current: Option<Token<'a>>,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    ControlSequence(&'a str),
    Symbol(char),
    WhiteSpace,
    EOF,
}

impl<'a> Token<'a> {
    #[allow(dead_code)]
    pub fn ends_expression(&self) -> bool {
        match *self {
            Token::EOF |
            Token::Symbol('}') |
            Token::ControlSequence("right") => true,
            _ => false,
        }
    }
}