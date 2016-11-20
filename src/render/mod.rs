#![allow(dead_code)]
// use parser::nodes::{ ParseNode };
// use font::{GLYPHS};
// use spacing::atom_spacing;
use constants::UNITS_TO_EM;
use layout::LayoutNode;
use layout::boundingbox::HasBoundingBox;

macro_rules! HEAD_TEMPLATE { () => { "<svg width=\"{:.2}\" height=\"{:.2}\" xmlns=\"http://www.w3.org/2000/svg\"><defs><style type=\"text/css\">@font-face{{font-family: rex;src: url('{}');}}</style></defs><g font-family=\"rex\" font-size=\"{:.1}px\">" } }
macro_rules! G_TEMPLATE { () => { "<g transform=\"translate({:.2},{:.2})\">\n" } }
macro_rules! BBOX_TEMPLATE { () => { "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"none\" stroke=\"blue\" stroke-width=\"0.2\"/>\n" } }
macro_rules! SYM_TEMPLATE { () => { "<text>{}</text></g>\n" } }
macro_rules! RULE_TEMPLATE { () => { r#"<rect x="{}" y="{}" width="{}" height="{}" fill="\#000"/>"# } }
macro_rules! SCALE_TEMPLATE { () => { r#"<g transform="scale({} {})">"# } }

struct Cursor {
    x: f64,
    y: f64,
}

pub struct Renderer {
    cursor: Cursor,
    nodes: Vec<LayoutNode>
}

const LEFT_PADDING: f64 = 12.0;
const TOP_PADDING: f64  = 5.0;
const FONT_SIZE: f64    = 48.0;
const SVG_HEADER: &'static str = "<?xml version=\"1.0\" standalone=\"no\"?>\
    <!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">";

impl Renderer {
    pub fn new(nodes: Vec<LayoutNode>) -> Renderer {
        let cursor = Cursor {
            x: LEFT_PADDING, // Left padding
            y: TOP_PADDING,  // Top  padding
        };

        Renderer {
            cursor: cursor,
            nodes: nodes,
        }
    }

    pub fn render(&self) -> String {
        let nodes = &self.nodes;
        println!("{:?}", nodes);

        let mut output = String::from(SVG_HEADER);

        let width  = nodes.get_width() * FONT_SIZE  + 2.0 * LEFT_PADDING;   // Left and right padding
        let height = nodes.get_height() * FONT_SIZE + 2.0 * TOP_PADDING;   // Top and bot padding
        let depth  = nodes.get_depth() * FONT_SIZE;

        output += &format!(HEAD_TEMPLATE!(), width, height - depth, "rex-xits.otf", FONT_SIZE);
        
        output += &format!(G_TEMPLATE!(), LEFT_PADDING, TOP_PADDING);
        output += &self.render_hbox(&nodes);
        output += "</g>";
        output += "</g></svg>";
        output
    }

    pub fn render_hbox(&self, nodes: &[LayoutNode]) -> String {
        let mut result = String::new();

        let height = nodes.get_height() * FONT_SIZE;
        let mut width: f64 = 0.0;

        use layout::boundingbox::HasBoundingBox;
        result += &format!(BBOX_TEMPLATE!(), 0, 0, 
            nodes.get_width() * FONT_SIZE, nodes.get_height() * FONT_SIZE);

        for node in nodes { match *node {
            LayoutNode::Glyph(ref gly) => {
                result += &format!(G_TEMPLATE!(), width, height);
                //result += &format!(BBOX_TEMPLATE!(), 0, -gh, gw, gh-gd);
                result += &format!(SYM_TEMPLATE!(), ::std::char::from_u32(gly.unicode)
                    .expect("Unable to decode unicode!"));

                let gw = gly.advance as f64 * UNITS_TO_EM * FONT_SIZE;
                width += gw;          
            },
            LayoutNode::Space(_) =>
                width += node.get_width() * FONT_SIZE,
            LayoutNode::Rule(rule) => {
                result += &format!(RULE_TEMPLATE!(), 
                    width, height - rule.height * FONT_SIZE, 
                    rule.width * FONT_SIZE, rule.height * FONT_SIZE);
                width += rule.width * FONT_SIZE;
            },
            LayoutNode::VerticalBox(ref vbox) => {
                result += &format!(G_TEMPLATE!(), width, height - vbox.get_height() * FONT_SIZE);
                result += &self.render_vbox(&vbox.contents);
                result += "</g>";
                width += vbox.get_width() * FONT_SIZE;
            },
            LayoutNode::HorizontalBox(ref hbox) => {
                result += &format!(G_TEMPLATE!(), width, height - hbox.get_height() * FONT_SIZE);
                result += &self.render_hbox(&hbox.contents);
                result += "</g>";
                width += hbox.get_width() * FONT_SIZE;                
            },
            LayoutNode::Scale(scale, ref node) => {
                let b_in = *node.clone();
                result += &format!(SCALE_TEMPLATE!(), scale, scale);
                if let LayoutNode::HorizontalBox(ref hbox) = b_in {
                    result += &self.render_hbox(&hbox.contents);
                }
                result += "</g>";                
            },
            _ => (),
        }}

        result
    }

    pub fn render_vbox(&self, nodes: &[LayoutNode]) -> String {
        let mut result = String::new();
        
        let mut height: f64 = 0.0;
        let mut width: f64 = 0.0;

        for node in nodes { match *node {
            LayoutNode::Rule(rule) => {
                result += &format!(RULE_TEMPLATE!(), 
                    width, height - rule.height * FONT_SIZE, 
                    nodes.get_width() * FONT_SIZE, rule.height * FONT_SIZE);
                height += rule.height * FONT_SIZE;
            },
            LayoutNode::Space(_) =>
                height += node.get_width() * FONT_SIZE,
            LayoutNode::HorizontalBox(ref hbox) => {
                result += &format!(G_TEMPLATE!(), width, height);
                result += &self.render_hbox(&hbox.contents);
                result += "</g>";
                height += hbox.get_height() * FONT_SIZE;      
            },
            LayoutNode::Kern(k) =>
                height += k * FONT_SIZE,
            LayoutNode::Scale(scale, ref node) => {
                let b_in = *node.clone();
                result += &format!(SCALE_TEMPLATE!(), scale, scale);
                if let LayoutNode::HorizontalBox(ref hbox) = b_in {
                    result += &self.render_hbox(&hbox.contents);
                }
                result += "</g>";                  
            }
            _ => (),
        }}

        result
    }
}