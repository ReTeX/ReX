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
use font::variants::VariantGlyph;
use layout::spacing::{atom_spacing, Spacing};
use parser::nodes::{ ParseNode, AtomType, AtomChange };
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
                            let largeop = glyph.vert_variant(size).as_layout(style);

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
                let style = style.cramped();
                let contents = layout(&mut rad.inner, style).as_node();
                let sqrt  = &GLYPHS[&SYMBOLS["sqrt"].unicode];

                let gap = match style.is_cramped() {
                    true  => RADICAL_VERTICAL_GAP,
                    false => RADICAL_DISPLAY_STYLE_VERTICAL_GAP,
                };

                let size = (*contents.height - *contents.depth)
                    / FONT_SIZE * 1000.0     // Convert to font units
                    + *gap
                    + *RADICAL_RULE_THICKNESS
                    + *RADICAL_EXTRA_ASCENDER; // Minimum gap

                let glyph = sqrt.vert_variant(size).as_layout(style);
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

                // Script processing is handled differently for Operators with limits.
                let mut italics_correction = Pixels(0.0);
                if let Some(ref b) = scripts.base {
                    if let Some(AtomType::Operator(limits)) = b.atom_type() {
                        if limits {
                            if let Some(gly) = b.is_symbol() {
                                let glyph = glyph_metrics(gly.unicode);
                                italics_correction = Unit::Font(glyph.italics as f64)
                                    .scaled(style);
                            }

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

                // We calculate the vertical positions of the scripts.  The `adjust_up`
                // variable will describe how far we need to adjust the superscript up.
                let mut adjust_up          = Pixels(0.0);
                let mut adjust_down        = Pixels(0.0);
                let mut accent_correction  = Pixels(0.0);

               if let Some(_) = scripts.superscript {
                    // We start with default values provided from the font.  These are called
                    // the standard positions in the OpenType specification.
                    adjust_up = match style.is_cramped() {
                        true  => SUPERSCRIPT_SHIFT_UP_CRAMPED,
                        false => SUPERSCRIPT_SHIFT_UP,
                    }.scaled(style);

                    // Next we check to see if the vertical shift meets the minimum
                    // clearance relative to the base.

                    // For accents, whose base is a simple symbol, we do not take
                    // the accent into account while positioning the superscript.
                    // TODO: This should probably be recursive.
                    let height = if let Some(ref b) = scripts.base {
                            if let ParseNode::Accent(ref acc) = **b {
                                if let Some(ref sym) = acc.nucleus.is_symbol() {
                                    let bh = glyph_metrics(sym.unicode)
                                        .height()
                                        .scaled(style);
                                    accent_correction = base.height - bh;
                                    println!("Scripts with accent. {:?}",
                                        accent_correction);
                                    println!("Accent base: {:?}", b);
                                    bh
                                } else { base.height }
                            } else { base.height }
                        } else { base.height };

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
                // [x] If there is no accent, typeset like normal.
                // [x] Take largest accent _smaller_ than nucleus.

                // [x] Determine offset of accent:
                //   (a) Accent has attachment correction:
                //     [x] If accentee has attachment correction,
                //         then align attachment corrections of both.
                //     [x] Otherwise, align attachment correction of
                //         accentee with center of nucleus, plus
                //         italics correction of nucleus is a symbol.
                //
                //   (b) Accent has no attachment correction:
                //     [x] If accentee has attachment correction,
                //         center of accent with accent correction of base.
                //     [x] Align accent center with base center (plus)
                //         italics correction if it's a symbol.
                //
                // [-] For superscripts, if character is simple symbol,
                //     scripts should not take accent into account for height.
                // [x] Layout nucleus with style cramped.
                // [x] Baseline of result == baseline of base.
                // [ ] The width of the resulting box is the width of the base.
                // [ ] Bottom accents: vertical placement is directly below nucleus,
                //       no correction takes place.
                // [ ] WideAccent vs Accent: Don't expand Accent types.

                let base = layout(&mut [ *acc.nucleus.clone() ], style.cramped());
                let accent_variant = glyph_metrics(acc.symbol.unicode)
                    .horz_variant(*base.width / FONT_SIZE * *UNITS_PER_EM);
                let accent = accent_variant.as_layout(style);

                // Attachment points for accent & base are calculated by
                //   (a) Non-symbol: width / 2.0,
                //   (b) Symbol:
                //      1. Attachment point (if there is one)
                //      2. Otherwise: (width + ic) / 2.0
                let base_offset = if base.contents.len() != 1 {
                        base.width / 2.0
                    } else if let Some(ref sym) = base.contents[0].is_symbol() {
                        let glyph = glyph_metrics(sym.unicode);
                        if glyph.attachment != 0 {
                            Unit::Font(glyph.attachment as f64).scaled(style)
                        } else {
                            Unit::Font((glyph.advance as i16 + glyph.italics)
                                as f64 / 2.0).scaled(style)
                        }
                    } else {
                        base.width / 2.0
                    };

                let acc_offset = match accent_variant {
                        VariantGlyph::Replacement(sym) => {
                            let glyph = glyph_metrics(sym.unicode);
                            if glyph.attachment != 0 {
                                Unit::Font(
                                    glyph.attachment as f64
                                ).as_pixels()
                            } else {
                                // For glyphs without attachmens, we must
                                // also account for combining glyphs
                                let off = 0.5*(sym.bbox.2 + sym.bbox.0) as f64;
                                Unit::Font(off).scaled(style)
                            }
                        },

                        VariantGlyph::Constructable(_, _) =>
                            accent.width / 2.0
                    };

                // Do not place the accent any _further_ than you would if given
                // an `x` character in the current style.
                let delta = -1. * base.height
                    .min(ACCENT_BASE_HEIGHT.scaled(style));

                // By not placing an offset on this vbox, we are assured that the
                // baseline will match the baseline of `base.as_node()`
                result.add_node(vbox!(
                    hbox!(kern!(horz: base_offset - acc_offset), accent),
                    kern!(vert: delta),
                    base.as_node()
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
                if height.max(-1. * depth) > 0.5 * *DELIMITED_SUB_FORMULA_MIN_HEIGHT as f64 {
                    let axis = *AXIS_HEIGHT as f64;

                    let mut clearance = 2. * (height - axis).max(axis - depth);
                    clearance = (DELIMITER_FACTOR * clearance)
                        .max(height - depth - *DELIMITER_SHORT_FALL as f64);

                    let axis = AXIS_HEIGHT.scaled(style);
                    let left = match d.left.unicode {
                        46  => kern!(horz: NULL_DELIMITER_SPACE),
                        _   =>
                            glyph_metrics(d.left.unicode)
                                .vert_variant(clearance)
                                .as_layout(style)
                                .centered(axis),
                    };

                    let right = match d.right.unicode {
                        46  => kern!(horz: NULL_DELIMITER_SPACE),
                        _   =>
                            glyph_metrics(d.right.unicode)
                                .vert_variant(clearance)
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
                let axis = AXIS_HEIGHT.scaled(style);


                let mut shift_up   = Pixels(0.0);
                let mut shift_down = Pixels(0.0);
                let mut gap_num    = Pixels(0.0);
                let mut gap_denom  = Pixels(0.0);
                if style > Style::Text {
                    shift_up = FRACTION_NUMERATOR_DISPLAY_STYLE_SHIFT_UP
                        .scaled(style);
                    shift_down = FRACTION_DENOMINATOR_DISPLAY_STYLE_SHIFT_DOWN
                        .scaled(style);
                    gap_num = FRACTION_NUM_DISPLAY_STYLE_GAP_MIN
                        .scaled(style);
                    gap_denom = FRACTION_DENOM_DISPLAY_STYLE_GAP_MIN
                        .scaled(style);
                } else {
                    shift_up = FRACTION_NUMERATOR_SHIFT_UP
                        .scaled(style);
                    shift_down = FRACTION_DENOMINATOR_SHIFT_DOWN
                        .scaled(style);
                    gap_num = FRACTION_NUMERATOR_GAP_MIN
                        .scaled(style);
                    gap_denom = FRACTION_DENOMINATOR_GAP_MIN
                        .scaled(style);
                }

                // TODO: Investigate.
                // It appears that the vertical layout system handles
                // Rules differentally than expect.  This result with me
                // being off by 1 Rule width in a few places.

                let kern_up = (shift_up - axis + bar/2.0).max(gap_num + numer.depth);
                let kern_down = (shift_down + axis - denom.height - 1.5*bar)
                    .max(gap_denom);
                let offset = denom.height + kern_down + 1.5*bar - axis;

                let width  = numer.width.max(numer.width);
                let inner = vbox!(
                    offset: offset;
                    numer,
                    kern!(vert: kern_up),
                    rule!(width: width, height: bar),
                    kern!(vert: kern_down),
                    denom
                );

                result.add_node(kern!(horz: NULL_DELIMITER_SPACE));
                result.add_node(inner);
                result.add_node(kern!(horz: NULL_DELIMITER_SPACE));
            },

            ParseNode::AtomChange(AtomChange { at, ref mut inner }) => {
                // Atom Types can change control flow for operators.
                // We handle this change in control flow here,
                // otherwise we do nothing.

                // TODO: This adds an unnecessary hbox.  Remove them.
                if inner.len() != 1 {
                    result.add_node(layout(inner, style).as_node());
                    continue;
                }

                match at {
                    AtomType::Operator(_) => {
                        if let Some(sym) = inner[0].is_symbol() {
                            inner[0].set_atom_type(at);
                        }
                    }
                    _ => (),
                }

                result.add_node(layout(inner, style).as_node());
            }
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

impl IsSymbol for LayoutNode {
    fn is_symbol(&self) -> Option<LayoutGlyph> {
        match self.node {
            LayoutVariant::Glyph(gly) => Some(gly),
            LayoutVariant::HorizontalBox(ref hb) => {
                if hb.contents.len() != 1 {
                    None
                } else {
                    hb.contents[0].is_symbol()
                }
            },
            LayoutVariant::VerticalBox(ref vb) => {
                if vb.contents.len() != 1 {
                    None
                } else {
                    vb.contents[0].is_symbol()
                }
            }
            _ => None,
        }
    }
}