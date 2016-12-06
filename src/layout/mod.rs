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

#[macro_use]
mod builders;
mod convert;
pub mod engine;
pub mod spacing;

use dimensions::{ Pixels, Unit };
use font::constants;
use std::ops::Deref;
use std::fmt;

// By default this will act as a horizontal box
#[derive(Clone, Debug)]
pub struct Layout {
    pub contents:  Vec<LayoutNode>,
    pub width:     Pixels,
    pub height:    Pixels,
    pub depth:     Pixels,
    pub offset:    Pixels,
    pub alignment: Alignment,
}

impl Layout {
    pub fn as_node(self) -> LayoutNode {
        LayoutNode {
            width:  self.width,
            height: self.height,
            depth:  self.depth,
            node: LayoutVariant::HorizontalBox(HorizontalBox {
                contents:  self.contents,
                offset:    self.offset,
                alignment: self.alignment,
            })
        }
    }

    pub fn new() -> Layout {
        Layout {
            contents:  vec![],
            width:     Pixels(0.0),
            height:    Pixels(0.0),
            depth:     Pixels(0.0),
            offset:    Pixels(0.0),
            alignment: Alignment::default(),
        }
    }

    pub fn add_node(&mut self, node: LayoutNode) {
        self.width += node.width;
        self.height = self.height.max(node.height);
        self.depth  = self.depth.min(node.depth);
        self.contents.push(node);
    }

    pub fn set_offset(&mut self, offset: Pixels) {
        self.offset = offset;
    }

    pub fn finalize(mut self) -> Layout {
        self.depth  -= self.offset;
        self.height -= self.offset;
        self
    }
}

#[derive(Clone)]
pub struct LayoutNode {
    pub node:   LayoutVariant,
    pub width:  Pixels,
    pub height: Pixels,
    pub depth:  Pixels,
}

#[derive(Clone)]
pub enum LayoutVariant {
    HorizontalBox (HorizontalBox),
    VerticalBox   (VerticalBox),
    Glyph         (LayoutGlyph),
    Rule,
    Kern,
}

#[derive(Clone, Default)]
pub struct HorizontalBox {
    pub contents:  Vec<LayoutNode>,
    pub offset:    Pixels,
    pub alignment: Alignment,
}

#[derive(Clone, Default)]
pub struct VerticalBox {
    pub contents:  Vec<LayoutNode>,
    pub offset:    Pixels,
    pub alignment: Alignment,
}

#[derive(Clone)]
pub struct LayoutGlyph {
    pub unicode:    u32,
    pub scale:      f64,
    pub offset:     Pixels,
    pub attachment: Pixels,
    pub italics:    Pixels,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Alignment {
    Centered(Pixels),
    Right(Pixels),
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
        write!(f, "LayoutGlyph(0x{:X})", self.unicode)
    }
}

impl fmt::Debug for LayoutNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.node {
            LayoutVariant::HorizontalBox(ref hb) =>
                write!(f, "HBox({:?})", hb.contents),
            LayoutVariant::VerticalBox(ref vb) =>
                write!(f, "VBox({:?})", vb.contents),
            LayoutVariant::Glyph(ref gly) =>
                write!(f, "Glyph({:?})", gly),
            LayoutVariant::Rule =>
                write!(f, "Rule()"),
            LayoutVariant::Kern => {
                let kern = if self.width == Pixels(0.0) {
                    self.height
                } else { self.width };

                write!(f, "Kern({:.1})", kern)
            }
        }
    }
}

impl LayoutNode {
    /// Center the vertical about the axis.
    /// For now this ignores offsets if already applied,
    /// and will break if there already are offsets.
    fn centered(mut self, axis: Pixels) -> LayoutNode {
        let shift = 0.5 * (self.height + self.depth) - axis;

        match self.node {
            LayoutVariant::VerticalBox(ref mut vb) => {
                vb.offset    = shift;
                self.height -= shift;
                self.depth  -= shift;
            },

            LayoutVariant::Glyph(_) =>
                return vbox!(offset: shift; self),

            _ => (),
        }

        self
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
    fn default() -> Style { Style::Display }
}

#[allow(dead_code)]
impl Style {
    fn cramped(self) -> Style {
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

    fn is_cramped(&self) -> bool {
        match *self {
            Style::Display |
            Style::Text |
            Style::Script |
            Style::ScriptScript
                => false,
            _   => true,
        }
    }

    fn numerator(self) -> Style {
        match self {
            Style::Display        => Style::Text,
            Style::DisplayCramped => Style::TextCramped,
            _ => self.superscript_variant(),
        }
    }

    fn denominator(self) -> Style {
        match self {
            Style::Display |
            Style::DisplayCramped => Style::TextCramped,
            _ => self.subscript_variant(),
        }
    }
}