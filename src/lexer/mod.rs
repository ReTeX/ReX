mod lexer;
//mod nom;
pub use self::lexer::*;
use std::fmt;

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
            Token::EOF
            | Token::Symbol('}')
            | Token::Command("right")
            | Token::Command(r"\") => true,
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

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Command(cmd) => write!(f, r"\{}", cmd),
            Token::Symbol(c)    => write!(f, r"'{}'", c),
            Token::WhiteSpace   => write!(f, r"' '"),
            Token::EOF          => write!(f, "EOF"),
        }
    }
}