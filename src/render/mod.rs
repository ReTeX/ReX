#![allow(dead_code)]
use parser::nodes::{ ParseNode };
use font::{Symbol, CONSTANTS, GLYPHS};

const SCALE: f64 = 8 as f64/450 as f64;

macro_rules! HEAD_TEMPLATE { () => { "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\"><defs><style type=\"text/css\">@font-face{{font-family: rex;src: url('{}');}}</style></defs><g font-family=\"rex\" font-size=\"16px\">" } }
macro_rules! G_TEMPLATE { () => { "<g transform=\"translate({},{})\">\n" } }
macro_rules! BBOX_TEMPLATE { () => { "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"none\" stroke=\"blue\" stroke-width=\"0.2\"/>\n" } }
macro_rules! SYM_TEMPLATE { () => { "<text>{}</text></g>\n" } }

pub fn render(nodes: Vec<ParseNode>) -> String {
    let mut body   = String::new();
    let mut output = String::from("<?xml version=\"1.0\" standalone=\"no\"?>\
        <!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \
        \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">");

    const em_to_px: f64    = 16f64;
    const units_to_em: f64 = 1f64/1000f64;
    
    let mut depth  = 0f64;
    let mut height = 0f64;
    let mut width  = 16f64;
    let y      = 16f64;

    // First calculate the height to determine the baseline
    for node in &nodes {
        if let &ParseNode::Symbol(sym) = node {
            let glyph = &GLYPHS[sym.id as usize];
            height = height.max(glyph.bbox.3 as f64 * units_to_em * em_to_px);
            depth  = depth.min(glyph.bbox.1 as f64 * units_to_em * em_to_px);
        }
    }

    for node in &nodes {
        if let &ParseNode::Symbol(sym) = node {
            let glyph  = &GLYPHS[sym.id as usize];

            println!("{:?}", glyph);

            let gh = glyph.bbox.3 as f64 * units_to_em * em_to_px;
            let gd = glyph.bbox.1 as f64 * units_to_em * em_to_px;
            let gw = glyph.advance as f64 * units_to_em * em_to_px;

            body += &format!(G_TEMPLATE!(), width , (height + y) );
            body += &format!(BBOX_TEMPLATE!(), 0, -gh, gw, gh-gd);
            body += &format!(SYM_TEMPLATE!(), ::std::char::from_u32(glyph.unicode)
                .expect("Unable to decode unicode!"));
            
            width += gw;
        }
    }

    output += &format!(HEAD_TEMPLATE!(), width + 16f64, height - depth + 16f64, "rex-xits.otf");
    output += &body;
    output += "</g></svg>";
    output
}