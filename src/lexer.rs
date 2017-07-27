use std::fmt;
use dimensions::Unit;
use parser::color::RGBA;
use error::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Command(&'a str),
    Symbol(char),
    WhiteSpace,
    EOF,
}

/// An owned variant of `Token<'a>`. This is only used for
/// having lifetime-free errors for now.  Once NLL is implemented
/// it's likely, we can use errors with lifetimes with minimal
/// change to the codebase.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OwnedToken {
    Command(String),
    Symbol(char),
    WhiteSpace,
    EOF,
}

impl<'a> Token<'a> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn ends_expression(self) -> bool {
        match self {
            Token::EOF
            | Token::Symbol('}')
            | Token::Command("right")
            | Token::Command(r"\") => true,
            _ => false,
        }
    }

    pub fn expect(&self, expected: Token<'a>) -> Result<(), Error> {
        if *self == expected {
            Ok(())
        } else {
            Err(Error::ExpectedTokenFound(expected.into(), (*self).into()))
        }
    }

    pub fn expect_command(self, expected: &'static str) -> Result<(), Error> {
        self.expect(Token::Command(expected))
    }

    pub fn expect_symbol(self, expected: char) -> Result<(), Error> {
        self.expect(Token::Symbol(expected))
    }

    pub fn expect_whitespace(self) -> Result<(), Error> {
        self.expect(Token::WhiteSpace)
    }

    pub fn expect_eof(self) -> Result<(), Error> {
        self.expect(Token::EOF)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Lexer<'a> {
    pub input: &'a str,

    /// The position of where _next_ token to be lexed begins.
    pub pos: usize,

    /// The position of where `self.current` begins.
    pub prev_pos: usize,

    /// The token currently being processed.
    pub current: Token<'a>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer, whose current token is the first token
    /// to be processed.
    pub fn new(input: &'a str) -> Lexer<'a> {
        let mut lex = Lexer {
            input: input,
            pos: 0,
            prev_pos: 0,
            current: Token::EOF,
        };

        lex.next();
        lex
    }

    /// Advanced to the next token to be processed, and return it.
    /// This will also modify `Lexer.current`.
    pub fn next(&mut self) -> Token<'a> {
        self.prev_pos = self.pos;
        self.current = match self.next_char() {
            Some(c) if c.is_whitespace() => {
                self.advance_while_whitespace();
                Token::WhiteSpace
            }
            Some('\\') => self.control_sequence(),
            Some(c) => Token::Symbol(c),
            None => Token::EOF,
        };

        debug!("{:?}", self.current);
        self.current
    }

    /// If the current token being processed from the lexer
    /// is a `WhiteSpace` then continue to call `.next()`
    /// until `lex.current` is the first non-whitespace token.
    /// This method is indepotent, so that calling this method
    /// twice has no effect.
    pub fn consume_whitespace(&mut self) {
        if self.current != Token::WhiteSpace {
            return;
        }
        self.advance_while_whitespace();
        self.next();
    }

    /// This method is the same as [consume_whitespace],
    /// except that it does not process the next token.
    fn advance_while_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if !c.is_whitespace() {
                break;
            }
            self.pos += c.len_utf8();
        }
    }

    /// Lex a control sequence.  This method assumes that
    /// `self.pos` points to the first character after `\`.
    /// The cursor will advance through the control sequence
    /// name, and consume all whitespace proceeding.  When
    /// complete `self.pos` will point to the first character
    /// of the next item to be lexed.
    fn control_sequence(&mut self) -> Token<'a> {
        let start = self.pos;

        // The first character is special in that a non-alphabetic
        // character is valid, but will terminate the lex.
        let end = match self.next_char() {
            None => return Token::EOF,
            Some(c) if !c.is_alphabetic() => self.pos,
            _ => {
                // Otherwise Proceed until the first non alphabetic.
                while let Some(c) = self.current_char() {
                    if !c.is_alphabetic() {
                        break;
                    }
                    self.pos += c.len_utf8();
                }
                self.pos
            }
        };

        // Consume all whitespace proceeding a control sequence
        self.advance_while_whitespace();
        Token::Command(&self.input[start..end])
    }

    /// This method will parse a dimension.  It assumes
    /// that the lexer is currently pointed to the first valid
    /// character in a dimension.  So it may be necessary to
    /// consume_whitespace() prior to using this method.
    pub fn dimension(&mut self) -> Result<Option<Unit>, Error> {
        // utter crap, rewrite.
        unimplemented!()
    }

    /// Expect to find an {<inner>}, and return <inner>
    pub fn group(&mut self) -> Result<&str, Error> {
        self.consume_whitespace();
        self.current.expect(Token::Symbol('{'))?;

        let start = self.pos;
        let end = match self.input[self.pos..].find('}') {
            Some(pos) => start + pos,
            None => return Err(Error::NoClosingBracket),
        };

        // Place cursor immediately after }
        self.pos = end + 1;
        self.next();
        Ok(&self.input[start..end])
    }

    /// Match a segment of alphanumeric characters.  This method will
    /// return an empty string if there are no alphanumeric characters.
    pub fn alphanumeric(&mut self) -> &str {
        // This method expects that the next "Token" is a sequence of
        // alphanumerics.  Since `current_char` points at the first
        // non-parsed token, we must check the current Token to proceed.
        let start = match self.current {
            Token::Symbol(c) if c.is_alphanumeric() => self.pos - c.len_utf8(),
            _ => return "",
        };

        while let Some(c) = self.current_char() {
            if !c.is_alphanumeric() {
                break;
            }
            self.pos += c.len_utf8()
        }
        let result = &self.input[start..self.pos];
        self.next();
        result
    }

    // Match a valid Color.  A color is defined as either:
    //   1. Alphabetic name for a valid CSS color.
    //   2. #RRGGBB (that is a # followed by 6 digits)
    //   3. #RRGGBBAA (that is a # followed by 8 digits)

    pub fn color(&mut self) -> Result<RGBA, Error> {
        unimplemented!()
    }

    fn next_char(&mut self) -> Option<char> {
        match self.current_char() {
            None => None,
            Some(c) => {
                self.pos += c.len_utf8();
                Some(c)
            }
        }
    }

    fn current_char(&mut self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }
}


impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Command(cmd) => write!(f, r#""\{}""#, cmd),
            Token::Symbol(c) => write!(f, r"'{}'", c),
            Token::WhiteSpace => write!(f, r"' '"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

impl<'a> fmt::Display for OwnedToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OwnedToken::Command(ref cmd) => write!(f, r#""\{}""#, cmd),
            OwnedToken::Symbol(c) => write!(f, r"'{}'", c),
            OwnedToken::WhiteSpace => write!(f, r"' '"),
            OwnedToken::EOF => write!(f, "EOF"),
        }
    }
}

impl<'a> From<Token<'a>> for OwnedToken {
    fn from(tok: Token<'a>) -> OwnedToken {
        match tok {
            Token::Command(cmd) => OwnedToken::Command(cmd.into()),
            Token::Symbol(c) => OwnedToken::Symbol(c),
            Token::WhiteSpace => OwnedToken::WhiteSpace,
            Token::EOF => OwnedToken::EOF,
        }
    }
}

#[cfg(test)]
mod tests {
    use lexer::{Lexer, Token};

    #[test]
    fn lex_tokens() {
        macro_rules! assert_eq_token_stream {
            ($left:expr, $right:expr) => {{
                let mut left  = Lexer::new($left);
                let mut right = Lexer::new($right);

                loop {
                    let l_tok = left.next();
                    let r_tok = right.next();

                    assert_eq!(l_tok, r_tok);
                    if l_tok == Token::EOF {
                        break
                    }
                }
            }}
        }

        assert_eq_token_stream!(r"\cs1", r"\cs 1");
        assert_eq_token_stream!(r"\cs1", r"\cs    1");
        assert_eq_token_stream!(r"\cs?", "\\cs\n\n\t?");
        assert_eq_token_stream!(r"\test\test", r"\test   \test");
        assert_eq_token_stream!(r"1     +       2", r"1 + 2");
        assert_eq_token_stream!(r"123\", "123");
    }

    #[test]
    fn lex_group() {
        macro_rules! assert_group {
            ($input:expr, $result:expr) => {
                let mut l = Lexer::new($input);
                assert_eq!(l.group(), $result);
                assert!(!(l.current == Token::Symbol('}')));
            }
        }

        assert_group!("{1}", Ok("1"));
        assert_group!("   {  abc } ", Ok("  abc "));
        assert_group!("{}", Ok(""));

        // This doesn't seem correct:
        // assert_group!("{{}}", Ok("{"));
    }

    #[test]
    fn lex_alphanumeric() {
        macro_rules! assert_alphanumeric {
            ($input:expr, $result:expr) => {
                let mut lex = Lexer::new($input);
                assert_eq!(lex.alphanumeric(), $result);
            }
        }

        // Ends on EOF
        assert_alphanumeric!("abc", "abc");
        assert_alphanumeric!("", "");

        // Ends on Whitespace
        assert_alphanumeric!("123 ", "123");
        assert_alphanumeric!(" 123", "");

        // End on Command
        assert_alphanumeric!(r"\pi2", "");
        assert_alphanumeric!(r"2\alpha", "2");

        // End on non-alphanumeric
        assert_alphanumeric!("{abc}", "");
        assert_alphanumeric!("abc!", "abc");
    }

    // #[test]
    // fn lex_dimension() {
    //     use dimensions::Unit;
    //     macro_rules! assert_dim {
    //         ($input:expr, $result:expr) => (
    //             let mut _l = Lexer::new($input);
    //             assert_eq!(_l.dimension().unwrap(), Some(Unit::Px($result)));
    //         )
    //     }

    //     assert_dim!(r"123 abc", 123.0);
    //     assert_dim!(r"1.23 abc", 1.23);
    //     assert_dim!(r"- 1.23 123", -1.23);
    //     assert_dim!(r"+1.34 134", 1.34);
    //     assert_dim!("-   12", -12.0);
    //     assert_dim!("+   12", 12.0);
    //     assert_dim!("-  .12", -0.12);
    //     assert_dim!("00.123000", 0.123);
    //     assert_dim!("001.10000", 1.1);
    // }
}
