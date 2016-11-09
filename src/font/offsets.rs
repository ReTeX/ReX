// Do not modify.  Automatically generated.
use parser::nodes::AtomType;
use font::{Style, Symbol};

use super::glyphs::range;
use super::glyphs::offset;

pub trait IsAtom {
    fn atom_type(&self, Style) -> Option<Symbol>;
}

// This macro implements IsAtom for char
//   [Encoding Range], [Atom Type], [Offset]

atom_types! {
    LATIN_LOWER => Alpha,
    LATIN_UPPER => Alpha,
    DIGITS      => Alpha,
    GREEK_UPPER => Alpha,
    @single '*' =>
    @single '+'
    @single '('
    @single '
}

impl IsAtom for char {
    fn atom_type(&self, mode: Style) -> Option<Symbol> {
        match *self as u32 {
            range::LATIN_LOWER =>
            c @ 'a'...'z' => Some(Symbol {
                id: (c as i32 + 164) as u16,
                atom_type: AtomType::Alpha,
            }),
            c @ 'A'...'Z' => Some(Symbol {
                id: (c as i32 + 164) as u16,
                atom_type: AtomType::Alpha,
            }),
            c @ '0'...'9' => Some(Symbol {
                id: (c as i32 + 66) as u16,
                atom_type: AtomType::Alpha,
            }),
            c @ 'Α'...'Ρ' => Some(Symbol {
                id: (c as i32 + -764) as u16,
                atom_type: AtomType::Alpha,
            }),
            c @ 'Σ'...'Ω' => Some(Symbol {
                id: (c as i32 + -764) as u16,
                atom_type: AtomType::Alpha,
            }),
            c @ 'α'...'ρ' => Some(Symbol {
                id: (c as i32 + -115) as u16,
                atom_type: AtomType::Alpha,
            }),
            c @ 'σ'...'ω' => Some(Symbol {
                id: (c as i32 + -115) as u16,
                atom_type: AtomType::Alpha,
            }),
            c @ '*' => Some(Symbol {
                id: 216 as u16,
                atom_type: AtomType::Binary,
            }),
            c @ '+' => Some(Symbol {
                id: 217 as u16,
                atom_type: AtomType::Binary,
            }),
            c @ '(' => Some(Symbol {
                id: 214 as u16,
                atom_type: AtomType::Open,
            }),
            c @ '[' => Some(Symbol {
                id: 255 as u16,
                atom_type: AtomType::Open,
            }),
            c @ ']' => Some(Symbol {
                id: 257 as u16,
                atom_type: AtomType::Close,
            }),
            c @ ')' => Some(Symbol {
                id: 215 as u16,
                atom_type: AtomType::Close,
            }),
            c @ '?' => Some(Symbol {
                id: 227 as u16,
                atom_type: AtomType::Close,
            }),
            c @ '!' => Some(Symbol {
                id: 207 as u16,
                atom_type: AtomType::Close,
            }),
            c @ '=' => Some(Symbol {
                id: 225 as u16,
                atom_type: AtomType::Relation,
            }),
            c @ '<' => Some(Symbol {
                id: 224 as u16,
                atom_type: AtomType::Relation,
            }),
            c @ '>' => Some(Symbol {
                id: 226 as u16,
                atom_type: AtomType::Relation,
            }),
            c @ ':' => Some(Symbol {
                id: 222 as u16,
                atom_type: AtomType::Relation,
            }),
            c @ ',' => Some(Symbol {
                id: 218 as u16,
                atom_type: AtomType::Punctuation,
            }),
            c @ ';' => Some(Symbol {
                id: 223 as u16,
                atom_type: AtomType::Punctuation,
            }),
            c @ '|' => Some(Symbol {
                id: 288 as u16,
                atom_type: AtomType::Ordinal,
            }),
            c @ '/' => Some(Symbol {
                id: 221 as u16,
                atom_type: AtomType::Ordinal,
            }),
            c @ '@' => Some(Symbol {
                id: 228 as u16,
                atom_type: AtomType::Ordinal,
            }),
            c @ '.' => Some(Symbol {
                id: 220 as u16,
                atom_type: AtomType::Ordinal,
            }),
            c @ '"' => Some(Symbol {
                id: 208 as u16,
                atom_type: AtomType::Ordinal,
            }),
            _ => None,
        }
    }
}