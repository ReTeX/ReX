#![allow(unused_assignments)]
#![allow(unused_variables)]

use super::Alignment;
use super::builders;
use super::{ Layout, LayoutNode, LayoutVariant, LayoutGlyph, Style };
use super::convert::AsLayoutNode;
use super::convert::ToPixels;

use dimensions::{ Pixels, Unit };
use font;
use font::IsAtom;
use font::constants::*;
use font::glyph_metrics;
use font::variants::Variant;
use font::variants::VariantGlyph;
use font::Symbol;
use layout::spacing::{atom_spacing, Spacing};
use parser::nodes::BarThickness;
use parser::nodes::{ ParseNode, AtomType, AtomChange, Accent, Delimited, GenFraction, Radical, Scripts };
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
                let sp = atom_spacing(p_at, at, style);
                if sp != Spacing::None {
                    let kern = sp.to_unit().scaled(style);
                    result.add_node(kern!(horz: kern));
                }
            }
        }

        // TODO: May need to ignore this if transparent atom_type.
        prev_at = node.atom_type();

        match *node {
            ParseNode::Symbol(sym) => add_symbol(&mut result, sym, style),
            ParseNode::Scripts(ref mut scripts) => add_scripts(&mut result, scripts, style),
            ParseNode::Radical(ref mut rad) => add_radical(&mut result, rad, style),
            ParseNode::Delimited(ref mut delim) => add_delimited(&mut result, delim, style),
            ParseNode::Accent(ref acc) => add_accent(&mut result, acc, style),
            ParseNode::GenFraction(ref mut frac) => add_frac(&mut result, frac, style),
            ParseNode::Group(ref mut gp) => result.add_node(layout(gp, style).as_node()),
            ParseNode::Rule(rule) => result.add_node(rule.as_layout(style)),
            ParseNode::Kerning(kern) => result.add_node(kern!(horz: kern.scaled(style))),
            ParseNode::Style(sty) => style = sty,
            ParseNode::AtomChange(AtomChange { at, ref mut inner }) =>
                add_atom_change(&mut result, at, inner, style),

            _ => println!("Warning: Ignored ParseNode: {:?}", node),
       }
    }

    result.finalize()
}

fn add_symbol(result: &mut Layout, sym: Symbol, style: Style) {
    // Operators are handled specially.  We may need to find a larger
    // symbol and vertical center it.
    if let AtomType::Operator(_) = sym.atom_type {
        add_largeop(result, sym, style);
    } else {
        let glyph = font::glyph_metrics(sym.unicode);
        result.add_node(glyph.as_layout(style));
    }
}

fn add_largeop(result: &mut Layout, sym: Symbol, style: Style) {
    let glyph = font::glyph_metrics(sym.unicode);

    if style > Style::Text {
        let size = *DISPLAY_OPERATOR_MIN_HEIGHT as f64;
        let axis_offset = AXIS_HEIGHT.scaled(style);

        let largeop = glyph.vert_variant(size).as_layout(style);
        let shift = 0.5 * (largeop.height + largeop.depth) - axis_offset;

        result.add_node(vbox!(offset: shift; largeop));
    } else {
        result.add_node(glyph.as_layout(style));
    }
}

fn add_accent(result: &mut Layout, acc: &Accent, style: Style) {
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
    // [x] Layout nucleus with style crampedelim.
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
}

fn add_delimited(result: &mut Layout, delim: &mut Delimited, style: Style) {
    let inner = layout(&mut delim.inner, style).as_node();

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
        let left = match delim.left.unicode {
            46  => kern!(horz: NULL_DELIMITER_SPACE),
            _   =>
                glyph_metrics(delim.left.unicode)
                    .vert_variant(clearance)
                    .as_layout(style)
                    .centered(axis),
        };

        let right = match delim.right.unicode {
            46  => kern!(horz: NULL_DELIMITER_SPACE),
            _   =>
                glyph_metrics(delim.right.unicode)
                    .vert_variant(clearance)
                    .as_layout(style)
                    .centered(axis),
        };

        result.add_node(left);
        result.add_node(inner);
        result.add_node(right);
    } else {
        let left  = match delim.left.unicode {
            46 => kern!(horz: NULL_DELIMITER_SPACE),
            _  => glyph_metrics(delim.left.unicode).as_layout(style),
        };

        let right = match delim.right.unicode {
            46 => kern!(horz: NULL_DELIMITER_SPACE),
            _  => glyph_metrics(delim.right.unicode).as_layout(style),
        };

        result.add_node(left);
        result.add_node(inner);
        result.add_node(right);
    }
}

