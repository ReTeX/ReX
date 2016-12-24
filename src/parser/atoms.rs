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
}

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
            ParseNode::Color(Color { ref inner, ..}) => {
                if inner.len() != 1 { return None }
                inner[0].atom_type()
            }
        }
    }
}