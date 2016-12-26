use super::ParseNode;
use super::nodes::{ Scripts, Accent, AtomChange, Color };
use font::IsAtom;

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
    Transparent,
}

pub trait IsAtom {
    fn atom_type(&self) -> AtomType;
}

impl IsAtom for ParseNode {
    fn atom_type(&self) -> AtomType {
        match *self {
            ParseNode::Symbol(ref sym)  => sym.atom_type,
            ParseNode::Delimited(_)     => AtomType::Inner,
            ParseNode::Radical(_)       => AtomType::Alpha,
            ParseNode::GenFraction(_)   => AtomType::Inner,
            ParseNode::Group(_)         => AtomType::Alpha,
            ParseNode::Scripts(ref scr) =>
                if let Some(ref base) = scr.base {
                    base.atom_type()
                } else {
                    AtomType::Alpha
                },

            ParseNode::Rule(_)          => AtomType::Alpha,
            ParseNode::Kerning(_)       => AtomType::Transparent,
            ParseNode::Accent(ref acc)  =>
                acc.nucleus.atom_type(),

            ParseNode::Style(_)         => AtomType::Transparent,
            ParseNode::AtomChange(ref ac) => ac.at,
            ParseNode::Color(ref clr)     =>
                if let Some(ref node) = clr.inner.first() {
                    node.atom_type()
                } else {
                    AtomType::Alpha
                },
        }
    }
}