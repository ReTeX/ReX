#![allow(dead_code)]
use parser::nodes::{ ParseNode };
use font::{Symbol, CONSTANTS, GLYPHS};

struct Cursor {
    pub x: i16,
    pub y: i16,
}

struct HBox {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub contents: Vec<Symbol>,
}

const SCALE: f64 = 8 as f64/450 as f64;

pub fn render(nodes: Vec<ParseNode>) {
    let mut hbox = HBox { width: 0., height: 0., depth: 0., contents: vec![] };

    for node in nodes {
        if let ParseNode::Symbol(sym) = node {
            let glyph = sym.id;
            hbox.width += glyph.advance;
            hbox.height = hbox.height.max(glyph.height);
            hbox.depth  = hbox.depth.min(glyph.depth);
            hbox.contents.push(glyph);
        }
    }
}