fn add_scripts(result: &mut Layout, scripts: &mut Scripts, style: Style) {
    // See: https://tug.org/TUGboat/tb27-1/tb86jackowski.pdf
    //      https://www.tug.org/tugboat/tb30-1/tb94vieth.pdf

    let base = match scripts.base {
        Some(ref b) => layout(&mut [ *b.clone() ], style),
        None        => Layout::new(),
    };

    let mut sup = match scripts.superscript {
        Some(ref b) => layout(&mut [ *b.clone() ], style.superscript_variant()),
        None        => Layout::new(),
    };

    let mut sub = match scripts.subscript {
        Some(ref b) => layout(&mut [ *b.clone() ], style.subscript_variant()),
        None        => Layout::new(),
    };

    // We use a different algoirthm for handling scripts for operators with limits.
    // This is where he handle Operators with limits.
    if let Some(ref b) = scripts.base {
        if Some(AtomType::Operator(true)) == b.atom_type() {
            add_operator_limits(result, base, sup, sub, style);
            return;
        }
    }

    // We calculate the vertical positions of the scripts.  The `adjust_up`
    // variable will describe how far we need to adjust the superscript up.
    let mut adjust_up   = Pixels(0.0);
    let mut adjust_down = Pixels(0.0);
    let mut sup_kern    = Pixels(0.0);
    let mut sub_kern    = Pixels(0.0);

    if let Some(ref s) = scripts.superscript {
        // Use default font values for first iteration of vertical height.
        adjust_up = match style.is_cramped() {
            true  => SUPERSCRIPT_SHIFT_UP_CRAMPED,
            false => SUPERSCRIPT_SHIFT_UP,
        }.scaled(style);

        let mut height = base.height;

        // TODO: These checks should be recursive?
        if let Some(ref b) = scripts.base {
            // For accents, whose base is a simple symbol, we do not take
            // the accent into account while positioning the superscript.
            if let ParseNode::Accent(ref acc) = **b {
                if let Some(sym) = acc.nucleus.is_symbol() {
                    height = glyph_metrics(sym.unicode)
                        .height()
                        .scaled(style);
                }
            }

            // Apply italics correction is base is a symbol
            else if let Some(base_sym) = b.is_symbol() {
                let bg = glyph_metrics(base_sym.unicode);

                // Provided that the base is a operator, we only use
                // italics correction infomration.
                if let Some(AtomType::Operator(_)) = b.atom_type() {
                    // This recently changed in LuaTeX.  See `nolimitsmode`.
                    // This needs to be the glyph information _after_ layout for base.
                    //sup_kern =  0.5 * bg.italic_correction().scaled(style);
                    sub_kern =-1. * bg.italic_correction().scaled(style);
                }

                // Lookup font kerning of superscript is also a symbol
                else if let Some(sup_sym) = s.is_symbol() {
                    use font::kerning::superscript_kern;
                    let sg = glyph_metrics(sup_sym.unicode);
                    let kern = Unit::Font(superscript_kern(bg, sg,
                        *adjust_up / FONT_SIZE * *UNITS_PER_EM)).scaled(style);

                    sup_kern = bg.italic_correction().scaled(style) + kern;
                } else {
                    sup_kern = bg.italic_correction().scaled(style);
                }
            }
        }

        let drop_max = SUPERSCRIPT_BASELINE_DROP_MAX.scaled(style);
        adjust_up = adjust_up
            .max(height - drop_max)
            .max(SUPERSCRIPT_BOTTOM_MIN.scaled(style) - sup.depth);
    }

    // We calculate the vertical position of the subscripts.  The `adjust_down`
    // variable will describe how far we need to adjust the subscript down.
    if let Some(ref s) = scripts.subscript {
        // Use default font values for first iteration of vertical height.
        adjust_down = SUBSCRIPT_SHIFT_DOWN.scaled(style);

        let depth = -1. * base.depth;
        let drop_min = SUBSCRIPT_BASELINE_DROP_MIN.scaled(style);

        adjust_down = adjust_down
            .max(sub.height - SUBSCRIPT_TOP_MAX.scaled(style))
            .max(drop_min + depth);

        // Provided that the base and subscript are symbols, we apply
        // kerning values found in the kerning font table
        if let Some(ref b) = scripts.base {
            if let (Some(ssym), Some(bsym)) = (s.is_symbol(), b.is_symbol()) {
                use font::kerning::subscript_kern;
                let bg = glyph_metrics(bsym.unicode);
                let sg = glyph_metrics(ssym.unicode);

                sub_kern += Unit::Font(subscript_kern(bg, sg,
                    *adjust_down / FONT_SIZE * *UNITS_PER_EM)).scaled(style);
            }
        }
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
        if sup_kern != Pixels(0.0) {
            sup.contents.insert(0, kern!(horz: sup_kern));
            sup.width += sup_kern;
        }

        let corrected_adjust =
            adjust_up - sub.height + adjust_down;

        contents.add_node(sup.as_node());
        contents.add_node(kern!(vert: corrected_adjust));
    }

    contents.set_offset(adjust_down);
    if !sub.contents.is_empty() {
        if sub_kern != Pixels(0.0) {
            sub.contents.insert(0, kern!(horz: sub_kern));
            sub.width += sub_kern;
        }

        contents.add_node(sub.as_node());
    }

    result.add_node(base.as_node());
    result.add_node(contents.build());
}

