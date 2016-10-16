use lexer::{Lexer, LexToken};
use parsenodes::*;

#[derive(Debug)]
enum ParsingMode {
    Math,
    Text
}

// TODO: Add font family here?
struct Parser {
    mode: ParsingMode,
}

impl Parser {
    pub fn parse(&mut self, input: &str) -> Result<MathList, String> {
        // initialize parsing state
        let mut ml = MathList::new();
        let mut lexer = Lexer::new(input);

        loop {
            let next = lexer.next_token();
            let node = match next {
                LexToken::WhiteSpace => continue,
                LexToken::EOF => break,
                LexToken::Symbol(c) => parse_symbol(&c).expect("No symbol"),
                LexToken::ControlSequence(cs) => parse_control(&cs).unwrap(),
                //_ => break
            };
            ml.push(node);
        }

        Ok(ml)
    }
}

fn parse_control(cs: &str) -> Option<ParseNode> {
    unimplemented!();
}

fn parse_symbol(ch: &char) -> Option<ParseNode> {
    match *ch {
        'a'...'z' | 'A'...'Z' | '1'...'9' => 
            Some(ParseNode::Symbol(Symbol{
                font_family: 1u8,
                code_point: *ch as u32,
                height: 0,
                width: 0,
                depth: 0,
                atomclass: AtomClass::Ordinal,
            })),
        
        '+' | '-' | '*' =>
            Some(ParseNode::Operator(Operator{
                symbol: Symbol {
                    font_family: 1u8,
                    code_point: *ch as u32,
                    height: 0,
                    width: 0,
                    depth: 0,
                    atomclass: AtomClass::Ordinal,                    
                },
                limits: true,
                largeop: false,
                successor: None,
            })),

        '=' | '<' | '>' => 
            Some(ParseNode::Symbol(Symbol {
                font_family: 1u8,
                code_point: *ch as u32,
                height: 0,
                width: 0,
                depth: 0,
                atomclass: AtomClass::Relation,                
            })),

        '(' | ')' | '[' | ']' => 
            Some(ParseNode::Delimiter(Delimiter{
                symbol: Symbol {
                    font_family: 1u8,
                    code_point: *ch as u32,
                    height: 0,
                    width: 0,
                    depth: 0,
                    atomclass: AtomClass::Delimiter,                    
                },
                successor: None,
            })),

        '.' | ','  => 
            Some(ParseNode::Symbol(Symbol {
                font_family: 1u8,
                code_point: *ch as u32,
                height: 0,
                width: 0,
                depth: 0,
                atomclass: AtomClass::Punctuation,                
            })),

        _ => { println!("Here: {:?}", *ch); None },
    }
}

#[cfg(test)]
mod tests {
    use super::{Parser, ParsingMode};
    use parsenodes::ParseNode;
    #[test]
    fn parser() {
        let mut p = Parser {
            mode: ParsingMode::Math
        };

        println!("");
        let ml = p.parse("a+b=c").unwrap();

        for item in ml {
            println!("{:#?}", item);
        }

        println!("\n");
        let m2 = p.parse("1 + (3 * 4) = 7").unwrap();

        for item in m2 {
            println!("{:#?}", item);
        }        

    }

}