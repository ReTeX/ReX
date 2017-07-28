#![allow(unused_assignments)]
#![allow(unused_variables)]

use std::cmp::{min, max};

use super::Alignment;
use super::builders;
use super::{Layout, LayoutNode, LayoutVariant, LayoutGlyph, Style, ColorChange};
use super::convert::AsLayoutNode;
use super::LayoutSettings;

use font;
use font::constants::*;
use font::glyph_metrics;
use font::variant::Variant;
use font::{AtomType, Symbol, VariantGlyph, FontUnit};
use font::kerning::{superscript_kern, subscript_kern};
use layout::spacing::{atom_spacing, Spacing};
use layout::convert::Scaled;
use parser::nodes::BarThickness;
use parser::nodes::{ParseNode, AtomChange, Accent, Delimited, GenFraction, Radical, Scripts, Stack};

/// Entry point to our recursive algorithm
pub fn layout(nodes: &[ParseNode], config: LayoutSettings) -> Layout {
    layout_recurse(nodes, config, AtomType::Transparent)
}

/// This method takes the parsing nodes and layouts them to layout nodes.
#[allow(unconditional_recursion)]
fn layout_recurse(nodes: &[ParseNode],
                  mut config: LayoutSettings,
                  parent_next: AtomType)
                  -> Layout {

    let mut result = Layout::new();
    let mut prev = AtomType::Transparent;

    for idx in 0..nodes.len() {
        let node = &nodes[idx];

        let next = if idx + 1 < nodes.len() {
            nodes[idx + 1].atom_type()
        } else {
            parent_next
        };

        let mut current = node.atom_type();
        if current == AtomType::Binary {
            if prev == AtomType::Transparent || prev == AtomType::Binary ||
               prev == AtomType::Relation || prev == AtomType::Open ||
               prev == AtomType::Punctuation {
                current = AtomType::Alpha;
            } else if let AtomType::Operator(_) = prev {
                current = AtomType::Alpha;
            } else if next == AtomType::Relation || next == AtomType::Close ||
                      next == AtomType::Punctuation {
                current = AtomType::Alpha;
            }
        }

        let sp = atom_spacing(prev, current, config.style);
        if sp != Spacing::None {
            let kern = sp.to_unit().scaled(config);
            result.add_node(kern!(horz: kern));
        }

        prev = current;

        match *node {
            ParseNode::Symbol(sym) => add_symbol(&mut result, sym, config),
            ParseNode::Scripts(ref scripts) => add_scripts(&mut result, scripts, config),
            ParseNode::Radical(ref rad) => add_radical(&mut result, rad, config),
            ParseNode::Delimited(ref delim) => add_delimited(&mut result, delim, config),
            ParseNode::Accent(ref acc) => add_accent(&mut result, acc, config),
            ParseNode::GenFraction(ref frac) => add_frac(&mut result, frac, config),
            ParseNode::Stack(ref stack) => add_substack(&mut result, stack, config),
            ParseNode::Group(ref gp) => result.add_node(layout(gp, config).as_node()),
            ParseNode::Rule(rule) => result.add_node(rule.as_layout(config)),
            ParseNode::Kerning(kern) => result.add_node(kern!(horz: kern.scaled(config))),
            ParseNode::Style(sty) => config.style = sty,

            ParseNode::Color(ref clr) => {
                let layout = layout_recurse(&clr.inner, config, next);

                result.add_node(LayoutNode {
                                    width: layout.width,
                                    height: layout.height,
                                    depth: layout.depth,
                                    node: LayoutVariant::Color(ColorChange {
                                                                   color: clr.color,
                                                                   inner: layout.contents,
                                                               }),
                                })
            }

            ParseNode::AtomChange(AtomChange { at, ref inner }) => {
                add_atom_change(&mut result, at, inner, config)
            }

            _ => println!("Warning: Ignored ParseNode: {:?}", node),
        }
    }

    result.finalize()
}

