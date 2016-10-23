
use lexer::Lexer;
use parser;
use parser::nodes::{ ParseNode, Radical, GenFraction };
use symbols::Symbol;

// For now, we will use trait objects to handle custom parsing.
pub trait TexCommand {
    fn parse_command(&mut self, &mut Lexer) -> Result<Option<ParseNode>, String>;
}

#[derive(Debug)]
pub struct RadicalBuilder { }

impl TexCommand for RadicalBuilder {
    fn parse_command(&mut self, lex: &mut Lexer) -> Result<Option<ParseNode>, String> {
        Ok(Some(ParseNode::Radical(Radical {
            inner: parser::required_macro_argument(lex)?,
        })))
    }
}

#[derive(Debug)]
pub struct GenFractionBuilder { 
    pub left_delimiter: Option<Symbol>,
    pub right_delimiter: Option<Symbol>,
    pub bar_thickness: u32,        // I need metrics
    //math_style: Option<()>,    // I need math styles
}

impl TexCommand for GenFractionBuilder {
    fn parse_command(&mut self, lex: &mut Lexer) -> Result<Option<ParseNode>, String> {
        Ok(Some(ParseNode::GenFraction(GenFraction {
            numerator: parser::required_macro_argument(lex)?,
            denominator: parser::required_macro_argument(lex)?,
            left_delimiter: self.left_delimiter,
            right_delimiter: self.right_delimiter,
            bar_thickness: self.bar_thickness,
        })))
    }
}