use super::{ LayoutNode, Rule, HorizontalBox, VerticalBox, Style };

use dimensions::{ Pixels, Unit };
use font;
use font::CONSTANTS;
use font::GLYPHS;
use font::SYMBOLS;
use layout::{ ToPixels, Scalable };
use layout::boundingbox::Bounded;
use parser::nodes::{ ParseNode, AtomType };
use render::FONT_SIZE;

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
                use spacing::Spacing;
                use spacing::atom_spacing;
                let sp = atom_spacing(p_at, at);
                if sp != Spacing::None {
                    layout.push(LayoutNode::Kern(sp.to_unit()
                        .as_pixels(FONT_SIZE).with_scale(style)));
                }
            }
        }

        // TODO: May need to ignore this if transparent atom_type.
        prev_at = node.atom_type();

        use layout::LayoutGlyph;
        match *node {
            ParseNode::Symbol(sym) => {
                let glyph = font::glyph_metrics(sym.unicode);
                layout.push(LayoutNode::Glyph(LayoutGlyph {
                    scale:   style.font_scale(),
                    height:  glyph.height() .as_pixels(FONT_SIZE).with_scale(style),
                    depth:   glyph.depth()  .as_pixels(FONT_SIZE).with_scale(style),
                    advance: glyph.advance().as_pixels(FONT_SIZE).with_scale(style),
                    unicode: glyph.unicode,
                }))
            },
            ParseNode::Group(ref mut gp) =>
                layout.push(LayoutNode::HorizontalBox(HorizontalBox {
                    contents: reduce(&mut gp.clone(), style),
                    ..Default::default()
                })),
            ParseNode::Rule(rule) =>
                layout.push(LayoutNode::Rule(Rule {
                    width:  rule.width .as_pixels(FONT_SIZE).with_scale(style),
                    height: rule.height.as_pixels(FONT_SIZE).with_scale(style),
                    // TODO: Implement this (needs optional macro arguments parsing)
                    depth:  Pixels(0f64),
                })),
            ParseNode::Kerning(kern) =>
                layout.push(LayoutNode::Kern(kern.as_pixels(FONT_SIZE).with_scale(style))),
            ParseNode::Spacing(sp) => {
                layout.push(LayoutNode::Kern(sp.to_unit()
                    .as_pixels(FONT_SIZE).with_scale(style)))
            },
            ParseNode::Radical(ref rad) => {
                //Reference rule 11 from pg 443 of TeXBook
                let style = style.cramped_variant();
                let glyph = &GLYPHS[&SYMBOLS["sqrt"].unicode];
                layout.push(LayoutNode::Glyph(LayoutGlyph {
                    scale:   style.font_scale(),
                    height:  glyph.height() .as_pixels(FONT_SIZE).with_scale(style),
                    depth:   glyph.depth()  .as_pixels(FONT_SIZE).with_scale(style),
                    advance: glyph.advance().as_pixels(FONT_SIZE).with_scale(style),
                    unicode: glyph.unicode,
                }));

                let contents = LayoutNode::HorizontalBox(HorizontalBox {
                    contents: reduce(&mut rad.inner.clone(), style),
                    ..Default::default()
                });

                let rule_thickness = Unit::Font(CONSTANTS.radical_rule_thickness as f64)
                    .as_pixels(FONT_SIZE).with_scale(style);
                let extra_ascender = Unit::Font(CONSTANTS.radical_extra_ascender as f64)
                    .as_pixels(FONT_SIZE).with_scale(style);
                let height         = glyph.height().as_pixels(FONT_SIZE).with_scale(style);
                let kerning        = height - contents.get_height() - rule_thickness - extra_ascender;

                layout.push(
                    LayoutNode::VerticalBox(VerticalBox {
                        contents: vec![
                            LayoutNode::Kern(extra_ascender),
                            LayoutNode::Rule(Rule {
                                width:  contents.get_width(),
                                height: rule_thickness,
                                depth:  Pixels(0f64),
                            }),
                            LayoutNode::Kern(kerning),
                            contents,
                        ],
                        ..Default::default()
                    }));
            },
            ParseNode::Scripts(ref scripts) => {
                use std::boxed::Box;

                let script_base = reduce(&mut [ *scripts.base.clone()
                    .unwrap_or(Box::new(ParseNode::Group(vec![]))) ], style);
                layout.push(
                    LayoutNode::HorizontalBox(HorizontalBox {
                        contents: script_base,
                        ..Default::default()
                    })
                );

                let style = style.superscript_variant();
                let super_script = LayoutNode::HorizontalBox(HorizontalBox {
                        contents: reduce(&mut [ *scripts.superscript.clone()
                            .unwrap_or(Box::new(ParseNode::Group(vec![]))) ], style),
                        ..Default::default()
                    });

                layout.push(LayoutNode::VerticalBox(VerticalBox {
                    contents: vec![
                        super_script,
                        LayoutNode::Kern(style.sup_shift_up()
                            .as_pixels(FONT_SIZE).with_scale(style)),
                    ],
                    ..Default::default()
                }));
            },
            ParseNode::GenFraction(ref frac) => {

            }
            _ => (),
       }
    }

    layout
}