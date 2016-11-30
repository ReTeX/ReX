use super::builders;
use super::{ Layout, LayoutNode, LayoutVariant, LayoutGlyph, Style };

use dimensions::{ Pixels, Unit };
use font::GLYPHS;
use font::IsAtom;
use font::SYMBOLS;
use font::constants::*;
use font::glyph_metrics;
use font::variants::Variant;
use font::variants::VariantGlyph;
use font;
use layout::ToPixels;
use layout::spacing::{atom_spacing, Spacing};
use parser::nodes::{ ParseNode, AtomType, Rule };
use render::FONT_SIZE;


/// This method takes the parsing nodes and reduces them to layout nodes.
#[allow(unconditional_recursion)]
#[allow(dead_code)]
pub fn reduce(nodes: &mut [ParseNode], mut style: Style) -> Layout {
    use super::spacing::normalize_types;
    normalize_types(nodes);

    let mut prev_at: Option<AtomType> = None;
    let mut layout = Layout::new();

    for node in nodes {
        if let Some(p_at) = prev_at {
            if let Some(at) = node.atom_type() {
                let sp = atom_spacing(p_at, at);
                if sp != Spacing::None {
                    let kern = sp.to_unit().scaled(style);
                    layout.add_node(kern!(horz: kern));
                }
            }
        }

        // TODO: May need to ignore this if transparent atom_type.
        prev_at = node.atom_type();

        match *node {
            ParseNode::Symbol(sym) => {
                let glyph = font::glyph_metrics(sym.unicode);
                match sym.atom_type {
                    AtomType::Operator(_) => {
                        if style == Style::Display {
                            let size = *DISPLAY_OPERATOR_MIN_HEIGHT as f64;
                            let axis_offset = AXIS_HEIGHT.scaled(style);
                            let largeop = glyph.variant(size).as_layout(style);

                            // Vertically center
                            let shift = 0.5 *
                                (largeop.height + largeop.depth) - axis_offset;
                            layout.add_node(vbox!(offset: shift; largeop));
                        } else {
                            layout.add_node(glyph.as_layout(style));
                        }
                    },
                    _ => layout.add_node(glyph.as_layout(style)),
                }
            },

            ParseNode::Group(ref mut gp) =>
                layout.add_node(reduce(gp, style).as_node()),

            ParseNode::Rule(rule) =>
                layout.add_node(rule.as_layout(style)),

            ParseNode::Kerning(kern) =>
                layout.add_node(kern!(horz: kern.scaled(style))),

            ParseNode::Radical(ref mut rad) => {
                //Reference rule 11 from pg 443 of TeXBook
                let style = style.cramped_variant();
                let contents = reduce(&mut rad.inner, style).as_node();
                let sqrt  = &GLYPHS[&SYMBOLS["sqrt"].unicode];

                let gap = match style.cramped() {
                    true  => RADICAL_VERTICAL_GAP,
                    false => RADICAL_DISPLAY_STYLE_VERTICAL_GAP,
                };

                let clearance = (*contents.height - *contents.depth)
                    / FONT_SIZE * 1000.0     // Convert to font units
                    + f64::from(gap)
                    + f64::from(RADICAL_RULE_THICKNESS)
                    + f64::from(RADICAL_EXTRA_ASCENDER); // Minimum gap

                let glyph = sqrt.variant(clearance).as_layout(style);
                let kerning = glyph.height
                    - contents.height
                    - RADICAL_RULE_THICKNESS.scaled(style)
                    - RADICAL_EXTRA_ASCENDER.scaled(style)
                    + contents.depth;

                layout.add_node(vbox!(offset: -1.0 * contents.depth; glyph));
                layout.add_node(vbox!(
                        kern!(vert: RADICAL_EXTRA_ASCENDER.scaled(style)),
                        rule!(
                            width:  contents.width,
                            height: RADICAL_RULE_THICKNESS.scaled(style)),
                        kern!(vert: kerning),
                        contents
                    ));
            },

            ParseNode::Scripts(ref mut scripts) => {
                // Vertical position of the script is calculated.  This depends on the following
                // information: (see page 26 of https://www.tug.org/tugboat/tb30-1/tb94vieth.pdf)

                // Before we start calculating layout, we layout the contents
                // of the base and scripts.  These dimensions will be needed.
                let base = match scripts.base {
                    Some(ref b) => reduce(&mut [ *b.clone() ], style),
                    None        => Layout::new(),
                };

                let mut sup = match scripts.superscript {
                    Some(ref b) => reduce(&mut [ *b.clone() ], style.superscript_variant()),
                    None        => Layout::new(),
                };

                let sub = match scripts.subscript {
                    Some(ref b) => reduce(&mut [ *b.clone() ], style.subscript_variant()),
                    None        => Layout::new(),
                };

                // We calculate the vertical positions of the scripts.  The `adjust_up`
                // variable will describe how far we need to adjust the superscript up.
                let mut italics_correction = Pixels(0.0);
                let mut adjust_up          = Pixels(0.0);
                let mut adjust_down        = Pixels(0.0);

               if let Some(_) = scripts.superscript {
                    // We start with default values provided from the font.  These are called
                    // the standard positions in the OpenType specification.
                    let mut default = match style.cramped() {
                        true  => SUPERSCRIPT_SHIFT_UP_CRAMPED,
                        false => SUPERSCRIPT_SHIFT_UP,
                    }.scaled(style);

                    // Next we check to see if the vertical shift meets the minimum
                    // clearance relative to the base.
                    let height   = base.height;
                    let drop_max = SUPERSCRIPT_BASELINE_DROP_MAX
                        .scaled(style);

                    if height - default > drop_max {
                        default = height - drop_max;
                    }

                    // Next we check that the bottom of the superscript is far enough
                    // from the bottom of the base
                    if sup.depth + default < SUPERSCRIPT_BOTTOM_MIN.scaled(style) {
                        default = SUPERSCRIPT_BOTTOM_MIN.scaled(style) - sup.depth;
                    }

                    // For superscripts we need to calculate the italics correction
                    // if the base is simply a symbol.
                    // TODO: This can probably be cleaned up a bit.
                    if let Some(ref bx) = scripts.base {
                        if let ParseNode::Symbol(sym) = **bx {
                            let glyph = glyph_metrics(sym.unicode);
                            italics_correction = Unit::Font(glyph.italics as f64)
                                .scaled(style)
                        }
                    }

                    adjust_up = default
                }

                // We calculate the vertical position of the subscripts.  The `adjust_down`
                // variable will describe how far we need to adjust the subscript down.
                if let Some(_) = scripts.subscript {
                    // We start with the default values provided from the font.
                    adjust_down = SUBSCRIPT_SHIFT_DOWN.scaled(style);

                    let depth = -1. * base.depth;
                    let drop_min = SUBSCRIPT_BASELINE_DROP_MIN.scaled(style);

                    adjust_down = adjust_down
                        .max(sub.height - SUBSCRIPT_TOP_MAX.scaled(style))
                        .max(drop_min + depth);
                }

                // TODO: lazy gap fix; see BottomMaxWithSubscript
                if !sub.contents.is_empty() && !sup.contents.is_empty() {
                    let sup_bot = adjust_up + sup.depth;
                    let sub_top = sub.height - adjust_down;
                    let gap_min = SUB_SUPERSCRIPT_GAP_MIN.scaled(style);
                    if sup_bot - sub_top < gap_min {
                        let adjust = (gap_min - sup_bot + sub_top) / 2.0;
                        adjust_up   += adjust;
                        adjust_down += adjust;
                    }
                }

                let mut contents = builders::VBox::new();
                if !sup.contents.is_empty() {
                    if italics_correction != Pixels(0.0) {
                        sup.contents.insert(0, kern!(horz: italics_correction));
                        sup.width += italics_correction;
                    }

                    let corrected_adjust =
                        adjust_up - sub.height + adjust_down;

                    contents.add_node(sup.as_node());
                    contents.add_node(kern!(vert: corrected_adjust));
                }

                contents.set_offset(adjust_down);
                if !sub.contents.is_empty() {
                    contents.add_node(sub.as_node());
                }

                layout.add_node(base.as_node());
                layout.add_node(contents.build());
            },

            ParseNode::Accent(ref acc) => {
                // TODO: Account for bottom accents (accent flag?)
                //   (LuaTeX) BottomAccent: The vertical placement of a bottom accent is
                //               straight below the accentee, no correction takes place.

                let nucleus = reduce(&mut [ *acc.nucleus.clone() ], style.cramped_variant());
                let delta = nucleus.height.min(ACCENT_BASE_HEIGHT
                    .scaled(style));

                let skew = if let Some(ref sym) = nucleus.is_symbol() {
                    sym.attachment
                } else { Pixels(0.0) };

                // If the accent has an attachment correction point, we will
                // align the attachment correction points of both the accent
                // and accentee.  Otherwise, we will align the center of the
                // accent with the attachment correction of the accentee.
                let symbol  = glyph_metrics(acc.symbol.unicode);
                let offset = if symbol.attachment != 0 {
                    skew - symbol.attachment_offset().scaled(style)
                } else {
                    let offset_x = Unit::Font(symbol.bbox.0 as f64)
                        .scaled(style);
                    let sym_width = Unit::Font((symbol.bbox.2 - symbol.bbox.0) as f64)
                        .scaled(style);

                    skew +
                        -1.0 * offset_x   // correct for combining characters
                        - 0.5 * (nucleus.width - sym_width)  // align centers
                };

                let symbol = symbol.as_layout(style);
                layout.add_node(vbox!(
                    hbox!(kern!(horz: offset), symbol),
                    kern!(vert: -1.0 * delta),
                    nucleus.as_node()
                ));
            },

            ParseNode::Style(sty) =>
                style = sty,

            _ => (),
       }
    }

    layout.finalize()
}

trait IsSymbol {
    fn is_symbol(&self) -> Option<LayoutGlyph>;
}

impl IsSymbol for Layout {
    fn is_symbol(&self) -> Option<LayoutGlyph> {
        if self.contents.len() != 1 { return None }
        let node = &self.contents[0];
        if let LayoutVariant::Glyph(ref lg) = node.node {
            return Some(lg.clone())
        } else { None }
    }
}

trait AsLayoutNode {
    fn as_layout(&self, sty: Style) -> LayoutNode;
}

impl AsLayoutNode for font::Glyph {
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

impl AsLayoutNode for VariantGlyph {
    fn as_layout(&self, style: Style) -> LayoutNode {
        match *self {
            VariantGlyph::Replacement(g) => {
                let glyph = font::glyph_metrics(g.unicode);
                glyph.as_layout(style)
            },

            VariantGlyph::Constructable(ref c) => {
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
        }
    }
}