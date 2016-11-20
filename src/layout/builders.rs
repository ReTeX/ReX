#![allow(dead_code)]
use super::{ HorizontalBox, LayoutNode };
use font::Glyph;
use ::std::default::Default;
use super::boundingbox::HasBoundingBox;

pub struct LayoutNodeBuilder {}

impl LayoutNodeBuilder {
    pub fn horizontal_box(&mut self, contents: Vec<LayoutNode>) -> LayoutNode {
        let bbox = contents.bounding_box();
        LayoutNode {
            bounding_box: bbox,
            data: LayoutNodeData::HorizontalBox(HorizontalBox {
                contents: contents,
                ..Default::default()
            }),
        }
    }

    fn glyph(&mut self, glyph: Glyph) -> LayoutNode {
        LayoutNode {
            bounding_box: glyph.bbox,
            data: LayoutNodeData::Glyph(glyph),
        }
    }
}

impl LayoutNode {
    pub fn build() -> LayoutNodeBuilder {
        LayoutNodeBuilder {}
    }
}

// LayoutNode::build().horiztonal_box(vec![
//     ln::Build
// ])