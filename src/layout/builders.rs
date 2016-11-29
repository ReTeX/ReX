#![allow(dead_code)]
use super::{ VerticalBox, HorizontalBox, LayoutNode, LayoutVariant };
use dimensions::Pixels;

#[derive(Default)]
pub struct VBox {
    width:  Pixels,
    height: Pixels,
    depth:  Pixels,
    node:   VerticalBox,
}

impl VBox {
    pub fn new() -> VBox { VBox::default() }

    pub fn add_node(&mut self, node: LayoutNode) {
        self.width   = self.width.max(node.width);
        self.height += node.height;
        self.node.contents.push(node);
    }

    pub fn set_offset(&mut self, offset: Pixels) {
        self.node.offset = offset;
    }

    pub fn build(mut self) -> LayoutNode {
        // The depth only depends on the depth
        // of the last element and offset.
        if let Some(node) = self.node.contents.last() {
            self.depth = node.depth;
        }

        self.depth  -= self.node.offset;
        self.height -= self.node.offset;

        LayoutNode {
            width:  self.width,
            height: self.height,
            depth:  self.depth,
            node:   LayoutVariant::VerticalBox(self.node),
        }
    }
}

macro_rules! vbox {
    (offset: $offset:expr; $($node:expr),*) => ({
        let mut _vbox = builders::VBox::new();
        $( _vbox.add_node($node); )*
        _vbox.set_offset($offset);
        _vbox.build()
    });

    ( $($node:expr),* ) => ({
        let mut _vbox = builders::VBox::new();
        $( _vbox.add_node($node); )*
        _vbox.build()
    });
}

#[derive(Default)]
pub struct HBox {
    width:  Pixels,
    height: Pixels,
    depth:  Pixels,
    node:   HorizontalBox,
}

impl HBox {
    pub fn new() -> HBox { HBox::default() }

    pub fn add_node(&mut self, node: LayoutNode) {
        self.width += node.width;
        self.height = self.height.max(node.height);
        self.depth  = self.depth.min(node.depth);
        self.node.contents.push(node);
    }

    pub fn set_offset(&mut self, offset: Pixels) {
        self.node.offset = offset;
    }

    pub fn build(mut self) -> LayoutNode {
        self.depth  -= self.node.offset;
        self.height -= self.node.offset;

        LayoutNode {
            width:  self.width,
            height: self.height,
            depth:  self.depth,
            node:   LayoutVariant::HorizontalBox(self.node),
        }
    }
}

macro_rules! hbox {
    (offset: $offset:expr; $($node:expr),*) => ({
        let mut _hbox = builders::HBox::new();
        $( _hbox.add_node($node); )*
        _hbox.set_offset($offset);
        _hbox.build()
    });

    ( $($node:expr),* ) => ({
        let mut _hbox = builders::HBox::new();
        $( _hbox.add_node($node); )*
        _hbox.build()
    });
}

macro_rules! rule {
    (width: $width:expr, height: $height:expr) => (
        rule!(width: $width, height: $height, depth: ::dimensions::Pixels::default())
    );

    (width: $width:expr, height: $height:expr, depth: $depth:expr) => (
        LayoutNode {
            width:  $width,
            height: $height,
            depth:  $depth,
            node: LayoutVariant::Rule,
        }
    );
}

macro_rules! kern {
    (vert: $height:expr) => (
        LayoutNode {
            width: ::dimensions::Pixels::default(),
            height: $height,
            depth: ::dimensions::Pixels::default(),
            node: LayoutVariant::Kern,
        }
    );

    (horz: $width:expr) => (
        LayoutNode {
            width: $width,
            height: ::dimensions::Pixels::default(),
            depth: ::dimensions::Pixels::default(),
            node: LayoutVariant::Kern,
        }
    );
}