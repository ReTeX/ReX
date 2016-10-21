#![allow(dead_code)]

#[derive(Clone, Copy, Debug)]
struct LexerState<'a> {
    input: &'a str,
    pos: usize,                    // cursor position of lexer
}

pub struct Lexer {
    state: Lexer
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'a> {
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

    pub fn next_token(&mut self) -> Token {
        match self.next_char() {
            None       => Token::EOF,
            Some(' ')  => Token::WhiteSpace,
            Some('\\') => self.extract_control_sequence(),
            Some(c)    => Token::Symbol(c),
        }
    }

    // This assumes that we have consumed '\'
    // remember to consume all spaces following it.
    fn extract_control_sequence(&mut self) -> Token {
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
                // We need to backtrack as to not include 
                // non-alphabetic in control sequence
                self.pos -= c.len_utf8();
                break
            }
        };

        let end = self.pos;
        self.consume_whitespace();
        Token::ControlSequence(&self.input[start..end])
    }

    fn consume_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            match c {
                ' ' | '\t' | '\r' | '\n' => self.pos += 1,
                _ => break,
            }
        }
    }

    #[inline]
    fn next_char(&mut self) -> Option<char> {
        match self.peek_char() {
            None => None,
            Some(c) => {
                self.pos += c.len_utf8();
                Some(c)
            }
        }
    }

    #[inline]
    fn peek_char(&mut self) -> Option<char> {
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