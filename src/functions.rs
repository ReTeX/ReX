use dimensions::Unit;
use font::{Weight, Family, Symbol, AtomType};
use font::Style as FontStyle;
use font::style::style_symbol;
use layout::Style;
use lexer::{Lexer, Token};
use parser as parse;
use parser::nodes::{ParseNode, Radical, GenFraction, Rule, BarThickness, AtomChange, Color, Stack};
use parser::color::RGBA;
use static_map;
use error::{Error, Result};

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    Radical,
    Rule,
    VExtend,
    HExtend,
    Color,
    ColorLit(RGBA),
    GenFraction(Option<Symbol>, Option<Symbol>, BarThickness, MathStyle),
    DelimiterSize(u8, AtomType),
    Kerning(Unit),
    Style(Style),
    AtomChange(AtomType),
    TextOperator(&'static str, bool),
    Stack(AtomType),
}

macro_rules! sym {
    (@at ord) => { AtomType::Ordinal };
    (@at bin) => { AtomType::Binary };
    (@at op)  => { AtomType::Operator };
    (@at open) => { AtomType::Open };
    (@at close) => { AtomType::Close };

    ($code:expr, $ord:ident) => ({
        Some(Symbol {
            unicode: $code as u32,
            atom_type: sym!(@at $ord),
        })
    });
}

macro_rules! text {
    ($code:expr) => ({

        ParseNode::Symbol(Symbol {
            unicode: style_symbol(
                $code as u32,
                FontStyle::default()
                    .with_family(Family::Roman)
                    .with_weight(Weight::None)),
            atom_type: AtomType::Ordinal,
        })
    })
}

fn text(s: &'static str) -> Vec<ParseNode> {
    let mut result = Vec::with_capacity(s.len());
    for c in s.chars() {
        if c == ',' {
            result.push(ParseNode::Kerning(Unit::Em(3f64/18f64)));
        } else {
            result.push(text!(c));
        }
    }
    result
}

pub static COMMANDS: static_map::Map<&'static str, Command> = static_map! {
    Default: Command::Radical,
    "frac"   => Command::GenFraction(None, None, BarThickness::Default, MathStyle::NoChange),
    "tfrac"  => Command::GenFraction(None, None, BarThickness::Default, MathStyle::Text),
    "dfrac"  => Command::GenFraction(None, None, BarThickness::Default, MathStyle::Display),
    "binom"  => Command::GenFraction(sym!(b'(', open), sym!(b')', close), BarThickness::None, MathStyle::NoChange),
    "tbinom" => Command::GenFraction(sym!(b'(', open), sym!(b')', close), BarThickness::None, MathStyle::Text),
    "dbinom" => Command::GenFraction(sym!(b'(', open), sym!(b')', close), BarThickness::None, MathStyle::Display),

    "substack" => Command::Stack(AtomType::Inner),

    "sqrt" => Command::Radical,

    "bigl"  => Command::DelimiterSize(1, AtomType::Open),
    "Bigl"  => Command::DelimiterSize(2, AtomType::Open),
    "biggl" => Command::DelimiterSize(3, AtomType::Open),
    "Biggl" => Command::DelimiterSize(4, AtomType::Open),
    "bigr"  => Command::DelimiterSize(1, AtomType::Close),
    "Bigr"  => Command::DelimiterSize(2, AtomType::Close),
    "biggr" => Command::DelimiterSize(3, AtomType::Close),
    "Biggr" => Command::DelimiterSize(4, AtomType::Close),
    "bigm"  => Command::DelimiterSize(1, AtomType::Relation),
    "Bigm"  => Command::DelimiterSize(2, AtomType::Relation),
    "biggm" => Command::DelimiterSize(3, AtomType::Relation),
    "Biggm" => Command::DelimiterSize(4, AtomType::Relation),
    "big"   => Command::DelimiterSize(1, AtomType::Ordinal),
    "Big"   => Command::DelimiterSize(2, AtomType::Ordinal),
    "bigg"  => Command::DelimiterSize(3, AtomType::Ordinal),
    "Bigg"  => Command::DelimiterSize(4, AtomType::Ordinal),

    "!"     => Command::Kerning(Unit::Em(-3f64/18f64)),
    ","     => Command::Kerning(Unit::Em(3f64/18f64)),
    ":"     => Command::Kerning(Unit::Em(4f64/18f64)),
    " "     => Command::Kerning(Unit::Em(1f64/4f64)),
    ";"     => Command::Kerning(Unit::Em(5f64/18f64)),
    "quad"  => Command::Kerning(Unit::Em(1.0f64)),
    "qquad" => Command::Kerning(Unit::Em(2.0f64)),
    "rule"  => Command::Rule,

    "vextend" => Command::VExtend,
    "hextend" => Command::HExtend,

    "textstyle"         => Command::Style(Style::Text),
    "displaystyle"      => Command::Style(Style::Display),
    "scriptstyle"       => Command::Style(Style::Script),
    "scriptscriptstyle" => Command::Style(Style::ScriptScript),

    "mathop"  => Command::AtomChange(AtomType::Operator(false)),
    "mathrel" => Command::AtomChange(AtomType::Relation),
    "mathord" => Command::AtomChange(AtomType::Alpha),

    "color"   => Command::Color,
    "blue"    => Command::ColorLit(RGBA(0,0,0xff,0xff)),
    "red"     => Command::ColorLit(RGBA(0xff,0,0,0xff)),
    "gray"    => Command::ColorLit(RGBA(0x80,0x80,0x80,0xff)),
    "phantom" => Command::ColorLit(RGBA(0,0,0,0)),

    "det"     => Command::TextOperator("det", true),
    "gcd"     => Command::TextOperator("gcd", true),
    "lim"     => Command::TextOperator("lim", true),
    "limsup"  => Command::TextOperator("lim,sup", true),
    "liminf"  => Command::TextOperator("lim,inf", true),
    "sup"     => Command::TextOperator("sup", true),
    "supp"    => Command::TextOperator("supp", true),
    "inf"     => Command::TextOperator("inf", true),
    "max"     => Command::TextOperator("max", true),
    "min"     => Command::TextOperator("min", true),
    "Pr"      => Command::TextOperator("Pr", true),

    "sin"     => Command::TextOperator("sin", false),
    "cos"     => Command::TextOperator("cos", false),
    "tan"     => Command::TextOperator("tan", false),
    "cot"     => Command::TextOperator("cot", false),
    "csc"     => Command::TextOperator("csc", false),
    "sec"     => Command::TextOperator("sec", false),
    "arcsin"  => Command::TextOperator("arcsin", false),
    "arccos"  => Command::TextOperator("arccos", false),
    "arctan"  => Command::TextOperator("arctan", false),
    "sinh"    => Command::TextOperator("sinh", false),
    "cosh"    => Command::TextOperator("cosh", false),
    "tanh"    => Command::TextOperator("tanh", false),
    "arg"     => Command::TextOperator("arg", false),
    "deg"     => Command::TextOperator("deg", false),
    "dim"     => Command::TextOperator("dim", false),
    "exp"     => Command::TextOperator("exp", false),
    "hom"     => Command::TextOperator("hom", false),
    "Hom"     => Command::TextOperator("Hom", false),
    "ker"     => Command::TextOperator("ker", false),
    "Ker"     => Command::TextOperator("Ker", false),
    "ln"      => Command::TextOperator("ln", false),
    "log"     => Command::TextOperator("log", false),
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MathStyle {
    Display,
    Text,
    NoChange,
}

