use super::ParseNode;
use font::AtomType;

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
            ParseNode::Extend(_,_)   => AtomType::Inner,
            ParseNode::Stack(ref s)  => s.atom_type,
        }
    }
}