use super::{ LayoutNode, LayoutGlyph, Rule, HorizontalBox, VerticalBox, Style };

use dimensions::{ Pixels, Unit };
use font;
use font::constants::*;
use font::glyph_metrics;
use font::GLYPHS;
use font::SYMBOLS;
use font::variants::Variant;
use font::variants::VariantGlyph;
use layout::ToPixels;
use layout::boundingbox::Bounded;
use parser::nodes::{ ParseNode, AtomType };
use render::FONT_SIZE;
use spacing::atom_spacing;
use spacing::Spacing;


macro_rules! hbox {
    ($contents:expr) => (
        LayoutNode::HorizontalBox(HorizontalBox {
            contents: $contents,
            ..Default::default()
        })
    )
}

macro_rules! vbox {
    ($contents:expr) => (
        LayoutNode::VerticalBox(VerticalBox {
            contents: $contents,
            ..Default::default()
        })
    );
    ($contents:expr, offset: $offset:expr) => (
        LayoutNode::VerticalBox(VerticalBox {
            contents: $contents,
            offset:   $offset,
            ..Default::default()
        })
    )
}


/// This method takes the parsing nodes and reduces them to layout nodes.
#[allow(unconditional_recursion)]
#[allow(dead_code)]
pub fn reduce(nodes: &mut [ParseNode], style: Style) -> Vec<LayoutNode> {
    // Rule (5), pg 442.  If first item is a Bin atom, change it
    // to an Ordinal item.
    use font::IsAtom;
    if let Some(mut node) = nodes.get_mut(0) {
        if node.atom_type() == Some(AtomType::Binary) {
            node.set_atom_type(AtomType::Ordinal)
        }
    }

    // Atom Changing Rules:
    //   Rule 5:
    //   - Current == Bin && Prev in {Bin,Op,Rel,Open,Punct}, Current -> Ord.
    //   Rule 6:
    //   - Current in {Rel,Close,Punct} && Prev == Bin => Prev -> Ord.
    for idx in 0..nodes.len() {
        if nodes[idx].atom_type() == Some(AtomType::Binary)
            && idx > 1 {
            match nodes[idx - 1].atom_type() {
                Some(AtomType::Binary) |
                Some(AtomType::Operator(_)) |
                Some(AtomType::Relation) |
                Some(AtomType::Open) |
                Some(AtomType::Punctuation) => {
                    nodes[idx].set_atom_type(AtomType::Alpha);
                    println!("Chaing binary to ordinal {:?}", nodes[idx]);
                },
                _ => (),
            }
        }

        if idx > 1
            && nodes[idx - 1].atom_type() == Some(AtomType::Binary) {
            match nodes[idx].atom_type() {
                Some(AtomType::Relation) |
                Some(AtomType::Close) |
                Some(AtomType::Punctuation) =>
                    nodes[idx - 1].set_atom_type(AtomType::Alpha),
                _ => (),
            }
        }
    }

    let mut prev_at: Option<AtomType> = None;
    let mut layout: Vec<LayoutNode> = Vec::with_capacity(nodes.len());
    for node in nodes {
        if let Some(p_at) = prev_at {
            if let Some(at) = node.atom_type() {
                let sp = atom_spacing(p_at, at);
                if sp != Spacing::None {
                    let kern = sp.to_unit().scaled_pixels(FONT_SIZE, style);
                    layout.push(LayoutNode::Kern(kern));
                }
            }
        }

        // TODO: May need to ignore this if transparent atom_type.
        prev_at = node.atom_type();

        match *node {
            ParseNode::Symbol(sym) => {
                use parser::nodes::AtomType;

                let glyph = font::glyph_metrics(sym.unicode);
                if let AtomType::Operator(_) = sym.atom_type {
                    // TODO: Only display style for now.  Change this.
                    // TODO: This should probably use `min op hieght` param.
                    let l_glyph = glyph.successor().into_layout_node(style);
                    let axis_offset = AXIS_HEIGHT
                        .scaled_pixels(FONT_SIZE, style);
                    let shift_down = 0.5 * ( l_glyph.get_height() + l_glyph.get_depth() ) - axis_offset;
                    layout.push(vbox!(vec![l_glyph], offset: shift_down));
                } else {
                    let glyph = font::glyph_metrics(sym.unicode);
                    layout.push(glyph.into_layout_node(style));
                }
            },

            ParseNode::Group(ref mut gp) =>
                layout.push(hbox!(reduce(&mut gp.clone(), style))),

            ParseNode::Rule(rule) =>
                layout.push(LayoutNode::Rule(Rule {
                    width:  rule.width .scaled_pixels(FONT_SIZE, style),
                    height: rule.height.scaled_pixels(FONT_SIZE, style),
                    depth:  Pixels(0f64),
                })),

            ParseNode::Kerning(kern) =>
                layout.push(LayoutNode::Kern(kern.scaled_pixels(FONT_SIZE, style))),

            ParseNode::Spacing(sp) => {
                let kern = sp.to_unit().scaled_pixels(FONT_SIZE, style);
                layout.push(LayoutNode::Kern(kern));
            }

            ParseNode::Radical(ref rad) => {
                //Reference rule 11 from pg 443 of TeXBook
                let style = style.cramped_variant();
                let contents = hbox!(reduce(&mut rad.inner.clone(), style));

                let sqrt  = &GLYPHS[&SYMBOLS["sqrt"].unicode];

                let gap = match style.cramped() {
                    true  => RADICAL_VERTICAL_GAP,
                    false => RADICAL_DISPLAY_STYLE_VERTICAL_GAP,
                };

                let clearance = (*contents.get_height() - *contents.get_depth())
                    / FONT_SIZE * 1000.0     // Convert to font units
                    + f64::from(gap)
                    + f64::from(RADICAL_RULE_THICKNESS)
                    + f64::from(RADICAL_EXTRA_ASCENDER); // Minimum gap

                println!("Clearance: {}", clearance);

                let glyph = sqrt.variant(clearance).into_layout_node(style);
                let offset = -1.0 * contents.get_depth();
                let ascender = RADICAL_EXTRA_ASCENDER.scaled_pixels(FONT_SIZE, style);
                let rule = RADICAL_RULE_THICKNESS.scaled_pixels(FONT_SIZE, style);
                let kerning = glyph.get_height()
                    - contents.get_height()
                    - rule
                    - ascender
                    - offset;

                layout.push(vbox!(vec![hbox![vec![glyph]]], offset: offset));
                layout.push(vbox!(vec![
                        LayoutNode::Kern(ascender),
                        LayoutNode::Rule(Rule {
                            width:  contents.get_width(),
                            height: rule,
                            depth:  Pixels(0f64),
                        }),
                        LayoutNode::Kern(kerning),
                        contents,
                    ]));
            },

            ParseNode::Scripts(ref scripts) => {
                // Vertical position of the script is calculated.  This depends on the following
                // information: (see page 26 of https://www.tug.org/tugboat/tb30-1/tb94vieth.pdf)

                // Before we start calculating layout, we layout the contents
                // of the base and scripts.  These dimensions will be needed.
                let base = match scripts.base {
                    Some(ref b) => reduce(&mut [ *b.clone() ], style),
                    None    => vec![],
                };

                // We calculate the vertical positions of the scripts.  The `adjust_up`
                // variable will describe how far we need to adjust the superscript up.
                let mut italics_correction = Pixels(0.0);
                let adjust_up = if let Some(_) = scripts.superscript {
                    // We start with default values provided from the font.  These are called
                    // the standard positions in the OpenType specification.
                    let mut default = match style.cramped() {
                        true  => SUPERSCRIPT_SHIFT_UP_CRAMPED,
                        false => SUPERSCRIPT_SHIFT_UP,
                    }.scaled_pixels(FONT_SIZE, style);

                    // Next we check to see if the vertical shift meets the minimum
                    // clearance relative to the base.
                    let height   = base.get_height();
                    let drop_max = SUPERSCRIPT_BASELINE_DROP_MAX
                        .scaled_pixels(FONT_SIZE, style);

                    if height - default > drop_max {
                        default = height - drop_max;
                    }

                    // For superscripts we need to calculate the italics correction
                    // if the base is simply a symbol.
                    // TODO: This can probably be cleaned up a bit.
                    if let Some(ref bx) = scripts.base {
                        if let ParseNode::Symbol(sym) = **bx {
                            let glyph = glyph_metrics(sym.unicode);
                            italics_correction = Unit::Font(glyph.italics as f64)
                                .scaled_pixels(FONT_SIZE, style)
                        }
                    }

                    default
                } else { Pixels(0.0) };

                // We calculate the vertical position of the subscripts.  The `adjust_down`
                // variable will describe how far we need to adjust the subscript down.
                let adjust_down = if let Some(_) = scripts.subscript {
                    // We start with the default values provided from the font.
                    let mut default = SUBSCRIPT_SHIFT_DOWN.scaled_pixels(FONT_SIZE, style);

                    let depth = -1. * base.get_depth();
                    let drop_min = SUBSCRIPT_BASELINE_DROP_MIN
                        .scaled_pixels(FONT_SIZE, style);

                    if default - depth < drop_min {
                        default = drop_min + depth;
                    }

                    default
                } else { Pixels(0.0) };

                // TODO: Also check for collisions.
                // Next we check that the superscript isn't too far down
                // and that the subscript isn't too far up.
                //     - CONSTNATS:
                //         - sub_superscript_gap_min: 264,
                //         - subscript_top_max: 400,
                //         - superscript_bottom_max_with_subscript: 400,
                //         - superscript_bottom_min: 125,

                // Now they we've calculated the drop down/up,
                // we need to calculate the spacing between the
                // subscript and superscript.

                let mut contents = vec![];

                if let Some(ref sub) = scripts.subscript {
                    let style  = style.subscript_variant();
                    let script = reduce(&mut [ *sub.clone() ], style);

                    contents.push(hbox!(script));
                }

                if let Some(ref s) = scripts.superscript {
                    let style = style.superscript_variant();
                    let mut script = reduce(&mut [ *s.clone() ], style);

                    if italics_correction != Pixels(0.0) {
                        script.insert(0, LayoutNode::Kern(italics_correction));
                    }

                    let corrected_adjust =
                        adjust_up - contents.get_height() + adjust_down;

                    contents.insert(0, LayoutNode::Kern(corrected_adjust));
                    contents.insert(0,
                        hbox!(vec![
                            LayoutNode::Kern(italics_correction),
                            hbox!(script)
                        ]));
                }

                layout.push(hbox!(base));
                layout.push(vbox!(contents, offset: adjust_down));
            },

            ParseNode::Extend(code, u) => {
                let paren = glyph_metrics(code); // Left parantheses

                match paren.variant(*u.as_pixels(FONT_SIZE)) {
                    VariantGlyph::Replacement(g) => {
                        let glyph = font::glyph_metrics(g.unicode);
                        layout.push(glyph.into_layout_node(style));
                    },
                    VariantGlyph::Constructable(c) => {
                        let mut contents: Vec<LayoutNode> = Vec::new();
                        for instr in c.iter().rev() {
                            contents.push(instr.glyph.into_layout_node(style));
                            if instr.overlap != 0.0 {
                                let unit = Unit::Font(-instr.overlap);
                                let kern = unit
                                    .scaled_pixels(FONT_SIZE, style);
                                contents.push(LayoutNode::Kern(kern));
                            }
                        }
                        layout.push(vbox!(contents));
                    },
                }
            },

            _ => (),
       }
    }

    layout
}