fn add_symbol(result: &mut Layout, sym: Symbol, config: LayoutSettings) {
    // Operators are handled specially.  We may need to find a larger
    // symbol and vertical center it.
    if let AtomType::Operator(_) = sym.atom_type {
        add_largeop(result, sym, config);
    } else {
        let glyph = font::glyph_metrics(sym.unicode);
        result.add_node(glyph.as_layout(config));
    }
}

fn add_largeop(result: &mut Layout, sym: Symbol, config: LayoutSettings) {
    let glyph = font::glyph_metrics(sym.unicode);

    if config.style > Style::Text {
        let axis_offset = AXIS_HEIGHT.scaled(config);

        let largeop = glyph
            .vert_variant(DISPLAY_OPERATOR_MIN_HEIGHT)
            .as_layout(config);
        let shift = (largeop.height + largeop.depth) / 2 - axis_offset;

        result.add_node(vbox!(offset: shift; largeop));
    } else {
        result.add_node(glyph.as_layout(config));
    }
}

fn add_accent(result: &mut Layout, acc: &Accent, config: LayoutSettings) {
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

    let base = layout(&[*acc.nucleus.clone()], config.cramped());
    let accent_variant = glyph_metrics(acc.symbol.unicode).horz_variant(base.width);
    let accent = accent_variant.as_layout(config);

    // Attachment points for accent & base are calculated by
    //   (a) Non-symbol: width / 2.0,
    //   (b) Symbol:
    //      1. Attachment point (if there is one)
    //      2. Otherwise: (width + ic) / 2.0
    let base_offset = if base.contents.len() != 1 {
        base.width / 2
    } else if let Some(ref sym) = base.contents[0].is_symbol() {
        let glyph = glyph_metrics(sym.unicode);
        if glyph.attachment != FontUnit::from(0) {
            glyph.attachment.scaled(config)
        } else {
            let offset: FontUnit = (glyph.advance + glyph.italics) / 2;
            offset.scaled(config)
        }
    } else {
        base.width / 2
    };

    let acc_offset = match accent_variant {
        VariantGlyph::Replacement(sym) => {
            let glyph = glyph_metrics(sym.unicode);
            if glyph.attachment != FontUnit::from(0) {
                glyph.attachment.scaled(config)
            } else {
                // For glyphs without attachmens, we must
                // also account for combining glyphs
                let offset: FontUnit = (sym.bbox.2 + sym.bbox.0) / 2;
                offset.scaled(config)
            }
        }

        VariantGlyph::Constructable(_, _) => accent.width / 2,
    };

    // Do not place the accent any _further_ than you would if given
    // an `x` character in the current style.
    let delta = -min(base.height, ACCENT_BASE_HEIGHT.scaled(config));

    // By not placing an offset on this vbox, we are assured that the
    // baseline will match the baseline of `base.as_node()`
    result.add_node(vbox!(hbox!(kern!(horz: base_offset - acc_offset), accent),
                          kern!(vert: delta),
                          base.as_node()));
}

fn add_delimited(result: &mut Layout, delim: &Delimited, config: LayoutSettings) {
    let inner = layout(&delim.inner, config).as_node();

    // Convert inner group dimensions to font unit
    let height = inner.height;
    let depth = inner.depth;

    // Only extend if we meet a certain size
    // TODO: This quick height check doesn't seem to be strong enough,
    // reference: http://tug.org/pipermail/luatex/2010-July/001745.html
    if max(height, -depth) > DELIMITED_SUB_FORMULA_MIN_HEIGHT / 2 {
        let axis = AXIS_HEIGHT;

        let mut clearance = 2 * max(height - axis, axis - depth);
        clearance = max(DELIMITER_FACTOR * clearance,
                        height - depth - DELIMITER_SHORT_FALL);

        let axis = AXIS_HEIGHT.scaled(config);
        let left = match delim.left.unicode {
            46 => kern!(horz: NULL_DELIMITER_SPACE),
            _ => {
                glyph_metrics(delim.left.unicode)
                    .vert_variant(clearance)
                    .as_layout(config)
                    .centered(axis)
            }
        };

        let right = match delim.right.unicode {
            46 => kern!(horz: NULL_DELIMITER_SPACE),
            _ => {
                glyph_metrics(delim.right.unicode)
                    .vert_variant(clearance)
                    .as_layout(config)
                    .centered(axis)
            }
        };

        result.add_node(left);
        result.add_node(inner);
        result.add_node(right);
    } else {
        let left = match delim.left.unicode {
            46 => kern!(horz: NULL_DELIMITER_SPACE),
            _ => glyph_metrics(delim.left.unicode).as_layout(config),
        };

        let right = match delim.right.unicode {
            46 => kern!(horz: NULL_DELIMITER_SPACE),
            _ => glyph_metrics(delim.right.unicode).as_layout(config),
        };

        result.add_node(left);
        result.add_node(inner);
        result.add_node(right);
    }
}

