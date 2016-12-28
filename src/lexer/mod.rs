mod lexer;
//mod nom;
pub use self::lexer::*;

#[derive(Clone, Copy, Debug)]
pub struct Lexer<'a> {
    pub input: &'a str,

    /// The position of the _next_ token to be lexed.  So it
    /// is a true statement that `self.input[0..self.pos]` displays
    /// all characters that have and is currently being processed.
    pub pos: usize,

    /// The token currently being processed.
    pub current: Token<'a>,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Command(&'a str),
    Symbol(char),
    WhiteSpace,
    EOF,
}

impl<'a> Token<'a> {
    #[allow(dead_code)]
    pub fn ends_expression(&self) -> bool {
        match *self {
            Token::EOF         |
            Token::Symbol('}') |
            Token::Command("right") => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn expect(&self, expected: Token<'a>) -> Result<(), String> {
        if *self == expected {
            Ok(())
        } else {
            Err(format!("Expected token '{:?}', found the token '{:?}'",
                expected, self))
        }
    }
}