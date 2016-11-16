#![allow(dead_code)]
use super::{ HorizontalBox, LayoutNode };
use font::Glyph;
use ::std::default::Default;

pub struct LayoutNodeBuilder {}

impl LayoutNodeBuilder {
    pub fn horizontal_box(&mut self, contents: Vec<LayoutNode>) -> LayoutNode {
        LayoutNode::HorizontalBox(HorizontalBox {
            contents: contents,
            ..Default::default()
        })
    }

    fn glyph(&mut self, glyph: Glyph) -> LayoutNode {
        LayoutNode::Glyph(glyph)
    }
}

impl LayoutNode {
    pub fn build() -> LayoutNodeBuilder {
        LayoutNodeBuilder {}
    }
}