fn add_operator_limits(result: &mut Layout, base: Layout,
        sup: Layout, sub: Layout, style: Style) {
    // Provided that the operator is a simple symbol, we need to account
    // for the italics correction of the symbol.  This how we "center"
    // the superscript and subscript of the limits.
    let delta = if let Some(gly) = base.is_symbol() {
        gly.italics
    } else {
        Pixels(0.0)
    };

    // Next we calculate the kerning required to separate the superscript
    // and subscript (respectively) from the base.
    let sup_kern = UPPER_LIMIT_BASELINE_RISE_MIN.scaled(style)
        .max(UPPER_LIMIT_GAP_MIN.scaled(style) - sup.depth);
    let sub_kern =
        (LOWER_LIMIT_BASELINE_DROP_MIN.scaled(style) - sub.height - base.depth)
        .max(LOWER_LIMIT_GAP_MIN.scaled(style) - base.depth);

    // We need to preserve the baseline of the operator when
    // attaching the scripts.  Since the base should already
    // be aligned, we only need to offset by the addition of
    // subscripts.
    let offset = sub.height + sub_kern;

    // We will construct a vbox containing the superscript/base/subscript.
    // We will all of these nodes, so we widen each to the largest.
    let width = base.width
        .max(sub.width + delta / 2.0)
        .max(sup.width + delta / 2.0);

    // My macro won't take `sup.width` in the alignment for some reason.
    // TODO: Fix that.
    let sup_width = sup.width;
    let sub_width = sub.width;

    result.add_node(vbox!(
        offset: offset;
        hbox![align: Alignment::Centered(sup_width);
            width: width;
            kern![horz: delta / 2.0],
            sup.as_node()
        ],

        kern!(vert: sup_kern),
        base.centered(width).as_node(),
        kern!(vert: sub_kern),

        hbox![align: Alignment::Centered(sub_width);
            width: width;
            kern![horz: -1. * delta / 2.0],
            sub.as_node()
        ]
    ));
}