trait IntoLayoutNode {
    fn into_layout_node(&self, sty: Style) -> LayoutNode;
}

impl IntoLayoutNode for font::Glyph {
    fn into_layout_node(&self, style: Style) -> LayoutNode {
        LayoutNode::Glyph(LayoutGlyph {
            scale:   style.font_scale(),
            height:  self.height() .scaled_pixels(FONT_SIZE, style),
            depth:   self.depth()  .scaled_pixels(FONT_SIZE, style),
            advance: self.advance().scaled_pixels(FONT_SIZE, style),
            unicode: self.unicode,
        })
    }
}

impl IntoLayoutNode for VariantGlyph {
    fn into_layout_node(&self, style: Style) -> LayoutNode {
        match *self {
            VariantGlyph::Replacement(g) => {
                let glyph = font::glyph_metrics(g.unicode);
                glyph.into_layout_node(style)
            },

            VariantGlyph::Constructable(ref c) => {
                let mut contents: Vec<LayoutNode> = Vec::new();
                for instr in c.iter().rev() {
                    contents.push(instr.glyph.into_layout_node(style));
                    if instr.overlap != 0.0 {
                        let unit = Unit::Font(-instr.overlap);
                        let kern = unit
                            .scaled_pixels(FONT_SIZE, style);
                        contents.push(LayoutNode::Kern(kern));
                    }
                }

                vbox!(contents)
            },
        }
    }
}