fn add_scripts(result: &mut Layout, scripts: &Scripts, config: LayoutSettings) {
    // See: https://tug.org/TUGboat/tb27-1/tb86jackowski.pdf
    //      https://www.tug.org/tugboat/tb30-1/tb94vieth.pdf

    let base = match scripts.base {
        Some(ref b) => layout(&[*b.clone()], config),
        None => Layout::new(),
    };

    let mut sup = match scripts.superscript {
        Some(ref b) => layout(&[*b.clone()], config.superscript_variant()),
        None => Layout::new(),
    };

    let mut sub = match scripts.subscript {
        Some(ref b) => layout(&[*b.clone()], config.subscript_variant()),
        None => Layout::new(),
    };

    // We use a different algoirthm for handling scripts for operators with limits.
    // This is where he handle Operators with limits.
    if let Some(ref b) = scripts.base {
        if AtomType::Operator(true) == b.atom_type() {
            add_operator_limits(result, base, sup, sub, config);
            return;
        }
    }

    // We calculate the vertical positions of the scripts.  The `adjust_up`
    // variable will describe how far we need to adjust the superscript up.
    let mut adjust_up = FontUnit::from(0);
    let mut adjust_down = FontUnit::from(0);
    let mut sup_kern = FontUnit::from(0);
    let mut sub_kern = FontUnit::from(0);

    if let Some(ref s) = scripts.superscript {
        // Use default font values for first iteration of vertical height.
        adjust_up = match config.style.is_cramped() {
                true => SUPERSCRIPT_SHIFT_UP_CRAMPED,
                false => SUPERSCRIPT_SHIFT_UP,
            }
            .scaled(config);

        let mut height = base.height;

        // TODO: These checks should be recursive?
        if let Some(ref b) = scripts.base {
            if b.atom_type() != AtomType::Operator(false) {
                // For accents, whose base is a simple symbol, we do not take
                // the accent into account while positioning the superscript.
                if let ParseNode::Accent(ref acc) = **b {
                    if let Some(sym) = acc.nucleus.is_symbol() {
                        height = glyph_metrics(sym.unicode).height().scaled(config);
                    }
                }
                // Apply italics correction is base is a symbol
                else if let Some(base_sym) = base.is_symbol() {
                    // Lookup font kerning of superscript is also a symbol
                    if let Some(sup_sym) = sup.is_symbol() {
                        let bg = glyph_metrics(base_sym.unicode);
                        let sg = glyph_metrics(sup_sym.unicode);

                        let kern = superscript_kern(bg, sg, adjust_up).scaled(config);

                        sup_kern = base_sym.italics + kern;
                    } else {
                        sup_kern = base_sym.italics;
                    }
                }
            }
        }

        let drop_max = SUPERSCRIPT_BASELINE_DROP_MAX.scaled(config);
        adjust_up = max!(adjust_up,
                         height - drop_max,
                         SUPERSCRIPT_BOTTOM_MIN.scaled(config) - sup.depth);
    }

    // We calculate the vertical position of the subscripts.  The `adjust_down`
    // variable will describe how far we need to adjust the subscript down.
    if let Some(ref s) = scripts.subscript {
        // Use default font values for first iteration of vertical height.
        adjust_down = SUBSCRIPT_SHIFT_DOWN.scaled(config);

        let depth = -base.depth;
        let drop_min = SUBSCRIPT_BASELINE_DROP_MIN.scaled(config);

        adjust_down = max!(adjust_down,
                           sub.height - SUBSCRIPT_TOP_MAX.scaled(config),
                           drop_min + depth);

        // Provided that the base and subscript are symbols, we apply
        // kerning values found in the kerning font table
        if let Some(ref b) = scripts.base {
            if let Some(base_sym) = base.is_symbol() {
                if AtomType::Operator(false) == b.atom_type() {
                    // This recently changed in LuaTeX.  See `nolimitsmode`.
                    // This needs to be the glyph information _after_ layout for base.
                    sub_kern = -glyph_metrics(base_sym.unicode)
                                    .italics
                                    .scaled(config);
                }
            }

            if let (Some(ssym), Some(bsym)) = (sub.is_symbol(), base.is_symbol()) {
                let bg = glyph_metrics(bsym.unicode);
                let sg = glyph_metrics(ssym.unicode);

                sub_kern += subscript_kern(bg, sg, adjust_down).scaled(config);
            }
        }
    }

    // TODO: lazy gap fix; see BottomMaxWithSubscript
    if !sub.contents.is_empty() && !sup.contents.is_empty() {
        let sup_bot = adjust_up + sup.depth;
        let sub_top = sub.height - adjust_down;
        let gap_min = SUB_SUPERSCRIPT_GAP_MIN.scaled(config);
        if sup_bot - sub_top < gap_min {
            let adjust = (gap_min - sup_bot + sub_top) / 2;
            adjust_up += adjust;
            adjust_down += adjust;
        }
    }

    let mut contents = builders::VBox::new();
    if !sup.contents.is_empty() {
        if sup_kern != FontUnit::from(0) {
            sup.contents.insert(0, kern!(horz: sup_kern));
            sup.width += sup_kern;
        }

        let corrected_adjust = adjust_up - sub.height + adjust_down;

        contents.add_node(sup.as_node());
        contents.add_node(kern!(vert: corrected_adjust));
    }

    contents.set_offset(adjust_down);
    if !sub.contents.is_empty() {
        if sub_kern != FontUnit::from(0) {
            sub.contents.insert(0, kern!(horz: sub_kern));
            sub.width += sub_kern;
        }

        contents.add_node(sub.as_node());
    }

    result.add_node(base.as_node());
    result.add_node(contents.build());
}