fn add_frac(result: &mut Layout, frac: &mut GenFraction, style: Style) {
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
        shift_up = FRACTION_NUMERATOR_DISPLAY_STYLE_SHIFT_UP.scaled(style);
        shift_down = FRACTION_DENOMINATOR_DISPLAY_STYLE_SHIFT_DOWN.scaled(style);
        gap_num = FRACTION_NUM_DISPLAY_STYLE_GAP_MIN.scaled(style);
        gap_denom = FRACTION_DENOM_DISPLAY_STYLE_GAP_MIN.scaled(style);
    } else {
        shift_up = FRACTION_NUMERATOR_SHIFT_UP.scaled(style);
        shift_down = FRACTION_DENOMINATOR_SHIFT_DOWN.scaled(style);
        gap_num = FRACTION_NUMERATOR_GAP_MIN.scaled(style);
        gap_denom = FRACTION_DENOMINATOR_GAP_MIN.scaled(style);
    }

    let kern_up = (shift_up - axis - bar/2.0).max(gap_num - numer.depth);
    let kern_down = (shift_down + axis - denom.height - 0.5*bar).max(-1. * gap_denom);
    let offset = denom.height + kern_down + 0.5*bar - axis;

    let width  = numer.width.max(numer.width);
    let inner = vbox!(offset: offset;
        numer,
        kern!(vert: kern_up),
        rule!(width: width, height: bar),
        kern!(vert: kern_down),
        denom
    );

    result.add_node(kern!(horz: NULL_DELIMITER_SPACE));
    result.add_node(inner);
    result.add_node(kern!(horz: NULL_DELIMITER_SPACE));
}

fn add_atom_change(result: &mut Layout, at: AtomType, inner: &mut [ParseNode], style: Style) {
    // Atom Types can change control flow for operators.
    // We handle this change in control flow here,
    // otherwise we do nothing.

    // TODO: This adds an unnecessary hbox.  Remove them.
    if inner.len() != 1 {
        result.add_node(layout(inner, style).as_node());
        return;
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

fn add_radical(result: &mut Layout, rad: &mut Radical, style: Style) {
    //Reference rule 11 from pg 443 of TeXBook
    let contents = layout(&mut rad.inner, style.cramped()).as_node();
    let sqrt  = glyph_metrics(0x221A); // The sqrt symbol.

    let gap = match style >= Style::Display {
        true  => RADICAL_DISPLAY_STYLE_VERTICAL_GAP,
        false => RADICAL_VERTICAL_GAP,
    };

    let size = (*contents.height - *contents.depth)
        / FONT_SIZE * 1000.0     // Convert to font units
        + *gap
        + *RADICAL_EXTRA_ASCENDER; // Minimum gap

    let gap = gap.scaled(style);

    let rule_thickness = RADICAL_RULE_THICKNESS.scaled(style);
    let glyph = sqrt.vert_variant(size).as_layout(style);

    let inner_center = 0.5 * (gap + contents.height + contents.depth + rule_thickness);
    let sym_center   = 0.5 * (glyph.height + glyph.depth);
    let offset = sym_center - inner_center;

    let top_padding = RADICAL_EXTRA_ASCENDER.scaled(style)
        - RADICAL_RULE_THICKNESS.scaled(style);

    let kerning = (glyph.height - offset)
        - RADICAL_EXTRA_ASCENDER.scaled(style)
        - contents.height;

    result.add_node(vbox!(offset: offset; glyph));
    result.add_node(vbox!(
            kern!(vert: top_padding),
            rule!(
                width:  contents.width,
                height: rule_thickness),
            kern!(vert: kerning),
            contents
        ));
}

trait IsSymbol {
    fn is_symbol(&self) -> Option<LayoutGlyph>;
}

impl IsSymbol for Layout {
    fn is_symbol(&self) -> Option<LayoutGlyph> {
        if self.contents.len() != 1 { return None }
        let node = &self.contents[0];
        match node.node {
            LayoutVariant::Glyph(lg) => Some(lg),
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
            },
            _ => None,
        }
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