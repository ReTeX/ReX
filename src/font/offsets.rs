// Do not modify.  Automatically generated.
use parser::AtomType;

pub trait IsAtom {
    fn atom_type(&self) -> Option<AtomType>;
}

impl IsAtom for char {
    fn atom_type(&self) -> Option<AtomType> {
        Some(match *self {
            'a'...'z' | 'A'...'Z' |
            '0'...'9' | 'Α'...'Ω' | 'α'...'ω'  => AtomType::Alpha,

            '*' | '+' | '-' => AtomType::Binary,
            '[' | '(' => AtomType::Open,
            ']' | ')' | '?' | '!' => AtomType::Close,
            '=' | '<' | '>' | ':' => AtomType::Relation,
            ',' | ';' => AtomType::Punctuation,
            '|' |  '/' | '@' | '.' | '"' => AtomType::Alpha,
            _ => return None,
        })
    }
}