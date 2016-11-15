#![allow(dead_code)]
use parser::nodes::{ ParseNode };
use font::{GLYPHS};
use spacing::atom_spacing;
use constants::{ UNITS_TO_EM, EM_TO_PX };
use layout::LayoutNode;
use layout::boundingbox::HasBoundingBox;

const SCALE: f64 = 8 as f64/450 as f64;

macro_rules! HEAD_TEMPLATE { () => { "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\"><defs><style type=\"text/css\">@font-face{{font-family: rex;src: url('{}');}}</style></defs><g font-family=\"rex\" font-size=\"16px\">" } }
macro_rules! G_TEMPLATE { () => { "<g transform=\"translate({},{})\">\n" } }
macro_rules! BBOX_TEMPLATE { () => { "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"none\" stroke=\"blue\" stroke-width=\"0.2\"/>\n" } }
macro_rules! SYM_TEMPLATE { () => { "<text>{}</text></g>\n" } }

pub fn render_inline(nodes: &[LayoutNode]) -> String {
    let mut result = String::new();

    let height = nodes.get_height();
    let mut width = 16f64; // TODO: Should be left side padding
    let padding_y = 16f64; // TODO: Should be top padding

    for node in nodes { match *node {
        LayoutNode::Glyph(ref gly) => {
            let gw = gly.advance;
            result += &format!(G_TEMPLATE!(), width, (height + padding_y));
            //result += &format!(BBOX_TEMPLATE!(), 0, -gh, gw, gh-gd);
            result += &format!(SYM_TEMPLATE!(), ::std::char::from_u32(gly.unicode)
                .expect("Unable to decode unicode!"));
            width += gw as f64 * UNITS_TO_EM * EM_TO_PX;          
        },
        LayoutNode::Space(_) =>
            width += node.get_width(),
        _ => (),
    }}

    result
}

pub fn render(nodes: Vec<LayoutNode>) -> String {
    println!("{:?}", nodes);

    let mut result = String::new();
    let mut output = String::from("<?xml version=\"1.0\" standalone=\"no\"?>\
         <!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \
         \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">");

    let width  = nodes.get_width() + 16f64;
    let height = nodes.get_height();
    let depth  = nodes.get_depth();

    output += &format!(HEAD_TEMPLATE!(), width + 16f64, height - depth + 16f64, "rex-xits.otf");
    output += &render_inline(&nodes);
    output += "</g></svg>";
    output
}