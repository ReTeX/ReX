use lexer::{Lexer, LexToken};
//use parsenodes::*;
use symbols::{MATHUNICODE, Symbol, IsSymbol};

#[derive(Debug)]
enum ParsingMode {
    Math,
    Text
}

// TODO: Add font family here?
struct Parser {
    //mode: ParsingMode,
}

impl Parser {
    pub fn parse(&mut self, input: &str) -> Result<Vec<Symbol>, String> {
        // initialize parsing state
        let mut ml: Vec<Symbol> = Vec::new();
        let mut lexer = Lexer::new(input);

        // TODO implement iterator for lexer
        loop {
            let next = lexer.next_token();
            let node = match next {
                LexToken::WhiteSpace          => continue,
                LexToken::EOF                 => break,
                LexToken::Symbol(c)           => parse_symbol(&c).expect("Symbol not found"),
                LexToken::ControlSequence(cs) => parse_control(&cs).expect("Control sequence not found"),
            };
            ml.push(node);
        }

        Ok(ml)
    }
}

fn parse_control(cs: &str) -> Option<Symbol> {
    MATHUNICODE.get(cs).cloned()
}

fn parse_symbol(ch: &char) -> Option<Symbol> {
    Some(Symbol {
        code: *ch as u32,
        atom_type: ch.atom_type().unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::Parser;
    #[test]
    fn parser() {
        let mut p = Parser {};

        println!("");
        println!("'a+b=c': {:?}", p.parse("a+b=c").unwrap());
        println!("'\\int(x+y)=z': {:?}", p.parse(r"\int(x+y)=z").unwrap());
    }
}