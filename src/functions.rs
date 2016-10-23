
use lexer::Lexer;
use parser;
use parser::nodes::{ ParseNode, Radical };

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