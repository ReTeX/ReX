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

#[derive(Debug, Clone, Copy, Default)]
struct ParserState {
    mode:  ParsingMode,
}

impl ParserState {
    fn parse_expression(&mut self, lex: &mut Lexer) -> Result<Vec<ParseNode>, String> {
        let mut ml: Vec<ParseNode> = Vec::new();

        loop {
            lex.consume_whitespace();
            if lex.current.ends_expression() { break; }

            let node = self.parse_atom(lex)?;
            ml.push(node);
        }

        Ok(ml)
    }

    fn parse_atom(&mut self, lex: &mut Lexer) -> Result<ParseNode, String> {
        let token = lex.current;
    
        // Check for a groups and implicit groups
        if token == Token::Symbol('{') {
            lex.next();
            let ret = ParseNode::Group(self.parse_expression(lex)?);
            lex.current.expect(Token::Symbol('}'))?;
            lex.next();
            return Ok(ret);
        } if token == Token::ControlSequence("left") {
            lex.next();
            let left  = self.expect_type(lex, AtomType::Open)?;
            let inner = self.parse_expression(lex)?;
            lex.current.expect(Token::ControlSequence("right"))?;
            lex.next();
            lex.consume_whitespace();
            let right = self.expect_type(lex, AtomType::Close)?;

            return Ok(ParseNode::Delimited(Delimited{
                left: left,
                right: right,
                inner: inner,
            }));
        } if let Ok(res) = tex_command(lex) {
            lex.next();
            return Ok(res)
        }

        Ok(match token {
            Token::Symbol(c)           => { lex.next(); parse_symbol(c) },
            Token::ControlSequence(cs) => { lex.next(); parse_control(cs) },
            _ => {
                println!("Tried to match: {:?}", token);
                unreachable!()
            },
        })
    }

    fn expect_type(&mut self, lex: &mut Lexer, expected: AtomType) 
            -> Result<Symbol, String> {
        let token = lex.current;
        lex.consume_whitespace();

        let token = match token {
            Token::Symbol(c)           => parse_symbol(c),
            Token::ControlSequence(cs) => parse_control(cs),
            _ => return Err("Expect specific token".to_string()),
        };

        let ret = if let ParseNode::Symbol(sym) = token {
            if sym.atom_type == expected {
                Ok(sym)
            } else {
                Err("Got wrong symbol type".to_string())
            }
        } else {
            Err("Expected a symbol".to_string())
        };
        lex.next();
        ret
    }
}

use parser::nodes::{RadicalBuilder, TexCommand};

pub fn tex_command(lex: &mut Lexer) -> Result<ParseNode, String> {
    let mut cmd: Box<TexCommand> = match lex.current {
        Token::ControlSequence("sqrt") => Box::new(RadicalBuilder{}),
        _ => return Err("Command not found!".to_string())
    };
    lex.next();
    cmd.parse_command(lex)
}

// <Math_Field> = <filler><Symbol> | <filler>{<mathmode>}
pub fn math_field(lex: &mut Lexer) -> Result<ParseNode, String> {
    while lex.current == Token::WhiteSpace {
        lex.next();
    }
    match lex.current {
        Token::Symbol(ch) => Ok(parse_symbol(ch)),
        Token::ControlSequence(cs) => Ok(parse_control(cs)),
        _ => Err("Expected Symbol after".to_string())
    } 
}

pub fn group(lex: &mut Lexer) -> Result<ParseNode, String> {
    unimplemented!()
}

pub fn parse(input: &str) -> Result<Vec<ParseNode>, String> {
    let mut lexer = Lexer::new(input);
    let mut state = ParserState::default();

    state.parse_expression(&mut lexer)
}

fn parse_control(cs: &str) -> ParseNode {
    ParseNode::Symbol(SYMBOLS.get(cs).cloned().expect(&format!("Expected command: {}", cs)))
}

fn parse_symbol(ch: char) -> ParseNode {
    ParseNode::Symbol(ch.atom_type(FontMode::Italic).expect(&format!("Expected symbol: {}", ch)))
}

#[cfg(test)]
mod tests {
    use parser::nodes::{ ParseNode, AtomType, Radical, Delimited };
    use parser::parse;
    use symbols::Symbol;

    macro_rules! assert_vec {
        ($left:expr, $right:expr) => {
            {
                assert_eq!($left.len(), $right.len());
                for (l, r) in $left.iter().zip($right.iter()) {
                    assert_eq!(l, r);
                }
            }
        }
    }

    #[test]
    fn parser() {
        assert_eq!(parse(r"").unwrap(), vec![]);

        assert_eq!(parse(r"1 + {2 + 3}").unwrap(),
            vec![ParseNode::Symbol(Symbol { code: 120803, atom_type: AtomType::Alpha }), 
                ParseNode::Symbol(Symbol { code: 43, atom_type: AtomType::Binary }), 
                ParseNode::Group(vec![ParseNode::Symbol(Symbol { code: 120804, atom_type: AtomType::Alpha }), 
                    ParseNode::Symbol(Symbol { code: 43, atom_type: AtomType::Binary }), 
                    ParseNode::Symbol(Symbol { code: 120805, atom_type: AtomType::Alpha })
            ])]);

        assert_eq!(parse(r"1+\left(3+2\right)=6").unwrap(),
            vec![ParseNode::Symbol(Symbol { code: 120803, atom_type: AtomType::Alpha }), 
                ParseNode::Symbol(Symbol { code: 43, atom_type: AtomType::Binary }), 
                ParseNode::Delimited(Delimited { 
                    left: Symbol { code: 40, atom_type: AtomType::Open }, 
                    right: Symbol { code: 41, atom_type: AtomType::Close }, 
                    inner: vec![ParseNode::Symbol(Symbol { code: 120805, atom_type: AtomType::Alpha }), 
                       ParseNode::Symbol(Symbol { code: 43, atom_type: AtomType::Binary }), 
                       ParseNode::Symbol(Symbol { code: 120804, atom_type: AtomType::Alpha })],
                }), 
                ParseNode::Symbol(Symbol { code: 61, atom_type: AtomType::Relation }), 
                ParseNode::Symbol(Symbol { code: 120808, atom_type: AtomType::Alpha })]);
        
        assert_eq!(parse(r"1+\sqrt2").unwrap(),
            vec![ParseNode::Symbol(Symbol { code: 120803, atom_type: AtomType::Alpha }), 
                 ParseNode::Symbol(Symbol { code: 43, atom_type: AtomType::Binary }), 
                 ParseNode::Radical(Radical { 
                    inner: Box::new(ParseNode::Symbol(Symbol { code: 120804, atom_type: AtomType::Alpha })) 
                 })]);

        assert_eq!(parse(r" 1 + \sqrt   2").unwrap(), parse(r"1+\sqrt2").unwrap());
    }
}