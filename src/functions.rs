use phf;
use font::Symbol;
use parser::nodes::{ AtomType, ParseNode, Radical, GenFraction, Rule, BarThickness };
use spacing::Spacing;
use lexer::Lexer;
use parser;
use parser::Locals;
use dimensions::Unit;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MathStyle {
    Display,
    Text,
    Script,
    ScriptScript,
    NoChange,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TexCommand {
    Radical,
    Rule,
    Extend,
    GenFraction {
        left_delimiter:  Option<Symbol>,
        right_delimiter: Option<Symbol>,
        bar_thickness:   BarThickness,
        math_style:      MathStyle,
    },
    DelimiterSize {
        atom_type: AtomType,
        size:      u8,
    },
    Spacing {
        size: Spacing,
    },
    Kerning(Unit),
}

/// With this method you can pass a parser function which will
/// first unwrap any group items if there are any.

// fn unwrap_with<T: Sized>(f: &FnMut(&mut Lexer, Locals) -> Result<Option<T>, String>)
//         -> Result<Option<T>, String> {
//     unimplemented!()
// }

impl TexCommand {
    #[allow(dead_code, unused_variables)]
    pub fn parse(self, lex: &mut Lexer, local: Locals) -> Result<Option<ParseNode>, String> {
        Ok(match self {
            TexCommand::Radical =>
                Some(ParseNode::Radical(Radical {
                    inner: parser::required_macro_argument(lex, local)?,
                })),
            TexCommand::GenFraction {
                left_delimiter:  ld,
                right_delimiter: rd,
                bar_thickness:   bt,
                math_style:      ms,
            } =>
                Some(ParseNode::GenFraction(GenFraction{
                    left_delimiter:  ld,
                    right_delimiter: rd,
                    bar_thickness:   bt,
                    numerator: parser::required_macro_argument(lex, local)?,
                    denominator: parser::required_macro_argument(lex, local)?,
                })),
            TexCommand::DelimiterSize {
                size: s,
                atom_type: at,
            } =>
                Some(ParseNode::Symbol(parser::expect_type(lex, local, at)?)),
            TexCommand::Spacing { size: sp } =>
                Some(ParseNode::Spacing(sp)),
            TexCommand::Rule => {
                lex.consume_whitespace();
                let w = lex.dimension()?
                    .expect("Unable to parse dimension for Rule.");
                lex.consume_whitespace();
                let h = lex.dimension()?
                    .expect("Unable to parse dimension for Rule.");

                Some(ParseNode::Rule(Rule {
                    width: w,
                    height: h,
                }))
            },
            TexCommand::Kerning(k) =>
                Some(ParseNode::Kerning(k)),
            TexCommand::Extend => {              // Only used for testing, for now.
                let h = lex.dimension()?
                    .expect("Unable to parse dimension for Extend.");
                Some(ParseNode::Extend(h))
            }
        })
    }
}

pub static COMMANDS: phf::Map<&'static str, TexCommand> = phf_map! {
    "frac" => TexCommand::GenFraction { left_delimiter: None, right_delimiter: None,  bar_thickness: BarThickness::Default, math_style: MathStyle::NoChange },
    "tfrac" => TexCommand::GenFraction { left_delimiter: None, right_delimiter: None, bar_thickness: BarThickness::Default, math_style: MathStyle::Text },
    "dfrac" => TexCommand::GenFraction { left_delimiter: None, right_delimiter: None, bar_thickness: BarThickness::Default, math_style: MathStyle::Display },
    "binom" => TexCommand::GenFraction { left_delimiter: Some(Symbol { unicode: '{' as u32, atom_type: AtomType::Open }), right_delimiter: Some(Symbol { unicode: '}' as u32, atom_type: AtomType::Close }), bar_thickness:  BarThickness::None, math_style: MathStyle::NoChange },
    "tbinom" => TexCommand::GenFraction { left_delimiter: Some(Symbol { unicode: '{' as u32, atom_type: AtomType::Open }), right_delimiter: Some(Symbol { unicode: '}' as u32, atom_type: AtomType::Close }), bar_thickness: BarThickness::None, math_style: MathStyle::Text },
    "dbinom" => TexCommand::GenFraction { left_delimiter: Some(Symbol { unicode: '{' as u32, atom_type: AtomType::Open }), right_delimiter: Some(Symbol { unicode: '}' as u32, atom_type: AtomType::Close }), bar_thickness: BarThickness::None, math_style: MathStyle::Display },
    "sqrt" => TexCommand::Radical,
    "bigl" => TexCommand::DelimiterSize { size: 1, atom_type: AtomType::Open },
    "Bigl" => TexCommand::DelimiterSize { size: 2, atom_type: AtomType::Open },
    "biggl" => TexCommand::DelimiterSize { size: 3, atom_type: AtomType::Open },
    "Biggl" => TexCommand::DelimiterSize { size: 4, atom_type: AtomType::Open },
    "bigr" => TexCommand::DelimiterSize { size: 1, atom_type: AtomType::Close },
    "Bigr" => TexCommand::DelimiterSize { size: 2, atom_type: AtomType::Close },
    "biggr" => TexCommand::DelimiterSize { size: 3, atom_type: AtomType::Close },
    "Biggr" => TexCommand::DelimiterSize { size: 4, atom_type: AtomType::Close },
    "bigm" => TexCommand::DelimiterSize { size: 1, atom_type: AtomType::Relation },
    "Bigm" => TexCommand::DelimiterSize { size: 2, atom_type: AtomType::Relation },
    "biggm" => TexCommand::DelimiterSize { size: 3, atom_type: AtomType::Relation },
    "Biggm" => TexCommand::DelimiterSize { size: 4, atom_type: AtomType::Relation },
    "big" => TexCommand::DelimiterSize { size: 1, atom_type: AtomType::Ordinal },
    "Big" => TexCommand::DelimiterSize { size: 2, atom_type: AtomType::Ordinal },
    "bigg" => TexCommand::DelimiterSize { size: 3, atom_type: AtomType::Ordinal },
    "Bigg" => TexCommand::DelimiterSize { size: 4, atom_type: AtomType::Ordinal },
    "!" => TexCommand::Kerning(Unit::Em(-3f64/18f64)),
    "," => TexCommand::Kerning(Unit::Em(3f64/18f64)),
    ":" => TexCommand::Kerning(Unit::Em(4f64/18f64)),
    " " => TexCommand::Kerning(Unit::Em(1f64/4f64)),
    ";" => TexCommand::Kerning(Unit::Em(5f64/18f64)),
    "quad" => TexCommand::Kerning(Unit::Em(1.0f64)),
    "qquad" => TexCommand::Kerning(Unit::Em(2.0f64)),
    "rule" => TexCommand::Rule,
    "extend" => TexCommand::Extend,
};