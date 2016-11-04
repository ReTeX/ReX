// Do not modify.  Automatically generated.
use parser::nodes::AtomType;
use font::{Style, Symbol};

pub trait IsAtom {
    fn atom_type(&self, Style) -> Option<Symbol>;
}

impl IsAtom for char {
    fn atom_type(&self, mode: Style) -> Option<Symbol> {
        match *self {
            c @ 'a'...'z' => Some(Symbol {
                id: (c as i32 + 0) as u16,
                atom_type: AtomType::Alpha,
            }),
            c @ 'A'...'Z' => Some(Symbol {
                id: (c as i32 + 0) as u16,
                atom_type: AtomType::Alpha,
            }),
            c @ '0'...'9' => Some(Symbol {
                id: (c as i32 + 0) as u16,
                atom_type: AtomType::Alpha,
            }),
            c @ 'Α'...'Ρ' => Some(Symbol {
                id: (c as i32 + -254) as u16,
                atom_type: AtomType::Alpha,
            }),
            c @ 'Σ'...'Ω' => Some(Symbol {
                id: (c as i32 + -255) as u16,
                atom_type: AtomType::Alpha,
            }),
            c @ 'α'...'ρ' => Some(Symbol {
                id: (c as i32 + -255) as u16,
                atom_type: AtomType::Alpha,
            }),
            c @ 'σ'...'ω' => Some(Symbol {
                id: (c as i32 + -255) as u16,
                atom_type: AtomType::Alpha,
            }),
            c @ '*' => Some(Symbol {
                id: 42 as u16,
                atom_type: AtomType::Binary,
            }),
            c @ '+' => Some(Symbol {
                id: 43 as u16,
                atom_type: AtomType::Binary,
            }),
            c @ '(' => Some(Symbol {
                id: 40 as u16,
                atom_type: AtomType::Open,
            }),
            c @ '[' => Some(Symbol {
                id: 91 as u16,
                atom_type: AtomType::Open,
            }),
            c @ ']' => Some(Symbol {
                id: 93 as u16,
                atom_type: AtomType::Close,
            }),
            c @ ')' => Some(Symbol {
                id: 41 as u16,
                atom_type: AtomType::Close,
            }),
            c @ '?' => Some(Symbol {
                id: 63 as u16,
                atom_type: AtomType::Close,
            }),
            c @ '!' => Some(Symbol {
                id: 33 as u16,
                atom_type: AtomType::Close,
            }),
            c @ '=' => Some(Symbol {
                id: 61 as u16,
                atom_type: AtomType::Relation,
            }),
            c @ '<' => Some(Symbol {
                id: 60 as u16,
                atom_type: AtomType::Relation,
            }),
            c @ '>' => Some(Symbol {
                id: 62 as u16,
                atom_type: AtomType::Relation,
            }),
            c @ ':' => Some(Symbol {
                id: 58 as u16,
                atom_type: AtomType::Relation,
            }),
            c @ ',' => Some(Symbol {
                id: 44 as u16,
                atom_type: AtomType::Punctuation,
            }),
            c @ ';' => Some(Symbol {
                id: 59 as u16,
                atom_type: AtomType::Punctuation,
            }),
            c @ '|' => Some(Symbol {
                id: 124 as u16,
                atom_type: AtomType::Ordinal,
            }),
            c @ '/' => Some(Symbol {
                id: 47 as u16,
                atom_type: AtomType::Ordinal,
            }),
            c @ '@' => Some(Symbol {
                id: 64 as u16,
                atom_type: AtomType::Ordinal,
            }),
            c @ '.' => Some(Symbol {
                id: 46 as u16,
                atom_type: AtomType::Ordinal,
            }),
            c @ '"' => Some(Symbol {
                id: 34 as u16,
                atom_type: AtomType::Ordinal,
            }),
            _ => None,
        }
    }
}