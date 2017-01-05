// Do not modify.  Automatically generated.
use parser::AtomType;

pub trait OptionalAtom {
    fn atom_type(&self) -> Option<AtomType>;
}

impl OptionalAtom for char {
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