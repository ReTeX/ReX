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
pub use self::offsets::IsAtom;

use parser::nodes::AtomType;

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

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
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