fn add_operator_limits(result: &mut Layout,
                       base: Layout,
                       sup: Layout,
                       sub: Layout,
                       config: LayoutSettings) {
    // Provided that the operator is a simple symbol, we need to account
    // for the italics correction of the symbol.  This how we "center"
    // the superscript and subscript of the limits.
    let delta = if let Some(gly) = base.is_symbol() {
        gly.italics
    } else {
        FontUnit::from(0)
    };

    // Next we calculate the kerning required to separate the superscript
    // and subscript (respectively) from the base.
    let sup_kern = max(UPPER_LIMIT_BASELINE_RISE_MIN.scaled(config),
                       UPPER_LIMIT_GAP_MIN.scaled(config) - sup.depth);
    let sub_kern = max(LOWER_LIMIT_GAP_MIN.scaled(config),
                       LOWER_LIMIT_BASELINE_DROP_MIN.scaled(config) - sub.height) -
                   base.depth;

    // We need to preserve the baseline of the operator when
    // attaching the scripts.  Since the base should already
    // be aligned, we only need to offset by the addition of
    // subscripts.
    let offset = sub.height + sub_kern;

    // We will construct a vbox containing the superscript/base/subscript.
    // We will all of these nodes, so we widen each to the largest.
    let width = max!(base.width, sub.width + delta / 2, sup.width + delta / 2);

    // My macro won't take `sup.width` in the alignment for some reason.
    // TODO: Fix that.
    let sup_width = sup.width;
    let sub_width = sub.width;

    result.add_node(vbox!(
        offset: offset;
        hbox![align: Alignment::Centered(sup_width);
            width: width;
            kern![horz: delta / 2],
            sup.as_node()
        ],

        kern!(vert: sup_kern),
        base.centered(width).as_node(),
        kern!(vert: sub_kern),

        hbox![align: Alignment::Centered(sub_width);
            width: width;
            kern![horz: -delta / 2],
            sub.as_node()
        ]
    ));
}

