use phf;
use lexer::Lexer;
use parser;
use parser::nodes::{ ParseNode, Radical, GenFraction };
use symbols::Symbol;

enum MathStyle {
    Display,
    Text,
    Script,
    ScriptScript,
    NoChange,
}

enum TexCommand {
    Radical,
    GenFraction {
        pub left_delimiter: Option<Symbol>,
        pub right_delimiter: Option<Symbol>,
        pub bar_thickness: u32,
        pub math_style: Option<MathStyle>,
    },
}

macro_rules! gen_frac {
    ($l:expr, $r:expr, $b:expr, $m:expr) => (
        TexCommand::GenFraction{ 
            pub left_delimiter: $l,
            pub right_delimiter: $r,
            pub bar_thickness: $b,
            pub math_style: $m,            
         }
    )
    ($l:expr, $r:expr, $b:expr) => (
        TexCommand::GenFraction{ 
            pub left_delimiter: $l,
            pub right_delimiter: $r,
            pub bar_thickness: $b,
            pub math_style: MathStyle::NoChange,            
         }
    )
}

pub static COMMANDS: phf::Map<&'static str, TexCommand> = phf_map! {
    "sqrt" => TexCommand::Radical,
    "frac" => gen_frac!{ None, None, 4 }
    "binom" => gen_frac!{ Symbol{ code: '(', size)}}
    ""
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