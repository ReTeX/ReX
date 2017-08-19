/// Font types used internally by ReX.  There is little concern for maintaining
/// backwards compatible changes and may regularly have major version changes.
/// Use with cautions.
#[macro_use]
extern crate serde_derive;

pub mod fp;

pub type FixedPoint = fp::F24P8;
pub type FontUnit = FixedPoint;

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
    Transparent,
}

#[derive(Debug, Clone)]
pub enum VariantGlyph {
    Replacement(Glyph),
    Constructable(Direction, Vec<GlyphInstruction>),
}

#[derive(Debug, Clone)]
pub struct GlyphVariants {
    pub replacements: &'static [ReplacementGlyph],
    pub constructable: Option<ConstructableGlyph>,
}

#[derive(Debug, Copy, Clone)]
pub struct ReplacementGlyph {
    pub unicode: u32,
    pub advance: FontUnit,
}

#[derive(Debug, Copy, Clone)]
pub struct ConstructableGlyph {
    pub parts: &'static [GlyphPart],
    pub italics_correction: FontUnit,
}

#[derive(Debug, Copy, Clone)]
pub struct GlyphPart {
    pub unicode: u32,
    pub start_connector_length: FontUnit,
    pub end_connector_length: FontUnit,
    pub full_advance: FontUnit,
    pub required: bool,
}

#[derive(Clone, Debug, Copy)]
pub struct GlyphInstruction {
    pub glyph: Glyph,
    pub overlap: FontUnit,
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Vertical,
    Horizontal,
}

use std::default::Default;

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
    #[inline]
    pub fn height(&self) -> FontUnit {
        self.bbox.3
    }

    #[inline]
    pub fn depth(&self) -> FontUnit {
        self.bbox.1
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub unicode: u32,
    pub atom_type: AtomType,
}

#[derive(Debug, Clone)]
pub struct KernRecord {
    pub top_right: Option<KernTable>,
    pub top_left: Option<KernTable>,
    pub bottom_right: Option<KernTable>,
    pub bottom_left: Option<KernTable>,
}

#[derive(Debug, Clone)]
pub struct KernTable {
    pub correction_heights: &'static [FontUnit], // unit::Font::<i16>()
    pub kern_values: &'static [FontUnit],        // unit::Font::<i16>()
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Style {
    pub family: Family,
    pub weight: Weight,
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