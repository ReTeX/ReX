#![allow(dead_code)]
// use parser::nodes::{ ParseNode };
// use font::{GLYPHS};
// use spacing::atom_spacing;
use layout::LayoutNode;
use layout::boundingbox::Bounded;
use dimensions::Pixels;

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

pub const FONT_SIZE: f64 = 48.0;
const LEFT_PADDING: f64  = 12.0;
const TOP_PADDING: f64   = 5.0;
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

        let width  = nodes.get_width()  + 2.0 * self.cursor.x;   // Left and right padding
        let height = nodes.get_height() + 2.0 * self.cursor.y;   // Top and bot padding
        let depth  = nodes.get_depth();

        output += &format!(HEAD_TEMPLATE!(), width, height - depth, "rex-xits.otf", FONT_SIZE);

        output += &format!(G_TEMPLATE!(), LEFT_PADDING, TOP_PADDING);
        output += &self.render_hbox(&nodes);
        output += "</g>";
        output += "</g></svg>";
        output
    }

    pub fn render_hbox(&self, nodes: &[LayoutNode]) -> String {
        let mut result = String::new();

        let height = nodes.get_height();
        let mut width = Pixels(0.0);

        use layout::boundingbox::Bounded;
        result += &format!(BBOX_TEMPLATE!(), 0, 0, 
            nodes.get_width(), nodes.get_height());

        for node in nodes { match *node {
            LayoutNode::Glyph(ref gly) => {
                result += &format!(G_TEMPLATE!(), width, height);
                //result += &format!(BBOX_TEMPLATE!(), 0, -gh, gw, gh-gd);
                if gly.scale != 1f64 {
                    result += &format!(SCALE_TEMPLATE!(), gly.scale, gly.scale);
                }

                result += &format!(SYM_TEMPLATE!(), ::std::char::from_u32(gly.unicode)
                    .expect("Unable to decode unicode!"));

                if gly.scale != 1f64 {
                    result += "</g>";
                }

                let gw = gly.advance;
                width += gw;          
            },
            LayoutNode::Space(_) =>
                width += node.get_width(),
            LayoutNode::Rule(rule) => {
                result += &format!(RULE_TEMPLATE!(), 
                    width, height - rule.height, rule.width, rule.height);
                width += rule.width;
            },
            LayoutNode::VerticalBox(ref vbox) => {
                result += &format!(G_TEMPLATE!(), width, height - vbox.get_height());
                result += &self.render_vbox(&vbox.contents);
                result += "</g>";
                width += vbox.get_width();
            },
            LayoutNode::HorizontalBox(ref hbox) => {
                result += &format!(G_TEMPLATE!(), width, height - hbox.get_height());
                result += &self.render_hbox(&hbox.contents);
                result += "</g>";
                width += hbox.get_width();
            },
            LayoutNode::Kern(k) =>
                width += k,
        }}

        result
    }

    pub fn render_vbox(&self, nodes: &[LayoutNode]) -> String {
        let mut result = String::new();
        
        let mut height = Pixels(0.0);
        let width      = Pixels(0.0);

        for node in nodes { match *node {
            LayoutNode::Rule(rule) => {
                result += &format!(RULE_TEMPLATE!(), 
                    width, height - rule.height, 
                    nodes.get_width(), rule.height);
                height += rule.height;
            },
            LayoutNode::Space(_) =>
                height += node.get_width(),
            LayoutNode::HorizontalBox(ref hbox) => {
                result += &format!(G_TEMPLATE!(), width, height);
                result += &self.render_hbox(&hbox.contents);
                result += "</g>";
                height += hbox.get_height();      
            },
            LayoutNode::Kern(k) =>
                height += k,
            _ => (),
        }}

        result
    }
}