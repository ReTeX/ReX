use super::{ LayoutNode, LayoutGlyph, Rule, HorizontalBox, VerticalBox, Style };

use dimensions::{ Pixels, Unit };
use font;
use font::CONSTANTS;
use font::glyph_metrics;
use font::GLYPHS;
use font::SYMBOLS;
use font::variants::Variant;
use font::variants::VariantGlyph;
use layout::{ ToPixels, Scalable };
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
                    let kern = sp.to_unit().as_pixels(FONT_SIZE).with_scale(style);
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
                    let axis_offset = Unit::Font(CONSTANTS.axis_height as f64)
                        .as_pixels(FONT_SIZE).with_scale(style);
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
                    width:  rule.width .as_pixels(FONT_SIZE).with_scale(style),
                    height: rule.height.as_pixels(FONT_SIZE).with_scale(style),
                    depth:  Pixels(0f64),
                })),

            ParseNode::Kerning(kern) =>
                layout.push(LayoutNode::Kern(kern.as_pixels(FONT_SIZE).with_scale(style))),

            ParseNode::Spacing(sp) => {
                let kern = sp.to_unit().as_pixels(FONT_SIZE).with_scale(style);
                layout.push(LayoutNode::Kern(kern));
            }

            ParseNode::Radical(ref rad) => {
                //Reference rule 11 from pg 443 of TeXBook
                let style = style.cramped_variant();
                let glyph = &GLYPHS[&SYMBOLS["sqrt"].unicode]; // TODO: variants?
                layout.push(glyph.into_layout_node(style));

                let contents = hbox!(reduce(&mut rad.inner.clone(), style));

                let rule_thickness = Unit::Font(CONSTANTS.radical_rule_thickness as f64)
                    .as_pixels(FONT_SIZE).with_scale(style);
                let extra_ascender = Unit::Font(CONSTANTS.radical_extra_ascender as f64)
                    .as_pixels(FONT_SIZE).with_scale(style);
                let height = glyph.height()
                    .as_pixels(FONT_SIZE).with_scale(style);
                let kerning = height
                    - contents.get_height()
                    - rule_thickness
                    - extra_ascender;

                layout.push(vbox!(vec![
                        LayoutNode::Kern(extra_ascender),
                        LayoutNode::Rule(Rule {
                            width:  contents.get_width(),
                            height: rule_thickness,
                            depth:  Pixels(0f64),
                        }),
                        LayoutNode::Kern(kerning),
                        contents,
                    ]));
            },

            ParseNode::Scripts(ref scripts) => {
                // Vertical position of the script is calculated.  This depends on the following
                // information: (see page 26 of https://www.tug.org/tugboat/tb30-1/tb94vieth.pdf)

                use std::boxed::Box;

                // First we calculate the vertical positions of the scripts,
                // by starting with the default positions (these are called)
                // the standard positions in the OpenType specification.
                let mut super_up = Unit::Font(match style.cramped() {
                    true  => CONSTANTS.superscript_shift_up_cramped,
                    false => CONSTANTS.superscript_shift_up,
                } as f64).as_pixels(FONT_SIZE);

                let mut sub_down = Unit::Font(
                    CONSTANTS.subscript_shift_down as f64)
                    .as_pixels(FONT_SIZE);

                println!("Initial ({}, {})", super_up, sub_down);

                // Next we check to see if the scripts meet the minimum
                // clearance requirements relative to the base. If it
                // doesn't, then adjust the baseline shifts for each script.
                // TODO: There must be a beter way to handle the Scripts nodes.
                let script_base = reduce(&mut [ *scripts.base.clone()
                    .unwrap_or(Box::new(ParseNode::Group(vec![]))) ], style);

                let base_height = script_base.get_height();
                let drop_max = Unit::Font(
                    CONSTANTS.superscript_baseline_drop_max as f64)
                    .as_pixels(FONT_SIZE);
                if base_height - super_up > drop_max {
                    super_up = base_height - drop_max;
                }

                println!("BH: {}, DMAX: {}", base_height, drop_max);

                let base_depth = script_base.get_depth();
                let drop_min = Unit::Font(
                    CONSTANTS.subscript_baseline_drop_min as f64)
                    .as_pixels(FONT_SIZE);
                if base_depth + sub_down < drop_min {
                    sub_down = base_depth - drop_min;
                }

                println!("BD: {}, DMIN: {}", base_depth, drop_min);

                println!("Min clearance ({}, {})", super_up, sub_down);

                // Next we check that the superscript isn't too far down
                // and that the subscript isn't too far up.
                //     - CONSTNATS:
                //         - sub_superscript_gap_min: 264,
                //         - subscript_top_max: 400,
                //         - superscript_bottom_max_with_subscript: 400,
                //         - superscript_bottom_min: 125,

                // let bottom_min = Unit::Font(
                //     CONSTANTS.superscript_bottom_max_with_subscript as f64)
                //     .as_pixels(FONT_SIZE);

                // let top_max = Unit::Font(
                //     CONSTANTS.subscript_top_max as f64).as_pixels(FONT_SIZE);

                // if super_up < bottom_min { super_up = bottom_min }
                // if sub_down >

                layout.push(hbox!(script_base));

                let italics_correction =
                    if let Some(ref bx) = scripts.base {
                        if let ParseNode::Symbol(sym) = **bx {
                            let glyph = font::glyph_metrics(sym.unicode);
                            Unit::Font(glyph.italics as f64)
                                .as_pixels(FONT_SIZE).with_scale(style)
                        } else { Pixels(0.0) }
                    } else { Pixels(0.0) };

                let style = style.superscript_variant();
                let mut super_script = reduce(&mut [ *scripts.superscript.clone()
                    .unwrap_or(Box::new(ParseNode::Group(vec![]))) ], style);

                if italics_correction != Pixels(0.0) {
                    super_script.insert(0, LayoutNode::Kern(italics_correction));
                }

                let style = style.subscript_variant();
                let sub_script = hbox!(reduce(&mut [ *scripts.subscript.clone()
                    .unwrap_or(Box::new(ParseNode::Group(vec![]))) ], style));

                if scripts.subscript.is_none() {
                    sub_down = Pixels(0.0);
                }

                let super_script = hbox!(super_script);

                layout.push(vbox!(vec![
                        super_script,
                        LayoutNode::Kern(super_up),
                        sub_script,
                    ], offset: sub_down));
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
                                    .as_pixels(FONT_SIZE).with_scale(style);
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
            height:  self.height() .as_pixels(FONT_SIZE).with_scale(style),
            depth:   self.depth()  .as_pixels(FONT_SIZE).with_scale(style),
            advance: self.advance().as_pixels(FONT_SIZE).with_scale(style),
            unicode: self.unicode,
        })
    }
}