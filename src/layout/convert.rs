//! This is a collection of tools used for converting ParseNodes into LayoutNodes.
use std::convert::From;
use std::ops::Mul;

use font;
use font::constants;
use font::{Direction, Glyph, VariantGlyph, FontUnit};
use dimensions::Unit;
use layout::LayoutSettings;

use super::Style;
use super::builders;
use super::{LayoutNode, LayoutVariant, LayoutGlyph};
use parser::nodes::Rule;

pub trait AsLayoutNode {
    fn as_layout(&self, config: LayoutSettings) -> LayoutNode;
}

impl AsLayoutNode for Glyph {
    fn as_layout(&self, config: LayoutSettings) -> LayoutNode {
        LayoutNode {
            height: self.height().scaled(config),
            width:  self.advance.scaled(config),
            depth:  self.depth().scaled(config),
            node:   LayoutVariant::Glyph(LayoutGlyph {
                unicode: self.unicode,
                scale: scale(1, config),
                attachment: self.attachment.scaled(config),
                italics: self.italics.scaled(config),
                offset:  FontUnit::from(0),
            })
        }
    }
}

impl AsLayoutNode for Rule {
    fn as_layout(&self, config: LayoutSettings) -> LayoutNode {
        LayoutNode {
            node:   LayoutVariant::Rule,
            width:  self.width .scaled(config),
            height: self.height.scaled(config),
            depth:  FontUnit::from(0),
        }
    }
}

impl AsLayoutNode for VariantGlyph {
    fn as_layout(&self, config: LayoutSettings) -> LayoutNode {
        match *self {
            VariantGlyph::Replacement(g) => {
                let glyph = font::glyph_metrics(g.unicode);
                glyph.as_layout(config)
            },

            VariantGlyph::Constructable(dir, ref c) => {
                match dir {
                    Direction::Vertical => {
                        let mut contents = builders::VBox::new();
                        for instr in c.iter().rev() {
                            contents.add_node(instr.glyph.as_layout(config));
                            if instr.overlap != FontUnit::from(0) {
                                let kern = -instr.overlap.scaled(config) - instr.glyph.depth();
                                contents.add_node(kern!(vert: kern));
                            }
                        }

                        contents.build()
                    },

                    Direction::Horizontal => {
                        let mut contents = builders::HBox::new();
                        for instr in c.iter() {
                            if instr.overlap != FontUnit::from(0) {
                                let kern = -instr.overlap.scaled(config);
                                contents.add_node(kern!(horz: kern));
                            }
                            contents.add_node(instr.glyph.as_layout(config));
                        }

                        contents.build()
                    }
                }
            },
        }
    }
}


fn scale<T>(n: T, config: LayoutSettings) -> FontUnit
    where FontUnit: From<T>,
          T: Mul<FontUnit, Output=FontUnit>
{
    match config.style {
        Style::Display |
        Style::DisplayCramped |
        Style::Text |
        Style::TextCramped
            => FontUnit::from(n),

        Style::Script |
        Style::ScriptCramped
            => n * constants::SCRIPT_PERCENT_SCALE_DOWN,

        Style::ScriptScript |
        Style::ScriptScriptCramped
            => n * constants::SCRIPT_SCRIPT_PERCENT_SCALE_DOWN,
    }
}

pub trait Scaled {
    fn scaled(self, LayoutSettings) -> FontUnit;
}

impl Scaled for FontUnit {
    fn scaled(self, config: LayoutSettings) -> FontUnit {
        scale(self, config)
    }
}

impl Scaled for Unit {
    fn scaled(self, config: LayoutSettings) -> FontUnit {
        match self {
            Unit::Font(size) => scale(FontUnit::from(size), config),
            Unit::Em(size) =>
                scale(constants::UNITS_PER_EM * FontUnit::from(size), config),
            Unit::Px(size) => {
                // We need to convert from Pixels back to font units
                let unit = FontUnit::from(size)
                    / constants::UNITS_PER_EM
                    / config.font_size;
                scale(FontUnit::from(unit), config)
            }
            _ => panic!("unsupported unit")
        }
    }
}
