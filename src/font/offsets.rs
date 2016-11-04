// Do not modify.  Automatically generated.
use parser::nodes::AtomType;

pub trait IsAtom {
    fn atom_type(&self, FontMode) -> Option<Symbol>;
}

impl IsAtom for char {
    fn atom_type(&self, mode: FontMode) -> Option<Symbol> {
        match *self {
            c @ 'a'...'z' => Some(Symbol {
                id: c + 0 as usize,
                atom_type: AtomType::Alpha,
            }),
            c @ 'A'...'Z' => Some(Symbol {
                id: c + 0 as usize,
                atom_type: AtomType::Alpha,
            }),
            c @ '0'...'9' => Some(Symbol {
                id: c + 0 as usize,
                atom_type: AtomType::Alpha,
            }),
            c @ 'Α'...'Ρ' => Some(Symbol {
                id: c + -254 as usize,
                atom_type: AtomType::Alpha,
            }),
            c @ 'Σ'...'Ω' => Some(Symbol {
                id: c + -255 as usize,
                atom_type: AtomType::Alpha,
            }),
            c @ 'α'...'ρ' => Some(Symbol {
                id: c + -255 as usize,
                atom_type: AtomType::Alpha,
            }),
            c @ 'σ'...'ω' => Some(Symbol {
                id: c + -255 as usize,
                atom_type: AtomType::Alpha,
            }),
            c @ '*' => Some(Symbol {
                id: 42,
                atom_type: AtomType::Binary,
            }),
            c @ '+' => Some(Symbol {
                id: 43,
                atom_type: AtomType::Binary,
            }),
            c @ '(' => Some(Symbol {
                id: 40,
                atom_type: AtomType::Open,
            }),
            c @ '[' => Some(Symbol {
                id: 91,
                atom_type: AtomType::Open,
            }),
            c @ ']' => Some(Symbol {
                id: 93,
                atom_type: AtomType::Close,
            }),
            c @ ')' => Some(Symbol {
                id: 41,
                atom_type: AtomType::Close,
            }),
            c @ '?' => Some(Symbol {
                id: 63,
                atom_type: AtomType::Close,
            }),
            c @ '!' => Some(Symbol {
                id: 33,
                atom_type: AtomType::Close,
            }),
            c @ '=' => Some(Symbol {
                id: 61,
                atom_type: AtomType::Relation,
            }),
            c @ '<' => Some(Symbol {
                id: 60,
                atom_type: AtomType::Relation,
            }),
            c @ '>' => Some(Symbol {
                id: 62,
                atom_type: AtomType::Relation,
            }),
            c @ ':' => Some(Symbol {
                id: 58,
                atom_type: AtomType::Relation,
            }),
            c @ ',' => Some(Symbol {
                id: 44,
                atom_type: AtomType::Punctuation,
            }),
            c @ ';' => Some(Symbol {
                id: 59,
                atom_type: AtomType::Punctuation,
            }),
            c @ '|' => Some(Symbol {
                id: 124,
                atom_type: AtomType::Ordinal,
            }),
            c @ '/' => Some(Symbol {
                id: 47,
                atom_type: AtomType::Ordinal,
            }),
            c @ '@' => Some(Symbol {
                id: 64,
                atom_type: AtomType::Ordinal,
            }),
            c @ '.' => Some(Symbol {
                id: 46,
                atom_type: AtomType::Ordinal,
            }),
            c @ '"' => Some(Symbol {
                id: 34,
                atom_type: AtomType::Ordinal,
            }),
            _ => None,
        }
    }
}