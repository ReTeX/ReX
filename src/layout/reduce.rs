use super::{ LayoutNode, LayoutGlyph, Rule, HorizontalBox, VerticalBox, Style };

use spacing::Spacing;
use spacing::atom_spacing;
use dimensions::{ Pixels, Unit };
use font;
use font::CONSTANTS;
use font::GLYPHS;
use font::SYMBOLS;
use layout::{ ToPixels, Scalable };
use layout::boundingbox::Bounded;
use parser::nodes::{ ParseNode, AtomType };
use render::FONT_SIZE;
use font::glyph_metrics;
use font::variants::Variant;
use font::variants::VariantGlyph;

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
                    layout.push(LayoutNode::Kern(sp.to_unit()
                        .as_pixels(FONT_SIZE)
                        .with_scale(style)));
                }
            }
        }

        // TODO: May need to ignore this if transparent atom_type.
        prev_at = node.atom_type();

        match *node {
            ParseNode::Symbol(sym) => {
                let glyph = font::glyph_metrics(sym.unicode);
                layout.push(glyph.into_layout_node(style));
            },

            ParseNode::Group(ref mut gp) =>
                layout.push(hbox!(reduce(&mut gp.clone(), style))),

            ParseNode::Rule(rule) =>
                layout.push(LayoutNode::Rule(Rule {
                    width:  rule.width .as_pixels(FONT_SIZE).with_scale(style),
                    height: rule.height.as_pixels(FONT_SIZE).with_scale(style),
                    // TODO: Implement this (needs optional macro arguments parsing)
                    depth:  Pixels(0f64),
                })),

            ParseNode::Kerning(kern) =>
                layout.push(LayoutNode::Kern(kern.as_pixels(FONT_SIZE).with_scale(style))),

            ParseNode::Spacing(sp) =>
                layout.push(LayoutNode::Kern(sp.to_unit()
                    .as_pixels(FONT_SIZE).with_scale(style))),

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
                use std::boxed::Box;

                let script_base = reduce(&mut [ *scripts.base.clone()
                    .unwrap_or(Box::new(ParseNode::Group(vec![]))) ], style);
                layout.push(hbox!(script_base));

                let style = style.superscript_variant();
                let super_script = hbox!(
                    reduce(&mut [ *scripts.superscript.clone()
                        .unwrap_or(Box::new(ParseNode::Group(vec![]))) ], style));

                layout.push(vbox!(vec![
                        super_script,
                        LayoutNode::Kern(style.sup_shift_up()
                            .as_pixels(FONT_SIZE).with_scale(style)),
                    ]));
            },

            ParseNode::Extend(u) => {
                // TODO: Remove me, only used for testing.
                let paren = glyph_metrics(0x28); // Left parantheses

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
                                contents.push(LayoutNode::Kern(
                                    Unit::Font(-instr.overlap)
                                        .as_pixels(FONT_SIZE)
                                        .with_scale(style) ));
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