fn add_frac(result: &mut Layout, frac: &GenFraction, config: LayoutSettings) {
    let bar = match frac.bar_thickness {
        BarThickness::Default => FRACTION_RULE_THICKNESS.scaled(config),
        BarThickness::None => FontUnit::from(0),
        BarThickness::Unit(u) => u.scaled(config),
    };

    let mut n = layout(&frac.numerator, config.numerator());
    let mut d = layout(&frac.denominator, config.denominator());

    if n.width > d.width {
        d.alignment = Alignment::Centered(d.width);
        d.width = n.width;
    } else {
        n.alignment = Alignment::Centered(n.width);
        n.width = d.width;
    }

    let numer = n.as_node();
    let denom = d.as_node();

    let mut shift_up = FontUnit::from(0);
    let mut shift_down = FontUnit::from(0);
    let mut gap_num = FontUnit::from(0);
    let mut gap_denom = FontUnit::from(0);
    let axis = AXIS_HEIGHT.scaled(config);

    if config.style > Style::Text {
        shift_up = FRACTION_NUMERATOR_DISPLAY_STYLE_SHIFT_UP.scaled(config);
        shift_down = FRACTION_DENOMINATOR_DISPLAY_STYLE_SHIFT_DOWN.scaled(config);
        gap_num = FRACTION_NUM_DISPLAY_STYLE_GAP_MIN.scaled(config);
        gap_denom = FRACTION_DENOM_DISPLAY_STYLE_GAP_MIN.scaled(config);
    } else {
        shift_up = FRACTION_NUMERATOR_SHIFT_UP.scaled(config);
        shift_down = FRACTION_DENOMINATOR_SHIFT_DOWN.scaled(config);
        gap_num = FRACTION_NUMERATOR_GAP_MIN.scaled(config);
        gap_denom = FRACTION_DENOMINATOR_GAP_MIN.scaled(config);
    }

    let kern_num = max(shift_up - axis - bar / 2, gap_num - numer.depth);
    let kern_den = max(shift_down + axis - denom.height - bar / 2, gap_denom);
    let offset = denom.height + kern_den + bar / 2 - axis;

    let width = numer.width;
    let inner = vbox!(offset: offset;
        numer,
        kern!(vert: kern_num),
        rule!(width: width, height: bar),
        kern!(vert: kern_den),
        denom
    );

    result.add_node(kern!(horz: NULL_DELIMITER_SPACE));
    result.add_node(inner);
    result.add_node(kern!(horz: NULL_DELIMITER_SPACE));
}

fn add_atom_change(result: &mut Layout,
                   at: AtomType,
                   inner: &[ParseNode],
                   config: LayoutSettings) {
    // Atom Types can change control flow for operators.
    // We handle this change in control flow here,
    // otherwise we do nothing.

    // TODO: This adds an unnecessary hbox.  Remove them.
    if inner.len() != 1 {
        result.add_node(layout(inner, config).as_node());
        return;
    }

    match at {
        AtomType::Operator(_) => {
            // if let Some(sym) = inner[0].is_symbol() {
            //     inner[0].set_atom_type(at);
            // }
        }
        _ => (),
    }

    result.add_node(layout(inner, config).as_node());
}

