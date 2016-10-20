#![allow(dead_code)]

use lexer::{Lexer, LexToken};
use symbols::{SYMBOLS, Symbol, IsSymbol, FontMode};
use parser::nodes::AtomType;
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
    mode: ParsingMode,
}

fn parse_expression(mut lex: Lexer, mut state: ParserState) -> Result<Vec<Symbol>, String> {
    let mut ml: Vec<Symbol> = Vec::new();
    loop {
        let token = lex.next_token();
        let node = match token {
            LexToken::WhiteSpace => {
                if state.mode == ParsingMode::Math ||
                   state.mode == ParsingMode::WhiteSpace {
                    continue;
                } else {
                    state.mode = ParsingMode::WhiteSpace;
                    Symbol {
                        code: ' ' as u32,
                        atom_type: AtomType::Ordinal,
                    }
                }
            },
            LexToken::EOF                 => break,
            LexToken::Symbol(c)           => parse_symbol(&c).expect("Symbol not found"),
            LexToken::ControlSequence(cs) => parse_control(&cs).expect("Control sequence not found"),
        };

        ml.push(node);
    }

    Ok(ml)
}

pub fn parse(input: &str) -> Result<Vec<Symbol>, String> {
    let lexer = Lexer::new(input);
    let state = ParserState::default();

    parse_expression(lexer, state)
}

fn parse_control(cs: &str) -> Option<Symbol> {
    SYMBOLS.get(cs).cloned()
}

fn parse_symbol(ch: &char) -> Option<Symbol> {
    ch.atom_type(FontMode::Italic)
}

#[cfg(test)]
mod tests {
    use super::parse;
    #[test]
    fn parser() {
        println!("");
        println!("'a+b=c': {:?}", parse("a+b=c").unwrap());
        println!("'\\int(x+y)=z': {:?}", parse(r"\int(x+y)=z").unwrap());
    }
}