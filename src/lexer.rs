use std::fmt;
use dimensions::Unit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Command(&'a str),
    Symbol(char),
    WhiteSpace,
    EOF,
}

impl<'a> Token<'a> {
    pub fn ends_expression(&self) -> bool {
        match *self {
            Token::EOF |
            Token::Symbol('}') |
            Token::Command("right") |
            Token::Command(r"\") => true,
            _ => false,
        }
    }

    pub fn expect(&self, expected: Token<'a>) -> Result<(), String> {
        if *self == expected {
            Ok(())
        } else {
            Err(format!("Expected token '{:?}', found the token '{:?}'",
                        expected,
                        self))
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Command(cmd) => write!(f, r"\{}", cmd),
            Token::Symbol(c) => write!(f, r"'{}'", c),
            Token::WhiteSpace => write!(f, r"' '"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

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

impl<'a> Lexer<'a> {
    /// Create a new lexer, whose current token is the first token
    /// to be processed.

    pub fn new(input: &'a str) -> Lexer<'a> {
        let mut lex = Lexer {
            input: input,
            current: Token::EOF,
            pos: 0,
        };

        lex.next();
        lex
    }

    /// Advanced to the next token to be processed, and return it.
    /// This will also modify `Lexer.current`.

    pub fn next(&mut self) -> Token<'a> {
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
        // characters is valid, but will terminate the lex.
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

    pub fn dimension(&mut self) -> Result<Option<Unit>, String> {
        // utter crap, rewrite.
        unimplemented!()
    }

    /// Expect to find an {<inner>}, and return <inner>

    pub fn group(&mut self) -> Result<&str, String> {
        self.consume_whitespace();
        if self.current != Token::Symbol('{') {
            return Err("Expected to find an open group.".into());
        }

        let start = self.pos;
        let mut found = false;
        while let Some(c) = self.current_char() {
            self.pos += c.len_utf8();
            if c == '}' {
                found = true;
                break;
            }
        }

        if !found {
            return Err("Unable to find closing bracket.".into());
        }

        self.next();
        let end = self.pos - 1;
        Ok(&self.input[start..end])
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

#[cfg(test)]
mod tests {
    use lexer::{Lexer, Token};

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

    macro_rules! print_token_stream {
        ($expr:expr) => {{
            let mut lex = Lexer::new($expr);
            print!("'{:?}': ", $expr);
            loop {
                let tok = lex.current;
                if tok == Token::EOF { break; }
                print!("{:?}", tok);
                lex.next();
            }
            println!("");
        }}
    }

    #[test]
    fn lex_tokens() {
        assert_eq_token_stream!(r"\cs1", r"\cs 1");
        assert_eq_token_stream!(r"\cs1", r"\cs    1");
        assert_eq_token_stream!(r"\cs?", "\\cs\n\n\t?");
        assert_eq_token_stream!(r"\test\test", r"\test   \test");
        assert_eq_token_stream!(r"1     +       2", r"1 + 2");
        assert_eq_token_stream!(r"123\", "123");
    }

    #[test]
    fn lex_dimension() {
        use dimensions::Unit;
        macro_rules! assert_dim {
            ($input:expr, $result:expr) => (
                let mut _l = Lexer::new($input);
                assert_eq!(_l.dimension().unwrap(), Some(Unit::Px($result)));
            )
        }

        assert_dim!(r"123 abc", 123.0);
        assert_dim!(r"1.23 abc", 1.23);
        assert_dim!(r"- 1.23 123", -1.23);
        assert_dim!(r"+1.34 134", 1.34);
        assert_dim!("-   12", -12.0);
        assert_dim!("+   12", 12.0);
        assert_dim!("-  .12", -0.12);
        assert_dim!("00.123000", 0.123);
        assert_dim!("001.10000", 1.1);
    }
}