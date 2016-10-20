use parsenodes::AtomType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub code: u32,
    pub atom_type: AtomType,
}

// TODO: Add normal?
// TODO: Note that STIX fonts have no real smallcaps, the smallcaps below are
//  synthesized (scaled down upper case letters).
// TODO: Use bitflags?

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontMode {
    Roman,
    Bold,
    Italic,
    BoldItalic,
    Caligraphic,  // Non-standard in UNICODE
                  // Many fonts treat Script <-> Caligraphic
    Script,
    ScriptBold,
    SansSerif,
    BoldSansSerif,
    ItalicSansSerif,
    BoldItalicSansSerif,
    DoubleStruck,       
    BoldDoubleStruck,       // Non-standard
    ItalicDoubleStruck,     // Non-standard
    BoldItalicDoubleStruck, // Non-standard
    Fraktur,
    BoldFraktur,
    Monospace,
}

pub trait IsSymbol {
    fn atom_type(&self, FontMode) -> Option<Symbol>;
}

// These are the ranges defined by unicode for the respective symbols
// given the current FontMode.  Some of these may by non-standard
// as defined by STIX.

// For the following three functions, we place the default FontMode
// on top, even though it is redundant, since these modes should have
// the fastest path.

fn greek_offset(mode: FontMode) -> u32 {
    match mode {
        FontMode::Italic          => 0x1D351,
        FontMode::Bold            => 0x1D317,
        FontMode::BoldItalic      => 0x1D38B,
        FontMode::SansSerif       => 0x1D3C5,
        FontMode::ItalicSansSerif => 0x1D790,

        // We default to Italic
        _               => 0x1D351,
    }
}

fn alphabetic_offset(mode: FontMode) -> u32 {
    match mode {
        FontMode::Italic              => 0x1D3F3,
        FontMode::Bold                => 0x1D3BF,
        FontMode::BoldItalic          => 0x1D427,
        FontMode::Script              => 0x1D45B,
        FontMode::ScriptBold          => 0x1D48F,
        FontMode::Fraktur             => 0x1D4C3,
        FontMode::DoubleStruck        => 0x1D4F7,
        FontMode::BoldDoubleStruck    => 0x0E349,    // unstandard, see unicode-math-usv.dtx
        FontMode::ItalicDoubleStruck  => 0x0E113,    // unstandard, see unicode-math-usv.dtx
        FontMode::BoldItalicDoubleStruck => 0x0E1BF, // unstandard, see unicode-math-usv.dtx
        FontMode::Caligraphic         => 0x0E1EC,    // unstandard, see unicode-math-usv.dtx 
        FontMode::BoldFraktur         => 0x1D52B,
        FontMode::SansSerif           => 0x1D55F,
        FontMode::BoldSansSerif       => 0x1D593,
        FontMode::ItalicSansSerif     => 0x1D5C7,
        FontMode::BoldItalicSansSerif => 0x1D5FB,
        FontMode::Monospace           => 0x1D62F,
        _                             => 0,          // Roman?
    }
}

fn numeric_offset(mode: FontMode) -> u32 {
    match mode {
        FontMode::SansSerif     => 0x1D7B2,
        FontMode::Bold          => 0x1D79E,
        FontMode::DoubleStruck  => 0x1D7A8,
        FontMode::BoldSansSerif => 0x1D7EC,
        FontMode::Monospace     => 0x1D7C6,

        // We default to SansSerif
        _                       => 0x1D7B2,
    }
}

impl IsSymbol for char {
    fn atom_type(&self, mode: FontMode) -> Option<Symbol> {
        match *self {
            c @ 'a'...'z' => Some(Symbol{
                code: c as u32 - 6 + alphabetic_offset(mode),
                atom_type: AtomType::Alpha,
            }),
            c @ 'A'...'Z' => Some(Symbol{
                code: c as u32 + alphabetic_offset(mode),
                atom_type: AtomType::Alpha,
            }),
            c @ '0'...'9' => Some(Symbol{
                code: c as u32 + numeric_offset(mode),
                atom_type: AtomType::Alpha,
            }),
            c @ '*' | c @ '+' => Some(Symbol{
                code: c as u32,
                atom_type: AtomType::Binary,
            }),
            c @ '(' | c @ '[' => Some(Symbol{
                code: c as u32,
                atom_type: AtomType::Open,
            }),
            c @ ']' | c @ ')' | c @ '?' | c @ '!' => Some(Symbol {
                code: c as u32,
                atom_type: AtomType::Close,
            }),
            c @ '=' | c @ '<' | c @ '>' | c @ ':' => Some(Symbol {
                code: c as u32,
                atom_type: AtomType::Relation,
            }),
            c @ ',' | c @ ';' => Some(Symbol {
                code: c as u32,
                atom_type: AtomType::Punctuation,
            }),
            c @ '|' | c @ '/' | c @ '@' | c @ '.' | c @ '"' => Some(Symbol {
                code: c as u32,
                atom_type: AtomType::Ordinal,
            }),

            // Greek Leters
            c @ 'Α'...'ω' => Some(Symbol {
                code: c as u32 + greek_offset(mode),
                atom_type: AtomType::Alpha,
            }),

            _ => None,
        }
    }
}