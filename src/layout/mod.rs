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
//! The units used in layout boxes must be in pixels (as defined in CSS).

pub mod boundingbox;
pub mod reduce;
//pub mod builders;

use dimensions::Unit;
use dimensions::FontUnit;
use dimensions::Unital;
use font::constants;
pub use self::boundingbox::BoundingBox;
use ::std::ops::Deref;

use dimensions::Pixels;

#[derive(Clone)]
pub enum LayoutNode {
    HorizontalBox (HorizontalBox),
    VerticalBox   (VerticalBox),
    Glyph         (LayoutGlyph),
    Rule          (Rule),
    Kern          (Pixels),
}

#[derive(Clone)]
pub struct LayoutGlyph {
    pub scale:   f64,
    pub height:  Pixels,
    pub depth:   Pixels,
    pub advance: Pixels,
    pub unicode: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct Rule {
    pub width:  Pixels,
    pub height: Pixels,
    pub depth:  Pixels,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Alignment {
    Centered,
    Right,
    Left,
    Inherit,
    Default,
}

impl Default for Alignment {
    fn default() -> Alignment {
        Alignment::Default
    }
}

#[derive(Clone, Default)]
pub struct HorizontalBox {
    pub contents: Vec<LayoutNode>,
    pub alignment: Alignment,
}

impl Deref for HorizontalBox {
    type Target = [LayoutNode];
    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

#[derive(Clone, Default)]
pub struct VerticalBox {
    pub contents:  Vec<LayoutNode>,
    pub alignment: Alignment,
    pub offset:    Pixels,
}

impl Deref for VerticalBox {
    type Target = [LayoutNode];
    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

use std::fmt;
impl fmt::Debug for VerticalBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.offset == Pixels(0.0) {
            write!(f, "VerticalBox({:?})", self.contents)
        } else {
            write!(f, "VerticalBox({:?}, offset: {:.1})", self.contents, *self.offset)
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
        write!(f, "LayoutGlyph(0x{:X}, height: {:.1}, depth: {:.1})", self.unicode, *self.height, *self.depth)
    }
}

impl fmt::Debug for LayoutNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LayoutNode::HorizontalBox(ref hb) =>
                write!(f, "{:?}", hb),
            LayoutNode::VerticalBox(ref vb) =>
                write!(f, "{:?}", vb),
            LayoutNode::Glyph(ref gly) =>
                write!(f, "{:?}", gly),
            LayoutNode::Rule(r) =>
                write!(f, "{:?}", r),
            LayoutNode::Kern(p) =>
                write!(f, "Kern({:.1})", p.0),
        }
    }
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
    fn cramped_variant(self) -> Style {
        match self {
            Style::ScriptScriptCramped |
            Style::ScriptScript
                => Style::ScriptScriptCramped,
            Style::ScriptCramped |
            Style::Script
                => Style::ScriptCramped,
            Style::TextCramped |
            Style::Text
                => Style::TextCramped,
            Style::DisplayCramped |
            Style::Display
                => Style::DisplayCramped,
        }
    }

    fn superscript_variant(self) -> Style {
        match self {
            Style::Display |
            Style::Text
                => Style::Script,
            Style::DisplayCramped |
            Style::TextCramped
                => Style::ScriptCramped,
            Style::Script |
            Style::ScriptScript
                => Style::ScriptScript,
            Style::ScriptCramped |
            Style::ScriptScriptCramped
                => Style::ScriptScriptCramped,
        }
    }

    fn subscript_variant(self) -> Style {
        match self {
            Style::Display |
            Style::Text |
            Style::DisplayCramped |
            Style::TextCramped
                => Style::ScriptCramped,
            Style::Script |
            Style::ScriptScript |
            Style::ScriptCramped |
            Style::ScriptScriptCramped
                => Style::ScriptScriptCramped,
        }
    }

    fn font_scale(self) -> f64 {
        use font::constants;
        match self {
            Style::Display |
            Style::DisplayCramped |
            Style::Text |
            Style::TextCramped
                => 1f64,
            Style::Script |
            Style::ScriptCramped
                => f64::from(constants::SCRIPT_PERCENT_SCALE_DOWN) / 100f64,
            Style::ScriptScript |
            Style::ScriptScriptCramped
                => f64::from(constants::SCRIPT_SCRIPT_PERCENT_SCALE_DOWN) / 100f64,
        }
    }

    fn sup_shift_up(self) -> Unit {
        match self {
            Style::Display |
            Style::Text |
            Style::Script |
            Style::ScriptScript
                => constants::SUPERSCRIPT_SHIFT_UP.into(),
            _   => constants::SUPERSCRIPT_SHIFT_UP_CRAMPED.into(),
        }
    }

    fn cramped(&self) -> bool {
        match *self {
            Style::Display |
            Style::Text |
            Style::Script |
            Style::ScriptScript
                => false,
            _   => true,
        }
    }
}

trait ToPixels: Sized {
    fn as_pixels(self, font_size: f64) -> Pixels;
    fn scaled_pixels(self, font_size: f64, sty: Style) -> Pixels {
        self.as_pixels(font_size) * sty.font_scale()
    }
}

impl ToPixels for Unit {
    // TODO: You can't assign pt values to fonts with given `font_size: f64`
    fn as_pixels(self, font_size: f64) -> Pixels {
        Pixels(match self {
            Unit::Font(u) => u / f64::from(constants::UNITS_PER_EM) * font_size,
            Unit::Em(u)   => u * font_size,
            Unit::Ex(u)   => u * font_size, // TODO: measure x width here
            Unit::Px(u)   => u
        })
    }
}

impl<U: Unital> ToPixels for FontUnit<U> {
    fn as_pixels(self, font_size: f64) -> Pixels {
        Unit::from(self).as_pixels(font_size)
    }
}