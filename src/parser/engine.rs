#![allow(dead_code)]

use lexer::{Lexer, Token};
use symbols::{SYMBOLS, Symbol, IsSymbol, FontMode};
use parser::nodes::{ AtomType, Delimited, ParseNode };
use std::default::Default;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParsingMode {
    Math,
    Text,
    WhiteSpace,
}

impl Default for ParsingMode {
    fn default() -> ParsingMode {
        ParsingMode::Math
    }
}

// TODO: Add font family here?
#[derive(Debug, Clone, Copy, Default)]
struct ParserState {
    mode:  ParsingMode,
}

impl ParserState {
    fn parse_expression(&mut self, lex: &mut Lexer) -> Result<Vec<ParseNode>, String> {
        let mut ml: Vec<ParseNode> = Vec::new();
        lex.advance();
        
        loop {
            let token = match lex.current() {
                Some(t) => t,
                None => break,
            };

            // Consume whitespaces as necessary
            if token == Token::WhiteSpace {
                if self.mode != ParsingMode::Math
                && self.mode != ParsingMode::WhiteSpace {
                    self.mode = ParsingMode::WhiteSpace;
                    ml.push(ParseNode::Symbol(Symbol {
                        code: ' ' as u32,
                        atom_type: AtomType::Ordinal,
                    }))
                }
                lex.advance();
                continue;
            }

            // Handle end of expressions
            if token.ends_expression() { break; }

            let node = self.parse_atom(lex, token).unwrap();
            ml.push(node);
        }

        Ok(ml)
    }

    fn parse_atom(&mut self, lex: &mut Lexer, token: Token) -> Result<ParseNode, String> {
        // Check for a groups and implicit groups
        if token == Token::Symbol('{') {
            let ret = ParseNode::Group(try!(self.parse_expression(lex)));
            try!(lex.expect_and_advance(Token::Symbol('}')));
            return Ok(ret);
        } if token == Token::ControlSequence("left") {
            lex.advance();
            let left  = match lex.current().unwrap() {
                Token::Symbol(c) => parse_symbol(c),
                _ => return Err("Expected delimiter".to_string())
            };
            let inner = try!(self.parse_expression(lex));
            try!(lex.expect_and_advance(Token::ControlSequence("right")));
            let right = match lex.current().unwrap() {
                Token::Symbol(c) => parse_symbol(c),
                _ => return Err("Expected delimiter".to_string())
            };
            lex.advance();

            return Ok(ParseNode::Delimited(Delimited{
                left: match left {
                    ParseNode::Symbol(s) => s,
                    _ => unreachable!(),
                },
                right: match right {
                    ParseNode::Symbol(s) => s,
                    _ => unreachable!(),   
                },
                inner: inner,
            }));
        }

        Ok(match token {
            Token::Symbol(c)           => { lex.advance(); parse_symbol(c) },
            Token::ControlSequence(cs) => { lex.advance(); parse_control(cs) },
            _ => unreachable!(),
        })
    }
}



pub fn parse(input: &str) -> Result<Vec<ParseNode>, String> {
    let mut lexer = Lexer::new(input);
    let mut state = ParserState::default();

    state.parse_expression(&mut lexer)
}

fn parse_control(cs: &str) -> ParseNode {
    ParseNode::Symbol(SYMBOLS.get(cs).cloned().unwrap())
}

fn parse_symbol(ch: char) -> ParseNode {
    ParseNode::Symbol(ch.atom_type(FontMode::Italic).unwrap())
}

#[cfg(test)]
mod tests {
    use super::parse;
    #[test]
    fn parser() {
        println!("");
        println!("'a+b=c': {:?}", parse("a+b=c").unwrap());
        println!("'\\int(x+y)=z': {:?}", parse(r"\int(x+y)=z").unwrap());
        println!("'1 + {{2 + 3}} = 4': {:?}", parse(r"1 + {2 + 3} = 4").unwrap());
        println!("'1+\\left(3+2\\right)=6': {:?}", parse(r"1+\left(3+2\right)=6").unwrap());
    }
}