macro_rules! required {
    ($lex:ident, $f:expr) => (
        if $lex.current == Token::Symbol('{') {
            $lex.next();
            let result = $f;
            $lex.consume_whitespace();
            $lex.current.expect(Token::Symbol('}'))?;
            $lex.next();
            result
        } else {
            panic!("Missing required argument!".to_string())
        }
    )
}

impl Command {
    pub fn parse(self, lex: &mut Lexer, local: FontStyle) -> Result<Option<ParseNode>> {
        Ok(match self {
            Command::Radical =>
                Some(ParseNode::Radical(Radical {
                    inner: parse::required_argument(lex, local)?,
                })),

            Command::GenFraction(ld, rd, bt, ms) =>
                Some(ParseNode::GenFraction(GenFraction{
                    left_delimiter:  ld,
                    right_delimiter: rd,
                    bar_thickness:   bt,
                    numerator: parse::required_argument(lex, local)?,
                    denominator: parse::required_argument(lex, local)?,
                })),

            Command::DelimiterSize(cs, at) =>
                Some(ParseNode::Symbol(parse::expect_type(lex, local, at)?)),
            Command::Rule => {
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

            Command::Kerning(k) =>
                Some(ParseNode::Kerning(k)),

            Command::VExtend => {
                let sym = required!(lex, {
                    let c = lex.current;
                    lex.next();
                    c
                });

                let code = if let Token::Symbol(ch) = sym {
                    ch as u32
                } else {
                    panic!("Not a symbol!!");
                };

                let height = required!(lex, lex.dimension())?
                    .expect("Unable to parse dimension for Extend.");

                Some(ParseNode::Extend(code, height))
            },

            Command::HExtend =>
                None,

            Command::Style(sty) =>
                Some(ParseNode::Style(sty)),

            Command::AtomChange(sty) => {
                Some(ParseNode::AtomChange(AtomChange {
                    at: sty,
                    inner: parse::required_argument(lex, local)?
                }))
            },

            Command::Color => {
                let rgba = {
                    let color = lex.group()?;
                    parse::color::COLOR_MAP
                        .get(color)
                        .ok_or_else(|| Error::UnrecognizedColor(color.into()))?
                };

                Some(ParseNode::Color(Color {
                    color: *rgba,
                    inner: parse::required_argument(lex, local)?
                }))
            },

            Command::ColorLit(rgba) => {
                Some(ParseNode::Color(Color {
                    color: rgba,
                    inner: parse::required_argument(lex, local)?
                }))
            },

            Command::TextOperator(op, limits) => {
                Some(ParseNode::AtomChange(AtomChange {
                    at: AtomType::Operator(limits),
                    inner: text(op),
                }))
            },

            Command::Stack(atom) => {
                if lex.current != Token::Symbol('{') {
                    return Err(Error::StackMustFollowGroup)
                }

                lex.next();
                let mut result: Vec<Vec<ParseNode>> = Vec::new();
                // Continue parsing expressions, until we reach '}'
                loop {
                    let expr = parse::expression(lex, local)?;
                    result.push(expr);

                    if lex.current == Token::Symbol('}') { break }
                    if lex.current != Token::Command(r"\") {
                        return Err(Error::StackMustFollowGroup)
                    } else {
                        lex.next();
                    }
                }

                lex.next();
                Some(ParseNode::Stack(Stack {
                    atom_type: atom,
                    lines: result,
                }))
            }
        }) // End Match
    }
}