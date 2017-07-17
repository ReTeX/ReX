use std::default::Default;

mod glyphs;
mod symbols;
mod offsets;
mod kerning_table;
pub mod kerning;
pub mod constants;
pub mod variants;
pub mod variant_tables;
pub mod fontselection;

pub use self::glyphs::GLYPHS;
pub use self::symbols::SYMBOLS;
pub use self::offsets::OptionalAtom;

use parser::AtomType;
use dimensions::FontUnit;

pub fn glyph_metrics(code: u32) -> Glyph {
    *GLYPHS
         .get(&code)
         .expect(&format!("Unable to find glyph for code {}", code))
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BBox(pub FontUnit, pub FontUnit, pub FontUnit, pub FontUnit);

#[derive(Debug, Clone, Copy)]
pub struct Glyph {
    pub unicode: u32,
    pub bbox: BBox,
    pub advance: FontUnit,
    pub lsb: FontUnit,
    pub italics: FontUnit,
    pub attachment: FontUnit,
}

impl Glyph {
    pub fn height(&self) -> FontUnit {
        self.bbox.3
    }
    pub fn depth(&self) -> FontUnit {
        self.bbox.1
    }
    pub fn advance(&self) -> FontUnit {
        self.advance
    }
    #[allow(dead_code)]
    pub fn lsb(&self) -> FontUnit {
        self.lsb
    }
    pub fn italic_correction(&self) -> FontUnit {
        self.italics
    }
    pub fn attachment_offset(&self) -> FontUnit {
        self.attachment
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub unicode: u32,
    pub atom_type: AtomType,
}

impl Symbol {
    pub fn with_style(self, style: Style) -> Symbol {
        Symbol {
            unicode: style.style_symbol(self.unicode),
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub struct KernRecord {
    top_right: Option<KernTable>,
    top_left: Option<KernTable>,
    bottom_right: Option<KernTable>,
    bottom_left: Option<KernTable>,
}

#[derive(Debug, Clone)]
pub struct KernTable {
    correction_heights: &'static [FontUnit], // unit::Font::<i16>()
    kern_values: &'static [FontUnit],        // unit::Font::<i16>()
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Style {
    family: Family,
    weight: Weight,
}

impl Style {
    pub fn new() -> Style {
        Style::default()
    }

    pub fn with_family(self, fam: Family) -> Style {
        Style {
            family: fam,
            ..self
        }
    }

    pub fn with_weight(self, weight: Weight) -> Style {
        Style {
            weight: weight,
            ..self
        }
    }

    pub fn with_bold(self) -> Style {
        Style {
            weight: self.weight.with_bold(),
            ..self
        }
    }

    pub fn with_italics(self) -> Style {
        Style {
            weight: self.weight.with_italics(),
            ..self
        }
    }
}

// NB: Changing the order of these variants requires
//     changing the LUP in fontselection
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Family {
    Roman,
    Script,
    Fraktur,
    SansSerif,
    Blackboard,
    Monospace,
    Normal,
}

// NB: Changing the order of these variants requires
//     changing the LUP in fontselection
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Weight {
    None,
    Italic,
    Bold,
    BoldItalic,
}

impl Weight {
    fn with_bold(self) -> Self {
        match self {
            Weight::Italic | Weight::BoldItalic => Weight::BoldItalic,
            _ => Weight::Bold,
        }
    }

    fn with_italics(self) -> Self {
        match self {
            Weight::Bold | Weight::BoldItalic => Weight::BoldItalic,
            _ => Weight::Italic,
        }
    }
}

impl Default for Family {
    fn default() -> Family {
        Family::Normal
    }
}

impl Default for Weight {
    fn default() -> Weight {
        Weight::None
    }
}