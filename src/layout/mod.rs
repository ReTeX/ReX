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
pub mod builders;

pub use self::boundingbox::BoundingBox;

use spacing::Spacing;
use font::Glyph;
#[derive(Clone, Debug)]
pub enum LayoutNode {
    HorizontalBox(HorizontalBox),
    Glyph(Glyph),
    Space(Spacing),
    Rule(Rule),
}

#[derive(Copy, Clone, Debug)]
pub struct Rule {
    width: f64,
    height: f64,
    depth: f64,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Alignment {
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

#[derive(Clone, Debug, Default)]
pub struct HorizontalBox {
    contents: Vec<LayoutNode>,
    alignment: Alignment,
}

use ::std::ops::Deref;
impl Deref for HorizontalBox {
    type Target = [LayoutNode];
    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

#[derive(Clone, Debug, Default)]
pub struct VerticalBox {
    contents: Vec<LayoutNode>,
    alignment: Alignment,
}

impl Deref for VerticalBox {
    type Target = [LayoutNode];
    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}