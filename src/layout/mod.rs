//! This module is where we convert ParseNodes to Layout boxes which are ready to be rendered.
//! The layout boxes follow a similar model as those found in HTML and TeX in that they both
//! have horizontal and vertical boxes.  One difference will be how glue is handled.  HTML/CSS
//! does not have anything similar to how glue is handled in TeX and so aboslute size will be
//! necessary for these scnarios.  It's unclear if we will be able to induce alignments from
//! glue, such as something being centered, aligned left/right, etc.  These questions may
//! also be useful to answer in SVG.
//!
//! Layout boxes will contain a minimal representation of what will be rendered.
//! This includes the container types: Horizontal/Vertical boxes,
//! and primitive types: Symbols, lines, spacing.
//!
//! While rendering in mathmode, most types require an atomtype to determine the kerning
//! between symbols.  This information must also be present with layout boxes.
//!
//! The units used in layout boxes must be in FontUnit (as defined in CSS).

#[macro_use]
mod builders;
mod convert;
pub mod engine;
pub mod spacing;

use parser::color::RGBA;
use font::constants;
use font::FontUnit;
use std::ops::Deref;
use std::fmt;
use std::cmp::{max, min};

// By default this will act as a horizontal box
#[derive(Clone, Debug, Default)]
pub struct Layout {
    pub contents: Vec<LayoutNode>,
    pub width: FontUnit,
    pub height: FontUnit,
    pub depth: FontUnit,
    pub offset: FontUnit,
    pub alignment: Alignment,
}

impl Layout {
    pub fn as_node(self) -> LayoutNode {
        LayoutNode {
            width: self.width,
            height: self.height,
            depth: self.depth,
            node: LayoutVariant::HorizontalBox(HorizontalBox {
                                                   contents: self.contents,
                                                   offset: self.offset,
                                                   alignment: self.alignment,
                                               }),
        }
    }

    pub fn new() -> Layout {
        Layout::default()
    }

    pub fn add_node(&mut self, node: LayoutNode) {
        self.width += node.width;
        self.height = max(self.height, node.height);
        self.depth = min(self.depth, node.depth);
        self.contents.push(node);
    }

    pub fn set_offset(&mut self, offset: FontUnit) {
        self.offset = offset;
    }

    pub fn finalize(mut self) -> Layout {
        self.depth -= self.offset;
        self.height -= self.offset;
        self
    }

    pub fn centered(mut self, new_width: FontUnit) -> Layout {
        self.alignment = Alignment::Centered(self.width);
        self.width = new_width;
        self
    }

    fn is_symbol(&self) -> Option<LayoutGlyph> {
        if self.contents.len() != 1 {
            return None;
        }
        self.contents[0].is_symbol()
    }
}

#[derive(Clone)]
pub struct LayoutNode {
    pub node: LayoutVariant,
    pub width: FontUnit,
    pub height: FontUnit,
    pub depth: FontUnit,
}

#[derive(Clone)]
pub enum LayoutVariant {
    HorizontalBox(HorizontalBox),
    VerticalBox(VerticalBox),
    Glyph(LayoutGlyph),
    Color(ColorChange),
    Rule,
    Kern,
}

#[derive(Clone)]
pub struct ColorChange {
    pub color: RGBA,
    pub inner: Vec<LayoutNode>,
}

#[derive(Clone, Default)]
pub struct HorizontalBox {
    pub contents: Vec<LayoutNode>,
    pub offset: FontUnit,
    pub alignment: Alignment,
}

#[derive(Clone, Default)]
pub struct VerticalBox {
    pub contents: Vec<LayoutNode>,
    pub offset: FontUnit,
    pub alignment: Alignment,
}

#[derive(Clone, Copy)]
pub struct LayoutGlyph {
    pub unicode: u32,
    pub scale: FontUnit,
    pub offset: FontUnit,
    pub attachment: FontUnit,
    pub italics: FontUnit,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Alignment {
    Centered(FontUnit),
    Right(FontUnit),
    Left,
    Inherit,
    Default,
}

impl Default for Alignment {
    fn default() -> Alignment {
        Alignment::Default
    }
}

impl Deref for HorizontalBox {
    type Target = [LayoutNode];
    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

impl Deref for VerticalBox {
    type Target = [LayoutNode];
    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

impl fmt::Debug for VerticalBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.offset == FontUnit::from(0) {
            write!(f, "VerticalBox({:?})", self.contents)
        } else {
            write!(f,
                   "VerticalBox({:?}, offset: {})",
                   self.contents,
                   self.offset)
        }
    }
}

impl fmt::Debug for HorizontalBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HorizontalBox({:?})", self.contents)
    }
}

impl fmt::Debug for LayoutGlyph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LayoutGlyph(0x{:X})", self.unicode)
    }
}

impl fmt::Debug for LayoutNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.node {
            LayoutVariant::HorizontalBox(ref hb) => write!(f, "HBox({:?})", hb.contents),
            LayoutVariant::VerticalBox(ref vb) => write!(f, "VBox({:?})", vb.contents),
            LayoutVariant::Glyph(ref gly) => write!(f, "Glyph({:?})", gly),
            LayoutVariant::Rule => write!(f, "Rule()"),
            LayoutVariant::Kern => {
                let kern = if self.width == FontUnit::from(0) {
                    self.height
                } else {
                    self.width
                };

                write!(f, "Kern({:.1})", kern)
            }
            LayoutVariant::Color(ref clr) => write!(f, "Color({:?}, {:?})", clr.color, clr.inner),
        }
    }
}

