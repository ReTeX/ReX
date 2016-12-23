#![allow(dead_code)]
// use parser::nodes::{ ParseNode };
// use font::{GLYPHS};
// use spacing::atom_spacing;
use layout::{ LayoutNode, Layout, LayoutVariant, Alignment };
//use layout::boundingbox::Bounded;
use dimensions::Pixels;

macro_rules! HEAD_TEMPLATE { () => { "<svg width=\"{:.2}\" height=\"{:.2}\" encoding=\"utf-8\" xmlns=\"http://www.w3.org/2000/svg\"><defs><style type=\"text/css\">@font-face{{font-family: rex;src: url('{}');}}</style></defs><g font-family=\"rex\" font-size=\"{:.1}px\">" } }
macro_rules! G_TEMPLATE { () => { "<g transform=\"translate({:.2},{:.2})\">\n" } }
macro_rules! BBOX_TEMPLATE { () => { "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"none\" stroke=\"blue\" stroke-width=\"0.2\"/>\n" } }
macro_rules! SYM_TEMPLATE { () => { "<text>{}</text></g>\n" } }
macro_rules! RULE_TEMPLATE { () => { r#"<rect x="{}" y="{}" width="{}" height="{}" fill="\#000"/>"# } }
macro_rules! SCALE_TEMPLATE { () => { r#"<g transform="scale({})">"# } }

struct Cursor {
    x: f64,
    y: f64,
}

pub struct Renderer {
    cursor: Cursor,
    layout: Layout,
}

pub const FONT_SIZE:    f64 = 128.0;
const     LEFT_PADDING: f64 = 12.0;
const     TOP_PADDING:  f64 = 5.0;
const     SVG_HEADER: &'static str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\
    <!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">";

impl Renderer {
    pub fn new(layout: Layout) -> Renderer {
        let cursor = Cursor {
            x: LEFT_PADDING, // Left padding
            y: TOP_PADDING,  // Top  padding
        };

        Renderer {
            cursor: cursor,
            layout: layout,
        }
    }

    pub fn render(&self) -> String {
        let mut output = String::from(SVG_HEADER);

        let width  = self.layout.width  + 2.0 * self.cursor.x;   // Left and right padding
        let height = self.layout.height + 2.0 * self.cursor.y;   // Top and bot padding
        let depth  = self.layout.depth;

        output += &format!(HEAD_TEMPLATE!(), width, height - depth, "rex-xits.otf", FONT_SIZE);

        output += &format!(G_TEMPLATE!(), LEFT_PADDING, TOP_PADDING);
        output += &self.render_hbox(&self.layout.contents,
            self.layout.height, self.layout.width, Alignment::Default);
        output += "</g>";
        output += "</g></svg>";
        output
    }

    pub fn render_hbox(&self, nodes: &[LayoutNode],
            height: Pixels, nodes_width: Pixels, alignment: Alignment) -> String {
        let mut result = String::new();
        let mut width = Pixels(0.0);

        if let Alignment::Centered(w) = alignment {
            width += (nodes_width - w)/2.0;
        }

        result += &format!(BBOX_TEMPLATE!(), 0, 0, nodes_width, height);

        for node in nodes { match node.node {
            LayoutVariant::Glyph(ref gly) => {
                result += &format!(G_TEMPLATE!(), width, height);
                //result += &format!(BBOX_TEMPLATE!(), 0, -gh, gw, gh-gd);
                if gly.scale != 1f64 {
                    result += &format!(SCALE_TEMPLATE!(), gly.scale);
                }

                result += &format!(SYM_TEMPLATE!(), ::std::char::from_u32(gly.unicode)
                    .expect("Unable to decode unicode!"));

                if gly.scale != 1f64 {
                    result += "</g>";
                }
                width += node.width;
            },
            LayoutVariant::Rule => {
                result += &format!(RULE_TEMPLATE!(),
                    width, height - node.height, node.width, node.height);
                width += node.width;
            },
            LayoutVariant::VerticalBox(ref vbox) => {
                result += &format!(G_TEMPLATE!(), width, height - node.height /*+ vbox.offset*/);
                result += &self.render_vbox(&vbox.contents);
                result += "</g>";
                width += node.width;
            },
            LayoutVariant::HorizontalBox(ref hbox) => {
                result += &format!(G_TEMPLATE!(), width, height - node.height);
                result += &self.render_hbox(&hbox.contents, node.height, node.width, hbox.alignment);
                result += "</g>";
                width += node.width;
            },
            LayoutVariant::Kern =>
                width += node.width,
        }}

        result
    }

    pub fn render_vbox(&self, nodes: &[LayoutNode]) -> String {
        let mut result = String::new();

        let mut height = Pixels(0.0);
        let width      = Pixels(0.0);

        for node in nodes { match node.node {
            LayoutVariant::Rule => {
                result += &format!(RULE_TEMPLATE!(),
                    width, height,
                    node.width, node.height);
                height += node.height;
            },
            LayoutVariant::HorizontalBox(ref hbox) => {
                result += &format!(G_TEMPLATE!(), width, height);
                result += &self.render_hbox(&hbox.contents, node.height, node.width, hbox.alignment);
                result += "</g>";
                height += node.height;
            },
            LayoutVariant::VerticalBox(ref vbox) => {
                result += &format!(G_TEMPLATE!(), width, height /*+ vbox.offset*/);
                result += &self.render_vbox(&vbox.contents);
                result += "</g>";
                height += node.height;
            },
            LayoutVariant::Kern =>
                height += node.height,
            LayoutVariant::Glyph(ref gly) => {
                result += &format!(G_TEMPLATE!(), width, height + node.height);

                if gly.scale != 1f64 {
                    result += &format!(SCALE_TEMPLATE!(), gly.scale);
                }

                result += &format!(SYM_TEMPLATE!(), ::std::char::from_u32(gly.unicode)
                    .expect("Unable to decode unicode!"));

                if gly.scale != 1f64 {
                    result += "</g>";
                }

                height += node.height;
            },
            //_ => (),
        }}

        result
   }
}