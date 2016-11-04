#![allow(dead_code)]
use parser::nodes::{ ParseNode };
use symbols::Symbol;
use metrics::CONSTANTS;

struct Cursor {
    pub x: i16,
    pub y: i16,
}

struct DocumentState {
    pub line_height: u8,
    pub doc_hieght: u16,
    pub doc_width: u16,
}

const SCALE: f64 = 8 as f64/504 as f64;

pub fn render(nodes: Vec<ParseNode>) {
    let mut cursor = Cursor { x: 0, y: 0 };
    cursor.x += CONSTANTS.math_leading;

    for node in nodes {
        match node {
            ParseNode::Symbol(sym) => {
                match sym.atom_type {
                    
                }
            }
        }
    }
}