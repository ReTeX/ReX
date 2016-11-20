#![allow(dead_code)]
use constants::{ UNITS_TO_EM };
use super::{ LayoutNode, Rule, HorizontalBox, VerticalBox };

/// Every object that will be rendered will be required to report their size.
/// If the object is a list of other objects, such as typesetting on a horizontal
/// line, or a vectical layout of fractions, then it can recursively read its size
/// from it's children.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox {
    width: f64,
    height: f64,
    depth: f64,
}

pub trait HasBoundingBox {
    fn bounding_box(&self) -> BoundingBox;

    fn get_width(&self) -> f64 {
        self.bounding_box().width
    }

    fn get_height(&self) -> f64 {
        self.bounding_box().height
    }

    fn get_depth(&self) -> f64 {
        self.bounding_box().depth
    }
}

impl  HasBoundingBox for Rule {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }
}

impl HasBoundingBox for HorizontalBox {
    fn bounding_box(&self) -> BoundingBox  {
        let mut width  = 0f64;
        let mut height = 0f64;
        let mut depth  = 0f64;
        for bx in &self.contents {
            width += bx.get_width();
            height = height.max(bx.get_height());
            depth  = depth.min(bx.get_depth());
        }
        BoundingBox {
            width: width,
            height: height,
            depth: depth,
        }
    }
}

impl HasBoundingBox for VerticalBox {
    fn bounding_box(&self) -> BoundingBox  {
        let mut width  = 0f64;
        let mut height = 0f64;
        let mut depth  = 0f64;
        for bx in &self.contents {
            width   = width.max(bx.get_width());
            height += bx.get_height();
            depth   = depth.min(bx.get_depth());
        }
        BoundingBox {
            width: width,
            height: height,
            depth: depth,
        }
    }    
}

impl<'a> HasBoundingBox for [LayoutNode] {
    fn bounding_box(&self) -> BoundingBox {
        let mut width  = 0f64;
        let mut height = 0f64;
        let mut depth  = 0f64;
        for bx in self {
            width += bx.get_width();
            height = height.max(bx.get_height());
            depth  = depth.min(bx.get_depth());
        }
        BoundingBox {
            width: width,
            height: height,
            depth: depth,
        }        
    }
}

use font::Glyph;
impl HasBoundingBox for Glyph {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            width:  self.advance as f64 * UNITS_TO_EM,
            height: self.bbox.3 as f64  * UNITS_TO_EM,
            depth:  self.bbox.1 as f64  * UNITS_TO_EM,
        }
    }
}

use spacing::Spacing;
impl HasBoundingBox for Spacing {
    fn bounding_box(&self) -> BoundingBox {
        let width = match *self {
            Spacing::None   => 0f64,
            Spacing::Thin   => 1_f64/6_f64,
            Spacing::Medium => 2_f64/9_f64,
            Spacing::Thick  => 3_f64/9_f64,
        };

        BoundingBox {
            width: width,
            height: 0f64,
            depth: 0f64,
        }
    }
}

impl HasBoundingBox for LayoutNode {
    fn bounding_box(&self) -> BoundingBox {
        match *self {
            LayoutNode::HorizontalBox(ref hbox) => hbox.bounding_box(),
            LayoutNode::VerticalBox(ref vbox)   => vbox.bounding_box(),
            LayoutNode::Glyph(ref gly)          => gly.bounding_box(),
            LayoutNode::Space(ref sp)           => sp.bounding_box(),
            LayoutNode::Rule(ref rule)          => rule.bounding_box(),
            LayoutNode::Kern(k)                 => BoundingBox { width: k, height: k, depth: 0f64 },
        }
    }
}