fn add_radical(result: &mut Layout, rad: &Radical, config: LayoutSettings) {
    //Reference rule 11 from pg 443 of TeXBook
    let contents = layout(&rad.inner, config.cramped()).as_node();
    let sqrt = glyph_metrics(0x221A); // The sqrt symbol.

    let gap = match config.style >= Style::Display {
        true => RADICAL_DISPLAY_STYLE_VERTICAL_GAP,
        false => RADICAL_VERTICAL_GAP,
    };

    let size = (contents.height - contents.depth) + gap + RADICAL_EXTRA_ASCENDER; // Minimum gap

    let gap = gap.scaled(config);

    let rule_thickness = RADICAL_RULE_THICKNESS.scaled(config);
    let glyph = sqrt.vert_variant(size).as_layout(config);

    let inner_center = (gap + contents.height + contents.depth + rule_thickness) / 2;
    let sym_center = (glyph.height + glyph.depth) / 2;
    let offset = sym_center - inner_center;

    let top_padding = RADICAL_EXTRA_ASCENDER.scaled(config) - RADICAL_RULE_THICKNESS.scaled(config);

    let kerning = (glyph.height - offset) - RADICAL_EXTRA_ASCENDER.scaled(config) - contents.height;

    result.add_node(vbox!(offset: offset; glyph));
    result.add_node(vbox!(kern!(vert: top_padding),
                          rule!(
                width:  contents.width,
                height: rule_thickness),
                          kern!(vert: kerning),
                          contents));
}

fn add_substack(result: &mut Layout, stack: &Stack, config: LayoutSettings) {
    let mut lines: Vec<Layout> = Vec::with_capacity(stack.lines.len());
    for line in &stack.lines {
        lines.push(layout(line, config));
    }

    // The line gap will be taken from STACK_GAP constants
    let gap_min = if config.style > Style::Text {
            STACK_DISPLAY_STYLE_GAP_MIN
        } else {
            STACK_GAP_MIN
        }
        .scaled(config);

    let gap_try = if config.style > Style::Text {
            STACK_TOP_DISPLAY_STYLE_SHIFT_UP - AXIS_HEIGHT + STACK_BOTTOM_SHIFT_DOWN -
            ACCENT_BASE_HEIGHT - ACCENT_BASE_HEIGHT
        } else {
            STACK_TOP_SHIFT_UP         //   480
            - AXIS_HEIGHT              // - 250
            + STACK_BOTTOM_SHIFT_DOWN  // + 800
            - ACCENT_BASE_HEIGHT - ACCENT_BASE_HEIGHT // - 900
        }
        .scaled(config);

    let mut stak = builders::VBox::new();

    let rest = lines.split_off(1);

    if lines.is_empty() {
        return;
    }
    stak.add_node(lines.pop().unwrap().as_node());

    let mut prev = FontUnit::from(0);
    for line in rest {
        let gap = max(gap_min, gap_try - prev);
        prev = line.depth;

        stak.add_node(kern![vert: gap]);
        stak.add_node(line.as_node());
    }

    // center the stack
    let offset = (stak.height + stak.depth) / 2 - AXIS_HEIGHT.scaled(config);

    stak.set_offset(offset);
    result.add_node(stak.build());
}

trait IsSymbol {
    fn is_symbol(&self) -> Option<LayoutGlyph>;
}

impl IsSymbol for Layout {
    fn is_symbol(&self) -> Option<LayoutGlyph> {
        if self.contents.len() != 1 {
            return None;
        }
        let node = &self.contents[0];
        match node.node {
            LayoutVariant::Glyph(lg) => Some(lg),
            LayoutVariant::HorizontalBox(ref hb) => {
                if hb.contents.len() != 1 {
                    None
                } else {
                    hb.contents[0].is_symbol()
                }
            }
            LayoutVariant::VerticalBox(ref vb) => {
                if vb.contents.len() != 1 {
                    None
                } else {
                    vb.contents[0].is_symbol()
                }
            }
            LayoutVariant::Color(ref clr) => {
                if clr.inner.len() != 1 {
                    return None;
                }
                clr.inner[0].is_symbol()
            }
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
            }
            LayoutVariant::VerticalBox(ref vb) => {
                if vb.contents.len() != 1 {
                    None
                } else {
                    vb.contents[0].is_symbol()
                }
            }
            LayoutVariant::Color(ref clr) => {
                if clr.inner.len() != 1 {
                    return None;
                }
                clr.inner[0].is_symbol()
            }
            _ => None,
        }
    }
}
