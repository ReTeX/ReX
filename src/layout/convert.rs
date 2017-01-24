//! This is a collection of tools used for converting ParseNodes into LayoutNodes.
use font;
use font::constants;
use font::variants::VariantGlyph;
use font::Glyph;
use dimensions::{ FontUnit, Unit, Pixels };
use layout::LayoutSettings;

use super::builders;
use super::{ LayoutNode, LayoutVariant, LayoutGlyph };
use parser::nodes::Rule;

pub trait AsLayoutNode {
    fn as_layout(&self, config: LayoutSettings) -> LayoutNode;
}

impl AsLayoutNode for Glyph {
    fn as_layout(&self, config: LayoutSettings) -> LayoutNode {
        LayoutNode {
            height: self.height() .scaled(config),
            width:  self.advance().scaled(config),
            depth:  self.depth()  .scaled(config),
            node:   LayoutVariant::Glyph(LayoutGlyph {
                unicode: self.unicode,
                scale: config.style.font_scale(),
                attachment: self.attachment_offset().scaled(config),
                italics: self.italic_correction().scaled(config),
                offset:  Pixels(0.0),
            })
        }
    }
}

impl AsLayoutNode for Rule {
    fn as_layout(&self, config: LayoutSettings) -> LayoutNode {
        LayoutNode {
            node:   LayoutVariant::Rule,
            width:  self.width.scaled(config),
            height: self.height.scaled(config),
            depth:  Pixels(0f64),
        }
    }
}

use font::variants::Direction;
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
                                let unit = -instr.overlap;
                                let kern = unit.scaled(config);
                                contents.add_node(kern!(vert: kern));
                            }
                        }

                        contents.build()
                    },

                    Direction::Horizontal => {
                        let mut contents = builders::HBox::new();
                        for instr in c.iter() {
                            if instr.overlap != 0.0 {
                                let unit = Unit::Font(-instr.overlap);
                                let kern = unit.scaled(config);
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

pub trait ToPixels: Sized {
    fn as_pixels(self, font_size: f64) -> Pixels;
    fn scaled(self, config: LayoutSettings) -> Pixels {
        self.as_pixels(config.font_size) * config.style.font_scale()
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

impl ToPixels for FontUnit {
    fn as_pixels(self, font_size: f64) -> Pixels {
        Unit::from(self).as_pixels(font_size)
    }
}
