#![allow(dead_code)]
use dimensions::Pixels;
use super::{ LayoutNode, Rule, HorizontalBox, VerticalBox, LayoutGlyph };

/// Every object that will be rendered will be required to report their size.
/// If the object is a list of other objects, such as typesetting on a horizontal
/// line, or a vectical layout of fractions, then it can recursively read its size
/// from it's children.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox {
    width:  Pixels,
    height: Pixels,
    depth:  Pixels,
}

pub trait Bounded {
    fn bounding_box(&self) -> BoundingBox;

    fn get_width(&self)  -> Pixels { self.bounding_box().width  }
    fn get_height(&self) -> Pixels { self.bounding_box().height }
    fn get_depth(&self)  -> Pixels { self.bounding_box().depth  }
}

impl Bounded for Rule {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            width:  self.width,
            height: self.height,
            depth:  self.depth,
        }
    }
}

impl Bounded for HorizontalBox {
    fn bounding_box(&self) -> BoundingBox  {
        let mut width  = Pixels(0f64);
        let mut height = Pixels(0f64);
        let mut depth  = Pixels(0f64);
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

impl Bounded for VerticalBox {
    fn bounding_box(&self) -> BoundingBox  {
        let mut width  = Pixels(0f64);
        let mut height = Pixels(0f64);
        let mut depth  = Pixels(0f64);
        for bx in &self.contents {
            width   = width.max(bx.get_width());
            height += bx.get_height();
            depth   = depth.min(bx.get_depth());
        }
        BoundingBox {
            width: width,
            height: height - self.offset,
            depth: depth + self.offset,
        }
    }
}

impl<'a> Bounded for [LayoutNode] {
    fn bounding_box(&self) -> BoundingBox {
        let mut width  = Pixels(0f64);
        let mut height = Pixels(0f64);
        let mut depth  = Pixels(0f64);
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

impl Bounded for LayoutGlyph {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            width:  self.advance,
            height: self.height,
            depth:  self.depth,
        }
    }
}

use spacing::Spacing;
impl Bounded for Spacing {
    fn bounding_box(&self) -> BoundingBox {
        let width = match *self {
            Spacing::None   => Pixels(0f64),
            Spacing::Thin   => Pixels(1_f64/6_f64),
            Spacing::Medium => Pixels(2_f64/9_f64),
            Spacing::Thick  => Pixels(3_f64/9_f64),
        };

        BoundingBox {
            width:  width,
            height: Pixels(0f64),
            depth:  Pixels(0f64),
        }
    }
}

impl Bounded for LayoutNode {
    fn bounding_box(&self) -> BoundingBox {
        match *self {
            LayoutNode::HorizontalBox(ref hbox) => hbox.bounding_box(),
            LayoutNode::VerticalBox(ref vbox)   => vbox.bounding_box(),
            LayoutNode::Glyph(ref gly)          => gly.bounding_box(),
            LayoutNode::Space(ref sp)           => sp.bounding_box(),
            LayoutNode::Rule(ref rule)          => rule.bounding_box(),
            LayoutNode::Kern(k)                 =>
                BoundingBox { width: k, height: k, depth: Pixels(0f64) },
        }
    }
}