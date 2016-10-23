#![allow(dead_code)]
/// Intermediate Nodes
///
/// Mathlist    - A list of atoms
///
/// Nodes in our parse tree:
///
/// Rendering Requirements for Parsing Nodes
///
/// - Symbols
///      + Font Family & Code Point
///      + Font Dimensions [ width, height, depth ]
///      + Lig/Kern -- todo
///
/// - Operators
///      + Symbol (minus atom type)
///      + limits [true/false]
///      + Extensible?
///      + Successor?    // 
///      + largeop [true/false]?
///
/// - Delimiters
///     + Symbol (minus atom type)
///     + Extensible?
///     + Successor?
///
/// - Accent
///     + Accent Character
///     + Inner Contents:
///        - Symbol or MathList
///     + Kerning if Symbol
///
/// - Radical
///     + Inner Contents: Mathlist
///     + Superscript: Mathlist
///
/// - Scripts Environment
///     + Type: [ Atom type inherited?, if so expose? ]
///     + Subscript: Mathlist
///     + Superscript: Mathlist
///     + Base: Mathlist
///  
/// - Generalized Fraction
///     + Numerator: Mathlist
///     + Denominator: Mathlist
///     + Barwidth: TexDimension
///     + Left Delimiter: Delimiter
///     + Right Delimiter: Delimiter
///
/// - Style Changes
///     + Enum: Text/Script/ScriptScript/Display..
///
/// - Size Change
///     + Enum: Small, Large, LARGE, etc..
///
/// TODO: Environment ???

// use std::boxed::Box;

// pub type MathList = Vec<ParseNode>;
// pub type BoxedMathList = Box<MathList>;

// There are additional classes defined from unicode-math 
// in addition to those defined by TeX.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtomType {
    Punctuation,
    Ordinal,
    Open,
    Close,
    Binary,
    Relation,
    Accent,
    AccentWide,
    BotAccent,
    BotAccentWide,
    Alpha,
    Fence,
    Operator,
    Over,
    Under,
}

use symbols::Symbol;

#[derive(Debug, PartialEq, Eq)]
pub struct Delimited {
    pub left:  Symbol,
    pub right: Symbol,
    pub inner: Vec<ParseNode>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseNode {
    Symbol(Symbol),
    Delimited(Delimited),
    Group(Vec<ParseNode>),
    Radical(Radical),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Radical {
    pub inner: Box<ParseNode>,
    // We will handle optional arguments at a later day
    // pub superscript: Vec<ParseNode>,
}

pub struct RadicalBuilder { }

use lexer::Lexer;

pub trait TexCommand {
    fn parse_command(&mut self, &mut Lexer) -> Result<ParseNode, String>;
}

// <math field>
use parser;
impl TexCommand for RadicalBuilder {
    fn parse_command(&mut self, lex: &mut Lexer) -> Result<ParseNode, String> {
        // Parse expression
        Ok(ParseNode::Radical(Radical {
            inner: Box::new(try!(parser::math_field(lex))),
        }))
    }
}

// /// Every symbol will need a font family
// /// and a code point for how the symbol will be rendered.
// /// This font family and code point will be used to look up
// /// additional information regarding the dimensions if needed.

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct SymbolIdentifier {
//     pub font_family: u8,
//     pub code_point: u32,
// }

// #[derive(Debug)]
// pub struct Symbol {
//     pub font_family: u8,
//     pub code_point: u32, 
//     pub height: u32,
//     pub width: u32,
//     pub depth: u32,
//     pub atom_type: AtomType,
// }

// #[derive(Debug)]
// pub struct Operator {
//     pub symbol: Symbol,
//     pub limits: bool,
//     pub largeop: bool,
//     pub successor: Option<SymbolIdentifier>,
// }

// #[derive(Debug)]
// pub struct Delimiter {
//     pub symbol: Symbol,
//     pub successor: Option<SymbolIdentifier>,
// }

// // The following are wrapper types,
// // which mean they can wrap either a symbol
// // or mathlist.
// #[derive(Debug)]
// pub enum SymOrMathList {
//     Symbol(Symbol),
//     MathList(Box<MathList>),
// }

// #[derive(Debug)]
// pub struct Accent {
//     pub inner: SymOrMathList,
//     pub accent: Symbol,
//     pub kern: u32,
// }

// #[derive(Debug)]
// pub struct Scripts {
//     pub subscript: SymOrMathList,
//     pub supscript: SymOrMathList,
//     pub base:  SymOrMathList,
//     pub atom_type: AtomType,
// }

// #[derive(Debug)]
// pub struct GenFraction {
//     pub numerator: Box<MathList>,
//     pub denominator: Box<MathList>,
//     pub barwidth: u32,
//     pub leftdelim: Delimiter,
//     pub rightdelim: Delimiter,
// }

// #[derive(Debug)]
// pub enum ParseNode {
//     Symbol(Symbol),
//     Operator(Operator),
//     Delimiter(Delimiter),
//     Accent(Accent),
//     Radical(Radical),
//     Scripts(Scripts),
//     GenFraction(GenFraction),
// }