#![allow(dead_code)]
use dimensions::Unit;

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
    Operator(bool),     // bool := limits or nolimits?
    Over,
    Under,
    Inner,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scripts {
    pub base:        Option<Box<ParseNode>>,
    pub superscript: Option<Box<ParseNode>>,
    pub subscript:   Option<Box<ParseNode>>,
}

use font::Symbol;
#[derive(Debug, PartialEq, Clone)]
pub struct Delimited {
    pub left:  Symbol,
    pub right: Symbol,
    pub inner: Vec<ParseNode>,
}

// TODO: It might be worth letting the `Group` variant
//   to have an atomtype associated with it.  By default,
//   it will be a `Ordinal`.
#[derive(Debug, PartialEq, Clone)]
pub enum ParseNode {
    Symbol      (Symbol),
    Delimited   (Delimited),
    Group       (Vec<ParseNode>),
    Radical     (Radical),
    GenFraction (GenFraction),
    Scripts     (Scripts),
    Rule        (Rule),
    Kerning     (Unit),
    Extend      (u32, Unit),
    Accent      (Accent),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Accent {
    pub symbol:  Symbol,
    pub nucleus: Box<ParseNode>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rule {
    pub width:  Unit,
    pub height: Unit,
    //pub depth:  Unit,
}

use font::IsAtom;
impl IsAtom for ParseNode {
    fn atom_type(&self) -> Option<AtomType> {
        match *self {
            ParseNode::Symbol(ref sym) => Some(sym.atom_type),
            ParseNode::Group(_)        => Some(AtomType::Alpha),
            ParseNode::Delimited(_)    => Some(AtomType::Fence),
            ParseNode::Radical(_)      => Some(AtomType::Alpha),
            ParseNode::GenFraction(_)  => Some(AtomType::Inner),
            ParseNode::Scripts(Scripts { base: ref b, .. })
                => if let Some(ref c) = *b { c.atom_type() } else { Some(AtomType::Alpha) },
            ParseNode::Accent(Accent { nucleus: ref n, .. })
                                       => n.atom_type(),
            ParseNode::Rule(_)         => None,
            ParseNode::Kerning(_)      => None,
            ParseNode::Extend(_, _)    => None,
        }
    }
}

impl ParseNode {
    pub fn set_atom_type(&mut self, at: AtomType) {
        match *self {
            ParseNode::Symbol(ref mut sym) => sym.atom_type = at,
            _ => (),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MathField {
    Symbol (Symbol),
    Group  (Vec<ParseNode>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Radical {
    pub inner: Vec<ParseNode>,
    // We will handle optional arguments at a later day
    // pub superscript: Vec<ParseNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GenFraction {
    pub numerator:       Vec<ParseNode>,
    pub denominator:     Vec<ParseNode>,
    pub bar_thickness:   BarThickness,
    pub left_delimiter:  Option<Symbol>,
    pub right_delimiter: Option<Symbol>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BarThickness {
    Default,
    None,
    Unit (Unit),
}