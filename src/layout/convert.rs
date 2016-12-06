//! This is a collection of tools used for converting ParseNodes into LayoutNodes.
use font;
use font::constants;
use font::variants::VariantGlyph;
use font::Glyph;
use dimensions::{ FontUnit, Unit, Unital, Pixels };
use render::FONT_SIZE;

use super::builders;
use super::{ Style, LayoutNode, LayoutVariant, LayoutGlyph };
use parser::nodes::Rule;

pub trait AsLayoutNode {
    fn as_layout(&self, sty: Style) -> LayoutNode;
}

impl AsLayoutNode for Glyph {
    fn as_layout(&self, style: Style) -> LayoutNode {
        LayoutNode {
            height: self.height() .scaled(style),
            width:  self.advance().scaled(style),
            depth:  self.depth()  .scaled(style),
            node:   LayoutVariant::Glyph(LayoutGlyph {
                unicode: self.unicode,
                scale: style.font_scale(),
                attachment: self.attachment_offset().scaled(style),
                italics: self.italic_correction().scaled(style),
                offset:  Pixels(0.0),
            })
        }
    }
}

impl AsLayoutNode for Rule {
    fn as_layout(&self, style: Style) -> LayoutNode {
        LayoutNode {
            node:   LayoutVariant::Rule,
            width:  self.width.scaled(style),
            height: self.height.scaled(style),
            depth:  Pixels(0f64),
        }
    }
}

use font::variants::Direction;
impl AsLayoutNode for VariantGlyph {
    fn as_layout(&self, style: Style) -> LayoutNode {
        match *self {
            VariantGlyph::Replacement(g) => {
                let glyph = font::glyph_metrics(g.unicode);
                glyph.as_layout(style)
            },

            VariantGlyph::Constructable(dir, ref c) => {
                match dir {
                    Direction::Vertical => {
                        let mut contents = builders::VBox::new();
                        for instr in c.iter().rev() {
                            contents.add_node(instr.glyph.as_layout(style));
                            if instr.overlap != 0.0 {
                                let unit = Unit::Font(-instr.overlap);
                                let kern = unit.scaled(style);
                                contents.add_node(kern!(vert: kern));
                            }
                        }

                        contents.build()
                    },

                    Direction::Horizontal => {
                        let mut contents = builders::HBox::new();
                        for instr in c.iter() {
                            contents.add_node(instr.glyph.as_layout(style));
                            if instr.overlap != 0.0 {
                                let unit = Unit::Font(-instr.overlap);
                                let kern = unit.scaled(style);
                                contents.add_node(kern!(horz: kern));
                            }
                        }

                        contents.build()
                    }
                }
            },
        }
    }
}

pub trait ToPixels: Sized {
    fn as_pixels(self) -> Pixels;
    fn scaled(self, sty: Style) -> Pixels {
        self.as_pixels() * sty.font_scale()
    }
}

impl ToPixels for Unit {
    // TODO: You can't assign pt values to fonts with given `font_size: f64`
    fn as_pixels(self) -> Pixels {
        Pixels(match self {
            Unit::Font(u) => u / f64::from(constants::UNITS_PER_EM) * FONT_SIZE,
            Unit::Em(u)   => u * FONT_SIZE,
            Unit::Ex(u)   => u * FONT_SIZE, // TODO: measure x width here
            Unit::Px(u)   => u
        })
    }
}

impl<U: Unital> ToPixels for FontUnit<U> {
    fn as_pixels(self) -> Pixels {
        Unit::from(self).as_pixels()
    }
}

pub trait ToFont {
    fn in_font(self) -> f64;
}

impl ToFont for Pixels {
    fn in_font(self) -> f64 { *self / FONT_SIZE * *constants::UNITS_PER_EM }
}