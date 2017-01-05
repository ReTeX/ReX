#![allow(dead_code)]
use super::{ Lexer, Token };
use dimensions::Unit;

/// The Lexer API.  No method here should fail.


// lexer::new(input)

// lex.current        current token
// lex.next()         consume current token, advance to next
// lex.expect(Token)  assert lex.current = Token, then consume
// lex.assert(Token)  assert current token is Token (needed?)   ??

// TODO: Implement a replacement unicode for malformed unicode
//       or issued a EOF

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

    /// Advanced to the next token to be processed, and
    /// return it.

    pub fn next(&mut self) -> Token<'a> {
        self.current = match self.next_char() {
            Some(c) if c.is_whitespace() => Token::WhiteSpace,
            Some('\\') => self.control_sequence(),
            Some(c)    => Token::Symbol(c),
            None       => Token::EOF,
        };

        //println!("{:?}", self.current);
        self.current
    }

    /// If the lexer is currently pointed to a WhiteSpace
    /// token, then advance until you reach the first
    /// non-WhiteSpace token.  This method is indepotent, so
    /// calling this method twice has no effect.

    pub fn consume_whitespace(&mut self) {
        if self.current != Token::WhiteSpace { return; }

        // while the current character points to a
        // whitespace token, advance the position.
        while let Some(c) = self.current_char() {
            if !c.is_whitespace() { break; }
            self.pos += 1;
        }

        // The cursor (self.pos) now points to the first
        // non-whitespace character.  Lex this token to
        // place it as current token to process
        self.next();
    }

    /// Lex a control sequence.  This method assumes that
    /// we are currently pointing to the first character
    /// after `\`.  This method continues advancing the cursor
    /// until a first non-alphabetic character is reached.
    /// The first character is special, in that a non-alphabetic
    /// character is valid, but will terminate the advance.
    /// Afterwards, this method consumes all whitespace characters.

    fn control_sequence(&mut self) -> Token<'a> {
        let start = self.pos;

        // The first character is special in that non-alphabetic
        // characters are valid, but will terminate the search.
        match self.next_char() {
            None => return Token::EOF,
            Some(c) if !c.is_alphabetic() => {
                let end = self.pos;
                while let Some(c) = self.current_char() {
                    if !c.is_whitespace() { break; }
                    self.pos += 1;
                }
                return Token::Command(&self.input[start..end]);
            },
            _ => { /* Otherwise we have an alphabetric, stop at next non alphabetic */ },
        };

        while let Some(c) = self.next_char()  {
            if c.is_alphabetic() { continue; }

            // backtrack, this is not part of the cs name
            self.pos -= c.len_utf8();
            break;
        };

        // We can not relay on the `self.consume_whitespace()` method
        // since it assumes that `self.current` points to the token
        // that is currently being processed--this control sequence--which
        // we have not yet placed into `self.current`.
        let end = self.pos;
        while let Some(c) = self.current_char() {
            if !c.is_whitespace() { break; }
            self.pos += 1;
        }

        Token::Command(&self.input[start..end])
    }

    /// This method will parse a dimension.  It assumes
    /// that the lexer is currently pointed to the first valid
    /// character in a dimension.  So it may be necessary to
    /// consume_whitespace() prior to using this method.

    pub fn dimension(&mut self) -> Result<Option<Unit>, String> {
        // Parse optional sign
        let sign = self.possible_sign() as f64;
        self.consume_whitespace();

        // We should've hit our first numeric value
        // Backtrack to parse this favlue.
        self.backtrack();
        let pos = self.pos;
        while let Some(n) = self.next_char() {
            if n.is_numeric() || n == '.' { continue; }
            self.pos -= n.len_utf8();
            break
        }

        // Unable to find any numeric values
        // Otherwise parse the result using the standard library.
        if pos == self.pos { return Ok(None) }
        let num = sign * self.input[pos..self.pos].parse::<f64>()
            .or(Err("Unable to parse dimension!".to_string()))?;
        let result = Some(Unit::Px(num));

        // TODO: Handle dimensions, px, em, etc.
        self.next();
        Ok(result)
    }

    /// Expect to find an {<inner>}, and return <inner>

    pub fn group(&mut self) -> Result<&str, String> {
       // println!("{:?}", self);
        self.consume_whitespace();
        if self.current != Token::Symbol('{') {
            return Err("Expected to find an open group.".into())
        }

        let start = self.pos;
        let mut end   = self.pos;
        while let Some(c) = self.next_char() {
            if c == '{' {
                self.pos -= 1;
                end = self.pos - 1;
                self.next();
                break;
            }
        }

        if start == end {
            return Err("Unable to find closing bracket.".into())
        }

       // println!("{}", &self.input[start..end]);
        //println!("{:?}", self);
        Ok(&self.input[start..end])
    }

    /// This method and `current_char` return the same value.
    /// The only difference is that this method will advance
    /// the cursor.

    fn possible_sign(&mut self) -> i8 {
        match self.current {
            Token::Symbol('-') => { self.next(); -1 },
            Token::Symbol('+') => { self.next(); 1 },
            _ => 1,
        }
    }

    fn backtrack(&mut self) {
        match self.current {
            Token::Symbol(sym) => self.pos -= sym.len_utf8(),
            Token::WhiteSpace => {
                self.pos -= 1;
                while let Token::WhiteSpace = self.current {
                    self.pos -= 1;
                }
            },
            Token::Command(_) => {
                /* TODO: Implement me */
            },
            _ => { /* EOF? */ }
        }
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


trait CanBeWhitespace {
    fn is_whitespace(&self) -> bool;
}

impl CanBeWhitespace for char {
    fn is_whitespace(&self) -> bool {
        match *self {
            ' ' | '\t' | '\n' | '\r' => true,
            _ => false,
        }
    }
}



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

#[cfg(test)]
mod tests {
    use super::super::{Lexer, Token};

    #[test]
    fn lex_tokens() {
        assert_eq_token_stream!(r"\cs1", r"\cs 1");
        assert_eq_token_stream!(r"\cs1", r"\cs  1");
        assert_eq_token_stream!(r"\cs1", "\\cs\n\t\r 1");
        assert_eq_token_stream!(r"\test\test", r"\test  \test");
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