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

pub fn glyph_metrics(code : u32) -> Glyph {
    GLYPHS.get(&code)
        .expect(&format!("Unable to find glyph for code {}", code))
        .clone()
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BBox(pub i16, pub i16, pub i16, pub i16);

#[derive(Debug, Clone, Copy)]
pub struct Glyph {
    pub unicode: u32,
    pub bbox: BBox,
    pub advance: u16,
    pub lsb: i16,
    pub italics: i16,    // design units
    pub attachment: i16, // design units
}

use dimensions::Unit;
impl Glyph {
    pub fn height(&self) -> Unit { Unit::Font(self.bbox.3 as f64) }
    pub fn depth(&self) -> Unit { Unit::Font(self.bbox.1 as f64) }
    pub fn advance(&self) -> Unit { Unit::Font(self.advance as f64) }
    #[allow(dead_code)]
    pub fn lsb(&self) -> Unit { Unit::Font(self.lsb as f64) }
    #[allow(dead_code)]
    pub fn italic_correction(&self) -> Unit { Unit::Font(self.italics as f64) }
    #[allow(dead_code)]
    pub fn attachment_offset(&self) -> Unit { Unit::Font(self.attachment as f64) }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub unicode: u32,
    pub atom_type: AtomType,
}

#[derive(Debug, Clone)]
pub struct KernRecord {
    top_right:     Option<KernTable>,
    top_left:      Option<KernTable>,
    bottom_right:  Option<KernTable>,
    bottom_left:   Option<KernTable>,
}

#[derive(Debug, Clone)]
pub struct KernTable {
    correction_heights: Vec<i16>,   // unit::Font::<i16>()
    kern_values:        Vec<i16>,   // unit::Font::<i16>()
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

    pub fn with_family(&self, fam: Family) -> Style {
        Style {
            family: fam,
            ..*self
        }
    }

    pub fn with_weight(&self, weight: Weight) -> Style {
        Style {
            weight: weight,
            ..*self
        }
    }

    pub fn from_cmd(cmd: &str, old: &Style) -> Option<Style> {
        if let Some(weight) = match cmd {
            "mathbf"   => {
                match old.weight {
                    Weight::Bold | Weight::None => Some(Weight::Bold),
                    _ => Some(Weight::BoldItalic),
                }
            },
            "mathit"   => {
                match old.weight {
                    Weight::Italic | Weight::None => Some(Weight::Italic),
                    _ => Some(Weight::BoldItalic),
                }
            },
            _ => None,
        } {
            Some(old.with_weight(weight))
        }

        else if let Some(family) = match cmd {
            "mathrm"   => Some(Family::Roman),
            "mathscr"  => Some(Family::Script),
            "mathfrak" => Some(Family::Fraktur),
            "mathbb"   => Some(Family::Blackboard),
            "mathsf"   => Some(Family::SansSerif),
            "mathtt"   => Some(Family::Monospace),
            "mathcal"  => Some(Family::Script),
            _ => None,
        } {
            Some(old.with_family(family))
        }

        else {
            None
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

impl Default for Family {
    fn default() -> Family {
        Family::Roman
    }
}

impl Default for Weight {
    fn default() -> Weight {
        Weight::Italic
    }
}