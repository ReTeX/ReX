#![allow(dead_code)]
use dimensions::Unit;
use layout::Style;

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
    Style       (Style),
    AtomChange  (AtomChange),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AtomChange {
    pub at: AtomType,
    pub inner: Vec<ParseNode>,
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

impl ParseNode {
    pub fn expect_left(self) -> Result<Symbol, String> {
        if let ParseNode::Symbol(sym) = self {
            if sym.atom_type == AtomType::Open
                || sym.atom_type == AtomType::Fence
                || sym.unicode == 46 {
                    return Ok(sym)
            } else {
                return Err(
                    format!(r#"Expected Open, Fence, or period after `\left`, found {:?}"#, sym))
            }
        } else {
            unreachable!()
        }
    }

    pub fn expect_right(self) -> Result<Symbol, String> {
        if let ParseNode::Symbol(sym) = self {
            if sym.atom_type == AtomType::Close
                || sym.atom_type == AtomType::Fence
                || sym.unicode == 46 {
                    return Ok(sym)
            } else {
                return Err(
                    format!(r#"Expected Open, Fence, or period after `\right`, found {:?}"#, sym))
            }
        } else {
            unreachable!()
        }
    }
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
            ParseNode::Style(_)        => None,
            ParseNode::AtomChange(AtomChange { at, .. }) => Some(at),
        }
    }
}

impl ParseNode {
    pub fn set_atom_type(&mut self, at: AtomType) {
        match *self {
            ParseNode::Symbol(ref mut sym) => sym.atom_type = at,
            ParseNode::Scripts(Scripts { ref mut base, ..}) =>
                if let Some(ref mut b) = *base {
                    b.set_atom_type(at);
                },
            ParseNode::AtomChange(ref mut node) =>
                node.at = at,
            _ => (),
        }
    }

    pub fn is_symbol(&self) -> Option<Symbol> {
        match *self {
            ParseNode::Symbol(sym) => Some(sym),
            ParseNode::Accent(ref acc) => acc.nucleus.is_symbol(),
            ParseNode::AtomChange(AtomChange { ref inner, .. }) => {
                if inner.len() != 1 { return None }
                inner[0].is_symbol()
            },
            ParseNode::Scripts(Scripts { ref base, ..}) => {
                if let Some(ref b) = *base {
                    b.is_symbol()
                } else { None }
            },
            _ => None,
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