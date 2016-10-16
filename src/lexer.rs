#![allow(dead_code)]

#[derive(Clone, Copy, Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LexToken<'a> {
    ControlSequence(&'a str),
    Symbol(char),
    WhiteSpace,
    EOF,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input,
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> LexToken {
        match self.next_char() {
            None       => LexToken::EOF,
            Some(' ')  => LexToken::WhiteSpace,
            Some('\\') => self.extract_control_sequence(),
            Some(c)    => LexToken::Symbol(c),
        }
    }

    // This assumes that we have consumed '\'
    // remember to consume all spaces following it.
    pub fn extract_control_sequence(&mut self) -> LexToken {
        let start = self.pos;

        // The first character is special in that non-alphabetic
        // characters are valid, but will terminate the search.
        match self.next_char() {
            None => return LexToken::EOF,
            Some(c) if !c.is_alphabetic() => {
                let end = self.pos;
                self.consume_whitespace();
                return LexToken::ControlSequence(&self.input[start..end]);
            }
            _ => { /* Otherwise we have an alphabetric, stop at next non alphabetic */ }
        };

        while let Some(c) = self.next_char()  {
            if !c.is_alphabetic() {
                // We need to backtrack as to not include 
                // non-alphabetic in control sequence
                self.pos -= c.len_utf8();
                break
            }
        };

        let end = self.pos;
        self.consume_whitespace();
        LexToken::ControlSequence(&self.input[start..end])
    }

    pub fn consume_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            match c {
                ' ' | '\t' | '\r' | '\n' => self.pos += 1,
                _ => break,
            }
        }
    }

    #[inline]
    pub fn next_char(&mut self) -> Option<char> {
        match self.peek_char() {
            None => None,
            Some(c) => {
                self.pos += c.len_utf8();
                Some(c)
            }
        }
    }

    #[inline]
    pub fn peek_char(&mut self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }
}

macro_rules! print_token_stream {
    ($x:expr) => {{
        let mut lex = Lexer::new($x);
        loop {
            let tok = lex.next_token();
            print!("{:?}", tok);
            if tok == LexToken::EOF {
                break
            }
        }
    }}
}

macro_rules! assert_eq_token_stream {
    ($left:expr, $right:expr) => {{
        let mut left  = Lexer::new($left);
        let mut right = Lexer::new($right);

        loop {
            let l_tok = left.next_token();
            let r_tok = right.next_token();

            assert_eq!(l_tok, r_tok);

            if l_tok == LexToken::EOF {
                break
            }
        }
    }}
}

#[cfg(test)]
mod tests {
    use super::{Lexer, LexToken};

    #[test]
    fn lextokens() {
        assert_eq_token_stream!(r"\cs1", r"\cs 1");
        assert_eq_token_stream!(r"\cs1", r"\cs  1");
        assert_eq_token_stream!(r"\cs1", "\\cs\n\t\r 1");
        assert_eq_token_stream!(r"\test\test", r"\test  \test");
    }
}