impl LayoutNode {
    /// Center the vertical about the axis.
    /// For now this ignores offsets if already applied,
    /// and will break if there already are offsets.
    fn centered(mut self, axis: FontUnit) -> LayoutNode {
        let shift = (self.height + self.depth) / 2 - axis;

        match self.node {
            LayoutVariant::VerticalBox(ref mut vb) => {
                vb.offset = shift;
                self.height -= shift;
                self.depth -= shift;
            }

            LayoutVariant::Glyph(_) => return vbox!(offset: shift; self),

            _ => (),
        }

        self
    }

    fn is_symbol(&self) -> Option<LayoutGlyph> {
        match self.node {
            LayoutVariant::Glyph(gly) => Some(gly),
            LayoutVariant::HorizontalBox(ref hb) => is_symbol(&hb.contents),
            LayoutVariant::VerticalBox(ref vb) => is_symbol(&vb.contents),
            LayoutVariant::Color(ref clr) => is_symbol(&clr.inner),
            _ => None,
        }
    }
}

pub fn is_symbol(contents: &[LayoutNode]) -> Option<LayoutGlyph> {
    if contents.len() != 1 {
        return None;
    }

    contents[0].is_symbol()
}

/// Display styles which are used in scaling glyphs.  The associated
/// methods are taken from pg.441 from the TeXBook
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Style {
    ScriptScriptCramped,
    ScriptScript,
    ScriptCramped,
    Script,
    TextCramped,
    Text,
    DisplayCramped,
    Display,
}

impl Default for Style {
    fn default() -> Style {
        Style::Display
    }
}

#[allow(dead_code)]
impl Style {
    fn cramped(self) -> Style {
        match self {
            Style::ScriptScriptCramped |
            Style::ScriptScript => Style::ScriptScriptCramped,
            Style::ScriptCramped | Style::Script => Style::ScriptCramped,
            Style::TextCramped | Style::Text => Style::TextCramped,
            Style::DisplayCramped | Style::Display => Style::DisplayCramped,
        }
    }

    fn superscript_variant(self) -> Style {
        match self {
            Style::Display | Style::Text => Style::Script,
            Style::DisplayCramped | Style::TextCramped => Style::ScriptCramped,
            Style::Script | Style::ScriptScript => Style::ScriptScript,
            Style::ScriptCramped |
            Style::ScriptScriptCramped => Style::ScriptScriptCramped,
        }
    }

    fn subscript_variant(self) -> Style {
        match self {
            Style::Display | Style::Text | Style::DisplayCramped | Style::TextCramped => {
                Style::ScriptCramped
            }
            Style::Script |
            Style::ScriptScript |
            Style::ScriptCramped |
            Style::ScriptScriptCramped => Style::ScriptScriptCramped,
        }
    }

    fn font_scale(self) -> FontUnit {
        match self {
            Style::Display | Style::DisplayCramped | Style::Text | Style::TextCramped => {
                FontUnit::from(1)
            }
            Style::Script | Style::ScriptCramped => {
                FontUnit::from(constants::SCRIPT_PERCENT_SCALE_DOWN)
            }
            Style::ScriptScript |
            Style::ScriptScriptCramped => {
                FontUnit::from(constants::SCRIPT_SCRIPT_PERCENT_SCALE_DOWN)
            }
        }
    }

    fn sup_shift_up(self) -> FontUnit {
        match self {
            Style::Display | Style::Text | Style::Script | Style::ScriptScript => {
                constants::SUPERSCRIPT_SHIFT_UP
            }
            _ => constants::SUPERSCRIPT_SHIFT_UP_CRAMPED,
        }
    }

    fn is_cramped(&self) -> bool {
        match *self {
            Style::Display | Style::Text | Style::Script | Style::ScriptScript => false,
            _ => true,
        }
    }

    fn numerator(self) -> Style {
        match self {
            Style::Display => Style::Text,
            Style::DisplayCramped => Style::TextCramped,
            _ => self.superscript_variant(),
        }
    }

    fn denominator(self) -> Style {
        match self {
            Style::Display | Style::DisplayCramped => Style::TextCramped,
            _ => self.subscript_variant(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct LayoutSettings {
    pub font_size: u16,
    pub style: Style,
}

impl LayoutSettings {
    fn cramped(self) -> LayoutSettings {
        LayoutSettings {
            style: self.style.cramped(),
            ..self
        }
    }

    fn superscript_variant(self) -> LayoutSettings {
        LayoutSettings {
            style: self.style.superscript_variant(),
            ..self
        }
    }

    fn subscript_variant(self) -> LayoutSettings {
        LayoutSettings {
            style: self.style.subscript_variant(),
            ..self
        }
    }

    fn numerator(self) -> LayoutSettings {
        LayoutSettings {
            style: self.style.numerator(),
            ..self
        }
    }

    fn denominator(self) -> LayoutSettings {
        LayoutSettings {
            style: self.style.denominator(),
            ..self
        }
    }

    fn with_display(self) -> LayoutSettings {
        LayoutSettings {
            style: Style::Display,
            ..self
        }
    }

    fn with_text(self) -> LayoutSettings {
        LayoutSettings {
            style: Style::Text,
            ..self
        }
    }
}