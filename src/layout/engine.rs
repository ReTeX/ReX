#![allow(unused_assignments)]
#![allow(unused_variables)]

use super::builders;
use super::{ Layout, LayoutNode, LayoutVariant, LayoutGlyph, Style };
use super::convert::AsLayoutNode;
use super::convert::ToPixels;

use dimensions::{ Pixels, Unit };
use font;
use font::GLYPHS;
use font::IsAtom;
use font::SYMBOLS;
use font::constants::*;
use font::glyph_metrics;
use font::variants::Variant;
use layout::spacing::{atom_spacing, Spacing};
use parser::nodes::{ ParseNode, AtomType };
use render::FONT_SIZE;

/// This method takes the parsing nodes and layouts them to layout nodes.
#[allow(unconditional_recursion)]
#[allow(dead_code)]
pub fn layout(nodes: &mut [ParseNode], mut style: Style) -> Layout {
    use super::spacing::normalize_types;
    normalize_types(nodes);

    let mut prev_at: Option<AtomType> = None;
    let mut result = Layout::new();

    for node in nodes {
        if let Some(p_at) = prev_at {
            if let Some(at) = node.atom_type() {
                let sp = atom_spacing(p_at, at);
                if sp != Spacing::None {
                    let kern = sp.to_unit().scaled(style);
                    result.add_node(kern!(horz: kern));
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
                            result.add_node(vbox!(offset: shift; largeop));
                        } else {
                            result.add_node(glyph.as_layout(style));
                        }
                    },
                    _ => result.add_node(glyph.as_layout(style)),
                }
            },

            ParseNode::Group(ref mut gp) =>
                result.add_node(layout(gp, style).as_node()),

            ParseNode::Rule(rule) =>
                result.add_node(rule.as_layout(style)),

            ParseNode::Kerning(kern) =>
                result.add_node(kern!(horz: kern.scaled(style))),

            ParseNode::Radical(ref mut rad) => {
                //Reference rule 11 from pg 443 of TeXBook
                let style = style.cramped_variant();
                let contents = layout(&mut rad.inner, style).as_node();
                let sqrt  = &GLYPHS[&SYMBOLS["sqrt"].unicode];

                let gap = match style.cramped() {
                    true  => RADICAL_VERTICAL_GAP,
                    false => RADICAL_DISPLAY_STYLE_VERTICAL_GAP,
                };

                let size = (*contents.height - *contents.depth)
                    / FONT_SIZE * 1000.0     // Convert to font units
                    + *gap
                    + *RADICAL_RULE_THICKNESS
                    + *RADICAL_EXTRA_ASCENDER; // Minimum gap

                let glyph = sqrt.variant(size).as_layout(style);
                let kerning = glyph.height
                    - contents.height
                    - RADICAL_RULE_THICKNESS.scaled(style)
                    - RADICAL_EXTRA_ASCENDER.scaled(style)
                    + contents.depth;

                result.add_node(vbox!(offset: -1.0 * contents.depth; glyph));
                result.add_node(vbox!(
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
                    Some(ref b) => layout(&mut [ *b.clone() ], style),
                    None        => Layout::new(),
                };

                let mut sup = match scripts.superscript {
                    Some(ref b) => layout(&mut [ *b.clone() ], style.superscript_variant()),
                    None        => Layout::new(),
                };

                let sub = match scripts.subscript {
                    Some(ref b) => layout(&mut [ *b.clone() ], style.subscript_variant()),
                    None        => Layout::new(),
                };

                println!("{:?}", scripts.base);

                let mut italics_correction = Pixels(0.0);
                if let Some(ref b) = scripts.base {
                    if let Some(AtomType::Operator(limits)) = b.atom_type() {
                        if let ParseNode::Symbol(gly) = **b {
                            if limits {
                                let glyph = glyph_metrics(gly.unicode);
                                italics_correction = Unit::Font(glyph.italics as f64)
                                    .scaled(style);

                                let width = base.width
                                    .max(sub.width)
                                    .max(sup.width);

                                let height = base.height - base.depth;
                                let kern1 = UPPER_LIMIT_BASELINE_RISE_MIN
                                    .scaled(style.superscript_variant())
                                    .max(UPPER_LIMIT_GAP_MIN.scaled(style) - sup.depth);
                                let kern2 = LOWER_LIMIT_BASELINE_DROP_MIN
                                    .scaled(style.subscript_variant())
                                    .max(LOWER_LIMIT_GAP_MIN.scaled(style) + sub.height);

                                // TODO: This doesn't account for variant glyphs
                                let offset = (base.height + base.depth) / 2.0
                                    - AXIS_HEIGHT.scaled(style)
                                    + sub.height + kern2;

                                let w1 = sup.width;
                                let w2 = sub.width;

                                use super::Alignment;
                                result.add_node(vbox!(
                                    offset: offset;
                                    hbox![align: Alignment::Centered(w1);
                                        width: width;
                                        kern![horz: italics_correction / 2.0],
                                        sup.as_node()
                                    ],
                                    kern!(vert: kern1),
                                    base.as_node(),
                                    kern!(vert: kern2),
                                    hbox![align: Alignment::Centered(w2);
                                        width: width;
                                        sub.as_node(),
                                        kern![horz: italics_correction / 2.0]
                                    ]
                                ));

                                continue
                            }
                        }
                    }
                }

                // We calculate the vertical positions of the scripts.  The `adjust_up`
                // variable will describe how far we need to adjust the superscript up.
                let mut adjust_up          = Pixels(0.0);
                let mut adjust_down        = Pixels(0.0);

               if let Some(_) = scripts.superscript {
                    // We start with default values provided from the font.  These are called
                    // the standard positions in the OpenType specification.
                    adjust_up = match style.cramped() {
                        true  => SUPERSCRIPT_SHIFT_UP_CRAMPED,
                        false => SUPERSCRIPT_SHIFT_UP,
                    }.scaled(style);

                    // Next we check to see if the vertical shift meets the minimum
                    // clearance relative to the base.
                    let height   = base.height;
                    let drop_max = SUPERSCRIPT_BASELINE_DROP_MAX
                        .scaled(style);

                    adjust_up = adjust_up
                        .max(height - drop_max)
                        .max(SUPERSCRIPT_BOTTOM_MIN.scaled(style) - sup.depth);

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

                result.add_node(base.as_node());
                result.add_node(contents.build());
            },

            ParseNode::Accent(ref acc) => {
                // TODO: Account for bottom accents (accent flag?)
                //   (LuaTeX) BottomAccent: The vertical placement of a bottom accent is
                //               straight below the accentee, no correction takes place.

                let nucleus = layout(&mut [ *acc.nucleus.clone() ], style.cramped_variant());
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
                result.add_node(vbox!(
                    hbox!(kern!(horz: offset), symbol),
                    kern!(vert: -1.0 * delta),
                    nucleus.as_node()
                ));
            },

            ParseNode::Style(sty) =>
                style = sty,

            ParseNode::Delimited(ref mut d) => {
                let inner = layout(&mut d.inner, style).as_node();

                // Convert inner group dimensions to font unit
                let height = *inner.height / FONT_SIZE * *UNITS_PER_EM as f64;
                let depth  = *inner.depth  / FONT_SIZE * *UNITS_PER_EM as f64;

                // Only extend if we meet a certain size
                // TODO: This quick height check doesn't seem to be strong enough,
                // reference: http://tug.org/pipermail/luatex/2010-July/001745.html
                if height - depth > *DELIMITED_SUB_FORMULA_MIN_HEIGHT as f64 {
                    let axis = *AXIS_HEIGHT as f64;

                    let mut clearance = 2. * (height - axis).max(axis - depth);
                    clearance = (DELIMITER_FACTOR * clearance)
                        .max(height - depth - *DELIMITER_SHORT_FALL as f64);

                    let axis = AXIS_HEIGHT.scaled(style);
                    let left = match d.left.unicode {
                        46  => kern!(horz: NULL_DELIMITER_SPACE),
                        _   =>
                            glyph_metrics(d.left.unicode)
                                .variant(clearance)
                                .as_layout(style)
                                .centered(axis),
                    };

                    let right = match d.right.unicode {
                        46  => kern!(horz: NULL_DELIMITER_SPACE),
                        _   =>
                            glyph_metrics(d.right.unicode)
                                .variant(clearance)
                                .as_layout(style)
                                .centered(axis),
                    };

                    result.add_node(left);
                    result.add_node(inner);
                    result.add_node(right);
                } else {
                    let left  = match d.left.unicode {
                        46 => kern!(horz: NULL_DELIMITER_SPACE),
                        _  => glyph_metrics(d.left.unicode).as_layout(style),
                    };

                    let right = match d.right.unicode {
                        46 => kern!(horz: NULL_DELIMITER_SPACE),
                        _  => glyph_metrics(d.right.unicode).as_layout(style),
                    };

                    result.add_node(left);
                    result.add_node(inner);
                    result.add_node(right);
                }
            },

            ParseNode::GenFraction(ref mut frac) => {
                use parser::nodes::BarThickness;
                use super::Alignment;

                let bar = match frac.bar_thickness {
                    BarThickness::Default => FRACTION_RULE_THICKNESS.scaled(style),
                    BarThickness::None    => Pixels(0.0),
                    BarThickness::Unit(u) => u.scaled(style),
                } ;

                let mut n = layout(&mut frac.numerator,   style.numerator());
                let mut d = layout(&mut frac.denominator, style.denominator());

                if n.width > d.width {
                    d.alignment = Alignment::Centered(d.width);
                    d.width     = n.width;
                } else {
                    n.alignment = Alignment::Centered(n.width);
                    n.width     = d.width;
                }

                let numer = n.as_node();
                let denom = d.as_node();

                let mut shift_up   = Pixels(0.0);
                let mut shift_down = Pixels(0.0);
                let mut gap_num    = Pixels(0.0);
                let mut gap_denom  = Pixels(0.0);
                if style > Style::Text {
                    shift_up = FRACTION_NUMERATOR_DISPLAY_STYLE_SHIFT_UP
                        .scaled(style.numerator());
                    shift_down = FRACTION_DENOMINATOR_DISPLAY_STYLE_SHIFT_DOWN
                        .scaled(style.denominator());
                    gap_num = FRACTION_NUM_DISPLAY_STYLE_GAP_MIN
                        .scaled(style.numerator());
                    gap_denom = FRACTION_DENOM_DISPLAY_STYLE_GAP_MIN
                        .scaled(style.denominator());
                } else {
                    shift_up = FRACTION_NUMERATOR_SHIFT_UP
                        .scaled(style.numerator());
                    shift_down = FRACTION_DENOMINATOR_SHIFT_DOWN
                        .scaled(style.denominator());
                    gap_num = FRACTION_NUMERATOR_GAP_MIN
                        .scaled(style.numerator());
                    gap_denom = FRACTION_DENOMINATOR_GAP_MIN
                        .scaled(style.denominator());
                }

                let axis = AXIS_HEIGHT.scaled(style);

                // I think this has to do with an inconsistency with the font parameters hack
                if style > Style::Text {
                    shift_up   -= axis;
                } else {
                    shift_up   += -1.0*bar;
                }

                shift_up = shift_up.max(gap_num - numer.depth);
                shift_down = shift_down.max(gap_denom + denom.height);

                // Another font inconsistency??
                if style > Style::Text {
                    shift_down -= 0.5*bar;
                }

                let width  = numer.width.max(numer.width);
                let offset = shift_down + 1.5 * bar - axis;
                let inner = vbox!(
                    offset: offset;
                    numer,
                    kern!(vert: shift_up),
                    rule!(width: width, height: bar),
                    kern!(vert: shift_down - denom.height),
                    denom
                );

                let height = *inner.height / FONT_SIZE * *UNITS_PER_EM as f64;
                let depth  = *inner.depth  / FONT_SIZE * *UNITS_PER_EM as f64;
                let mut clearance = 2. * (height - *axis).max(*axis - depth);
                clearance = (DELIMITER_FACTOR * clearance)
                    .max(height - depth - *DELIMITER_SHORT_FALL as f64);
                if let Some(delim) = frac.left_delimiter {
                    let glyph = glyph_metrics(delim.unicode)
                        .variant(clearance)
                        .as_layout(style)
                        .centered(axis);
                    result.add_node(glyph);
                } else {
                    result.add_node(kern!(horz: NULL_DELIMITER_SPACE))
                }

                result.add_node(inner);

                if let Some(delim) = frac.right_delimiter {
                    let glyph = glyph_metrics(delim.unicode)
                        .variant(clearance)
                        .as_layout(style)
                        .centered(axis);
                    result.add_node(glyph);
                } else {
                    result.add_node(kern!(horz: NULL_DELIMITER_SPACE))
                }
            },

            _ => (),
       }
    }

    result.finalize()
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