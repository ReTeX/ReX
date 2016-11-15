use super::{ HorizontalBox, LayoutNode };
use font::Glyph;
use ::std::default::Default;

pub struct LayoutNodeBuilder {}

impl LayoutNodeBuilder {
    pub fn HorizontalBox(&mut self, contents: Vec<LayoutNode>) -> LayoutNode {
        LayoutNode::HorizontalBox(HorizontalBox {
            contents: contents,
            ..Default::default()
        })
    }

    fn Glyph(&mut self, glyph: Glyph) -> LayoutNode {
        LayoutNode::Glyph(glyph)
    }
}

impl LayoutNode {
    pub fn build() -> LayoutNodeBuilder {
        LayoutNodeBuilder {}
    }
}