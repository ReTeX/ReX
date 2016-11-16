#![allow(dead_code)]
use super::{ Lexer, Token };

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
        use font::fontselection::{Family, Weight};
        let mut lex = Lexer {
            input: input,
            current: Token::EOF,
            pos: 0,
            family: Family::Normal,
            weight: Weight::None,
        };

        lex.next();
        lex
    }

    /// Advanced to the next token to be processed, and
    /// return it.

    pub fn next(&mut self) -> Token<'a> {
        self.current = match self.next_char() {
            None       => Token::EOF,
            Some(' ') | Some('\t') | Some('\r') | Some('\n')  
                       => Token::WhiteSpace,
            Some('\\') => self.control_sequence(),
            Some(c)    => Token::Symbol(c),
        };

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
            match c {
                ' ' | '\t' | '\r' | '\n' => self.pos += 1,
                _ => { break; },
            }
        }

        // The cursor (self.pos) now points to the first
        // non-whitespace character.  Lex this token to
        // place it as currently processed token
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
                self.consume_whitespace();
                return Token::ControlSequence(&self.input[start..end]);
            }
            _ => { /* Otherwise we have an alphabetric, stop at next non alphabetic */ }
        };

        while let Some(c) = self.next_char()  {
            if !c.is_alphabetic() {
                // backtrack, this is not part of the cs name
                self.pos -= c.len_utf8();
                break
            }
        };

        // We can not relay on the `self.consume_whitespace()` method 
        // since it assumes that `self.current` points to the token
        // that is currently being processed -- this control sequence,
        // which we have not yet placed into `self.current`.
        let end = self.pos;
        while let Some(c) = self.current_char() {
            match c {
                ' ' | '\t' | '\r' | '\n' => self.pos += 1,
                _ => { break; },
            }
        }

        Token::ControlSequence(&self.input[start..end])
    }

    /// This method will parse a dimension.  It assumes
    /// that the lexer is currently pointed to the first valid 
    /// character in a dimension.  So it may be necessary to 
    /// consume_whitespace() prior to using this method.

    pub fn dimension(&mut self) -> Option<u32> {
        // We need to backtrack.
        match self.current {
            Token::Symbol(sym) => self.pos -= sym.len_utf8(),
            // TODO: If we accept dimensions such as \textwidth
            //  we need to recognize them here, for now we don't.
            _ => return None,
        }

        let pos = self.pos;
        while let Some(n) = self.next_char() {
            if !n.is_numeric() {
                self.pos -= n.len_utf8();
                break
            }
        }

        if pos == self.pos { return None }
        let result = Some(self.input[pos..self.pos].parse::<u32>().unwrap());
        self.next();
        result
    }

    /// This method and `current_char` return the same value.
    /// The only difference is that this method will advance
    /// the cursor.

    #[inline]
    fn next_char(&mut self) -> Option<char> {
        match self.current_char() {
            None => None,
            Some(c) => {
                self.pos += c.len_utf8();
                Some(c)
            }
        }
    }

    #[inline]
    fn current_char(&mut self) -> Option<char> {
        self.input[self.pos..].chars().next()
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
    fn lex_numbers() {
        let mut lex = Lexer::new(r"123 abc");
        assert_eq!(lex.dimension(), Some(123));
    }
}