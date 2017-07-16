use staticmap;
use font::Symbol;
use parser::nodes::{ ParseNode, Radical, GenFraction, Rule, BarThickness, AtomChange, Color, Stack };
use parser::AtomType;
use lexer::Lexer;
use lexer::Token;
use parser as parse;
use dimensions::Unit;
use layout::Style;
use font::{Weight, Family};
use font::Style as FontStyle;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TexCommand {
    Radical,
    Rule,
    VExtend,
    HExtend,
    Color,
    ColorLit(&'static str),
    GenFraction {
        left:  Option<Symbol>,
        right: Option<Symbol>,
        bar:   BarThickness,
        style: MathStyle,
    },
    DelimiterSize {
        atom_type: AtomType,
        size:      u8,
    },
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
        Symbol {
            unicode: $code as u32,
            atom_type: sym!(@at $ord),
        }
    });
}

macro_rules! text {
    ($code:expr) => ({
        ParseNode::Symbol(Symbol {
            unicode: ::font::Style::default()
                .with_family(Family::Roman)
                .with_weight(Weight::None)
                .style_symbol($code as u32),
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

pub static COMMANDS: staticmap::Map<&'static str, TexCommand> = static_map! {
    Default: TexCommand::Radical,
    "frac"   => TexCommand::GenFraction { left: None, right: None, bar: BarThickness::Default, style: MathStyle::NoChange },
    "tfrac"  => TexCommand::GenFraction { left: None, right: None, bar: BarThickness::Default, style: MathStyle::Text },
    "dfrac"  => TexCommand::GenFraction { left: None, right: None, bar: BarThickness::Default, style: MathStyle::Display },
    "binom"  => TexCommand::GenFraction { left: Some(sym!(b'(', open)), right: Some(sym!(b')', close)), bar: BarThickness::None, style: MathStyle::NoChange },
    "tbinom" => TexCommand::GenFraction { left: Some(sym!(b'(', open)), right: Some(sym!(b')', close)), bar: BarThickness::None, style: MathStyle::Text },
    "dbinom" => TexCommand::GenFraction { left: Some(sym!(b'(', open)), right: Some(sym!(b')', close)), bar: BarThickness::None, style: MathStyle::Display },

    "substack" => TexCommand::Stack(AtomType::Inner),

    "sqrt" => TexCommand::Radical,

    "bigl"  => TexCommand::DelimiterSize { size: 1, atom_type: AtomType::Open },
    "Bigl"  => TexCommand::DelimiterSize { size: 2, atom_type: AtomType::Open },
    "biggl" => TexCommand::DelimiterSize { size: 3, atom_type: AtomType::Open },
    "Biggl" => TexCommand::DelimiterSize { size: 4, atom_type: AtomType::Open },
    "bigr"  => TexCommand::DelimiterSize { size: 1, atom_type: AtomType::Close },
    "Bigr"  => TexCommand::DelimiterSize { size: 2, atom_type: AtomType::Close },
    "biggr" => TexCommand::DelimiterSize { size: 3, atom_type: AtomType::Close },
    "Biggr" => TexCommand::DelimiterSize { size: 4, atom_type: AtomType::Close },
    "bigm"  => TexCommand::DelimiterSize { size: 1, atom_type: AtomType::Relation },
    "Bigm"  => TexCommand::DelimiterSize { size: 2, atom_type: AtomType::Relation },
    "biggm" => TexCommand::DelimiterSize { size: 3, atom_type: AtomType::Relation },
    "Biggm" => TexCommand::DelimiterSize { size: 4, atom_type: AtomType::Relation },
    "big"   => TexCommand::DelimiterSize { size: 1, atom_type: AtomType::Ordinal },
    "Big"   => TexCommand::DelimiterSize { size: 2, atom_type: AtomType::Ordinal },
    "bigg"  => TexCommand::DelimiterSize { size: 3, atom_type: AtomType::Ordinal },
    "Bigg"  => TexCommand::DelimiterSize { size: 4, atom_type: AtomType::Ordinal },

    "!"     => TexCommand::Kerning(Unit::Em(-3f64/18f64)),
    ","     => TexCommand::Kerning(Unit::Em(3f64/18f64)),
    ":"     => TexCommand::Kerning(Unit::Em(4f64/18f64)),
    " "     => TexCommand::Kerning(Unit::Em(1f64/4f64)),
    ";"     => TexCommand::Kerning(Unit::Em(5f64/18f64)),
    "quad"  => TexCommand::Kerning(Unit::Em(1.0f64)),
    "qquad" => TexCommand::Kerning(Unit::Em(2.0f64)),
    "rule"  => TexCommand::Rule,

    "vextend" => TexCommand::VExtend,
    "hextend" => TexCommand::HExtend,

    "textstyle"         => TexCommand::Style(Style::Text),
    "displaystyle"      => TexCommand::Style(Style::Display),
    "scriptstyle"       => TexCommand::Style(Style::Script),
    "scriptscriptstyle" => TexCommand::Style(Style::ScriptScript),

    "mathop"  => TexCommand::AtomChange(AtomType::Operator(false)),
    "mathrel" => TexCommand::AtomChange(AtomType::Relation),
    "mathord" => TexCommand::AtomChange(AtomType::Alpha),

    "color"   => TexCommand::Color,
    "blue"    => TexCommand::ColorLit("blue"),
    "red"     => TexCommand::ColorLit("red"),
    "gray"    => TexCommand::ColorLit("gray"),
    "phantom" => TexCommand::ColorLit("rgba(0,0,0,0)"),

    "det"     => TexCommand::TextOperator("det", true),
    "gcd"     => TexCommand::TextOperator("gcd", true),
    "lim"     => TexCommand::TextOperator("lim", true),
    "limsup"  => TexCommand::TextOperator("lim,sup", true),
    "liminf"  => TexCommand::TextOperator("lim,inf", true),
    "sup"     => TexCommand::TextOperator("sup", true),
    "supp"    => TexCommand::TextOperator("supp", true),
    "inf"     => TexCommand::TextOperator("inf", true),
    "max"     => TexCommand::TextOperator("max", true),
    "min"     => TexCommand::TextOperator("min", true),
    "Pr"      => TexCommand::TextOperator("Pr", true),

    "sin"     => TexCommand::TextOperator("sin", false),
    "cos"     => TexCommand::TextOperator("cos", false),
    "tan"     => TexCommand::TextOperator("tan", false),
    "cot"     => TexCommand::TextOperator("cot", false),
    "csc"     => TexCommand::TextOperator("csc", false),
    "sec"     => TexCommand::TextOperator("sec", false),
    "arcsin"  => TexCommand::TextOperator("arcsin", false),
    "arccos"  => TexCommand::TextOperator("arccos", false),
    "arctan"  => TexCommand::TextOperator("arctan", false),
    "sinh"    => TexCommand::TextOperator("sinh", false),
    "cosh"    => TexCommand::TextOperator("cosh", false),
    "tanh"    => TexCommand::TextOperator("tanh", false),
    "arg"     => TexCommand::TextOperator("arg", false),
    "deg"     => TexCommand::TextOperator("deg", false),
    "dim"     => TexCommand::TextOperator("dim", false),
    "exp"     => TexCommand::TextOperator("exp", false),
    "hom"     => TexCommand::TextOperator("hom", false),
    "Hom"     => TexCommand::TextOperator("Hom", false),
    "ker"     => TexCommand::TextOperator("ker", false),
    "Ker"     => TexCommand::TextOperator("Ker", false),
    "ln"      => TexCommand::TextOperator("ln", false),
    "log"     => TexCommand::TextOperator("log", false),
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

impl TexCommand {
    pub fn parse(self, lex: &mut Lexer, local: FontStyle) -> Result<Option<ParseNode>, String> {
        Ok(match self {
            TexCommand::Radical =>
                Some(ParseNode::Radical(Radical {
                    inner: parse::required_macro_argument(lex, local)?,
                })),

            TexCommand::GenFraction {
                left:  ld,
                right: rd,
                bar:   bt,
                style: ms,
            } =>
                Some(ParseNode::GenFraction(GenFraction{
                    left_delimiter:  ld,
                    right_delimiter: rd,
                    bar_thickness:   bt,
                    numerator: parse::required_macro_argument(lex, local)?,
                    denominator: parse::required_macro_argument(lex, local)?,
                })),

            TexCommand::DelimiterSize {
                size: s,
                atom_type: at,
            } =>
                Some(ParseNode::Symbol(parse::expect_type(lex, local, at)?)),
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

            TexCommand::VExtend => {
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

            TexCommand::HExtend =>
                None,

            TexCommand::Style(sty) =>
                Some(ParseNode::Style(sty)),

            TexCommand::AtomChange(sty) => {
                Some(ParseNode::AtomChange(AtomChange {
                    at: sty,
                    inner: parse::required_macro_argument(lex, local)?
                }))
            },

            TexCommand::Color => {
                let color = lex.group()?.to_string();

                Some(ParseNode::Color(Color {
                    color: color,
                    inner: parse::required_macro_argument(lex, local)?
                }))
            },

            TexCommand::ColorLit(clr) => {
                Some(ParseNode::Color(Color {
                    color: clr.to_string(),
                    inner: parse::required_macro_argument(lex, local)?
                }))
            },

            TexCommand::TextOperator(op, limits) => {
                Some(ParseNode::AtomChange(AtomChange {
                    at: AtomType::Operator(limits),
                    inner: text(op),
                }))
            },

            TexCommand::Stack(atom) => {
                if lex.current != Token::Symbol('{') {
                    return Err("Stack commands must follow a group.".into());
                }

                lex.next();
                let mut result: Vec<Vec<ParseNode>> = Vec::new();
                // Continue parsing expressions, until we reach '}'
                loop {
                    let expr = parse::expression(lex, local)?;
                    result.push(expr);

                    if lex.current == Token::Symbol('}') { break }
                    if lex.current != Token::Command(r"\") {
                        return Err(format!(
                            "Stack command parsing terminated pre-maturely. \
                            Perhaps there is an unbalenced bracket? \
                            Expression terminated with {}", lex.current));
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