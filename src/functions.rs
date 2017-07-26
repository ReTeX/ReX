use dimensions::Unit;
use font::{Weight, Family, Symbol, AtomType};
use font::Style;
use font::style::style_symbol;
use layout::Style as LayoutStyle;
use lexer::{Lexer, Token};
use parser as parse;
use parser::nodes::{ParseNode, Radical, GenFraction, Rule, BarThickness, AtomChange, Color, Stack};
use parser::color::RGBA;
use static_map;
use error::{Error, Result};

macro_rules! text {
    ($code:expr) => ({

        ParseNode::Symbol(Symbol {
            unicode: style_symbol(
                $code as u32,
                Style::default()
                    .with_family(Family::Roman)
                    .with_weight(Weight::None)),
            atom_type: AtomType::Ordinal,
        })
    })
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MathStyle {
    Display,
    Text,
    NoChange,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    Radical,
    Rule,
    VExtend,
    Color,
    ColorLit(RGBA),
    Fraction(Option<Symbol>, Option<Symbol>, BarThickness, MathStyle),
    DelimiterSize(u8, AtomType),
    Kerning(Unit),
    Style(LayoutStyle),
    AtomChange(AtomType),
    TextOperator(&'static str, bool),
    Stack(AtomType),
}

impl Command {
    pub fn parse(self, lex: &mut Lexer, local: Style) -> Result<ParseNode> {
        use self::Command::*;
        match self {
            Radical              => radical(lex, local),
            Rule                 => rule(lex, local),
            VExtend              => v_extend(lex, local),
            Color                => color(lex, local),
            ColorLit(a)          => color_lit(lex, local, a),
            Fraction(a, b, c, d) => fraction(lex, local, a, b, c, d),
            DelimiterSize(a, b)  => delimiter_size(lex, local, a, b),
            Kerning(a)           => kerning(lex, local, a),
            Style(a)             => style(lex, local, a),
            AtomChange(a)        => atom_change(lex, local, a),
            TextOperator(a, b)   => text_operator(lex, local, a, b),
            Stack(a)             => stack(lex, local, a),
        }
    }
}

fn radical(lex: &mut Lexer, local: Style) -> Result<ParseNode> {
    let inner = parse::required_argument(lex, local)?;
    Ok(ParseNode::Radical(Radical { inner }))
}

fn rule(lex: &mut Lexer, local: Style) -> Result<ParseNode> {
    lex.consume_whitespace();
    let width = lex.dimension()?.expect("Unable to parse dimension for Rule.");
    lex.consume_whitespace();
    let height = lex.dimension()?.expect("Unable to parse dimension for Rule.");
    Ok(ParseNode::Rule(Rule { width, height }))
}

fn v_extend(lex: &mut Lexer, local: Style) -> Result<ParseNode> {
    let arg = parse::required_argument_with(lex, local, parse::symbol)?;
    let sym = match arg {
        Some(ParseNode::Symbol(sym)) => sym,

        // TODO: add better error
        _ => return Err(Error::ExpectedOpenGroup),
    } ;

    let height = parse::required_argument_with(lex, local, parse::dimension)?;
    Ok(ParseNode::Extend(sym.unicode, height))
}

fn color(lex: &mut Lexer, local: Style) -> Result<ParseNode> {
    let color = parse::required_argument_with(lex, local, parse::color)?;
    let inner = parse::required_argument(lex, local)?;
    Ok(ParseNode::Color(Color { color, inner }))
}

fn color_lit(lex: &mut Lexer, local: Style, color: RGBA) -> Result<ParseNode> {
    let inner = parse::required_argument(lex, local)?;
    Ok(ParseNode::Color(Color { color, inner }))
}

fn fraction(lex: &mut Lexer, local: Style, left_delimiter: Option<Symbol>, right_delimiter: Option<Symbol>, bar_thickness: BarThickness, _: MathStyle) -> Result<ParseNode> {
    let numerator = parse::required_argument(lex, local)?;
    let denominator = parse::required_argument(lex, local)?;

    Ok(ParseNode::GenFraction(GenFraction{
        left_delimiter,
        right_delimiter,
        bar_thickness,
        numerator,
        denominator
    }))
}

fn delimiter_size(lex: &mut Lexer, local: Style, _: u8, atom_type: AtomType) -> Result<ParseNode> {
    let symbol = parse::expect_type(lex, local, atom_type)?;
    Ok(ParseNode::Symbol(symbol))
}

fn kerning(lex: &mut Lexer, local: Style, unit: Unit) -> Result<ParseNode> {
    Ok(ParseNode::Kerning(unit))
}

fn style(lex: &mut Lexer, local: Style, new_style: LayoutStyle) -> Result<ParseNode> {
    Ok(ParseNode::Style(new_style))
}

fn atom_change(lex: &mut Lexer, local: Style, at: AtomType) -> Result<ParseNode> {
    let inner = parse::required_argument(lex, local)?;
    Ok(ParseNode::AtomChange(AtomChange { at, inner }))
}

fn text_operator(lex: &mut Lexer, local: Style, text: &str, limits: bool) -> Result<ParseNode> {
    const SMALL_SKIP: Unit = Unit::Em(3f64/18f64);
    let at = AtomType::Operator(limits);
    let mut inner = Vec::with_capacity(text.len());

    for c in text.chars() {
        if c == ',' {
            inner.push(ParseNode::Kerning(SMALL_SKIP));
        } else {
            inner.push(text!(c));
        }
    }

    Ok(ParseNode::AtomChange(AtomChange { at, inner }))
}

fn stack(lex: &mut Lexer, local: Style, atom_type: AtomType) -> Result<ParseNode> {
    if lex.current != Token::Symbol('{') {
        return Err(Error::StackMustFollowGroup)
    }

    lex.next();
    let mut lines: Vec<Vec<ParseNode>> = Vec::new();

    // Continue parsing expressions, until we reach '}'
    loop {
        lines.push(parse::expression(lex, local)?);
        match lex.current {
            Token::Symbol('}')   => break,
            Token::Command(r"\") => lex.next(),
            _ => return  Err(Error::UnexpectedEof),
        };
    }

    lex.next();
    Ok(ParseNode::Stack(Stack { atom_type, lines }))
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

pub static COMMANDS: static_map::Map<&'static str, Command> = static_map! {
    Default: Command::Radical,

    // Fraction commands
    "frac"   => Command::Fraction(None, None, BarThickness::Default, MathStyle::NoChange),
    "tfrac"  => Command::Fraction(None, None, BarThickness::Default, MathStyle::Text),
    "dfrac"  => Command::Fraction(None, None, BarThickness::Default, MathStyle::Display),
    "binom"  => Command::Fraction(sym!(b'(', open), sym!(b')', close), BarThickness::None, MathStyle::NoChange),
    "tbinom" => Command::Fraction(sym!(b'(', open), sym!(b')', close), BarThickness::None, MathStyle::Text),
    "dbinom" => Command::Fraction(sym!(b'(', open), sym!(b')', close), BarThickness::None, MathStyle::Display),

    // Stacking commands
    "substack" => Command::Stack(AtomType::Inner),

    // Radical commands
    "sqrt" => Command::Radical,

    // Delimiter size commands
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

    // Spacing related commands
    "!"     => Command::Kerning(Unit::Em(-3f64/18f64)),
    ","     => Command::Kerning(Unit::Em(3f64/18f64)),
    ":"     => Command::Kerning(Unit::Em(4f64/18f64)),
    " "     => Command::Kerning(Unit::Em(1f64/4f64)),
    ";"     => Command::Kerning(Unit::Em(5f64/18f64)),
    "quad"  => Command::Kerning(Unit::Em(1.0f64)),
    "qquad" => Command::Kerning(Unit::Em(2.0f64)),
    "rule"  => Command::Rule,

    // Useful other than debugging?
    "vextend" => Command::VExtend,

    // Display style changes
    "textstyle"         => Command::Style(LayoutStyle::Text),
    "displaystyle"      => Command::Style(LayoutStyle::Display),
    "scriptstyle"       => Command::Style(LayoutStyle::Script),
    "scriptscriptstyle" => Command::Style(LayoutStyle::ScriptScript),

    // Atom-type changes
    "mathop"  => Command::AtomChange(AtomType::Operator(false)),
    "mathrel" => Command::AtomChange(AtomType::Relation),
    "mathord" => Command::AtomChange(AtomType::Alpha),

    // Color related
    "color"   => Command::Color,
    "blue"    => Command::ColorLit(RGBA(0,0,0xff,0xff)),
    "red"     => Command::ColorLit(RGBA(0xff,0,0,0xff)),
    "gray"    => Command::ColorLit(RGBA(0x80,0x80,0x80,0xff)),
    "phantom" => Command::ColorLit(RGBA(0,0,0,0)),

    // Operators with limits
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

    // Operators without limits
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