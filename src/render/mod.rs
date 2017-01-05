#![allow(dead_code)]
// use parser::nodes::{ ParseNode };
// use font::{GLYPHS};
// use spacing::atom_spacing;
use layout::{ LayoutNode, LayoutVariant, Alignment };
//use layout::boundingbox::Bounded;
use dimensions::Pixels;
use std::path::Path;
use std::convert::AsRef;

use parser::parse;
use layout::LayoutSettings;
use layout::engine::layout;
use layout::Style;

use std::fs::File;
use std::io::Write;

macro_rules! HEAD_TEMPLATE { () => { "<svg width=\"{:.2}\" height=\"{:.2}\" xmlns=\"http://www.w3.org/2000/svg\"><defs><style type=\"text/css\">@font-face{{font-family: rex;src: url('{}');}}</style></defs><g font-family=\"rex\" font-size=\"{:.1}px\">" } }
macro_rules! G_TEMPLATE { () => { "<g transform=\"translate({:.2},{:.2})\">\n" } }
macro_rules! BBOX_TEMPLATE { () => { "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"none\" stroke=\"blue\" stroke-width=\"0.2\"/>\n" } }
macro_rules! SYM_TEMPLATE { () => { "<text>{}</text></g>\n" } }
macro_rules! RULE_TEMPLATE { () => { r##"<rect x="{}" y="{}" width="{}" height="{}" fill="#000"/>"## } }
macro_rules! SCALE_TEMPLATE { () => { r#"<g transform="scale({})">"# } }
macro_rules! COLOR_TEMPLATE { () => { r#"<g transform="translate({:.2},{:.2})" fill="{}">"# } }

const SVG_HEADER: &'static str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?><!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">"#;

macro_rules! debug {
    ($fmt:expr, $($arg:tt)*) => (
        if cfg!(debug_assertions) {
            println!($fmt, $($arg)*);
        }
    )
}

#[derive(Copy, Clone)]
pub struct SVGRenderer {
    pub font_size:    f64,
    pub horz_padding: f64,
    pub vert_padding: f64,
    pub strict:       bool,
    pub gzip:         bool,
    pub style:        Style,
    pub debug:        bool,
    cursor:           Cursor,
}

impl SVGRenderer {
    pub fn new() -> SVGRenderer {
        SVGRenderer {
            font_size:    48.0,
            horz_padding: 12.0,
            vert_padding: 5.0,
            strict:       true,
            gzip:         false,
            style:        Style::Display,
            debug:        false,
            cursor:       Cursor { x: 0.0, y: 0.0 },
        }
    }

    pub fn font_size(self, size: f64) -> SVGRenderer {
        SVGRenderer {
            font_size: size,
            ..self
        }
    }

    pub fn horz_padding(self, size: f64) -> SVGRenderer {
        SVGRenderer {
            horz_padding: size,
            ..self
        }
    }

    pub fn vert_padding(self, size: f64) -> SVGRenderer {
        SVGRenderer {
            vert_padding: size,
            ..self
        }
    }

    pub fn style(self, style: Style) -> SVGRenderer {
        SVGRenderer {
            style: style,
            ..self
        }
    }

    pub fn debug(self, debug: bool) -> SVGRenderer {
        SVGRenderer {
            debug: debug,
            ..self
        }
    }

    pub fn render_to_file<P: AsRef<Path>>(&self, file: P, tex: &str) {
        let output = self.render(tex);

        let mut f = File::create(file).unwrap();
        f.write_all(output.as_bytes()).unwrap();
    }

    pub fn render(&self, tex: &str) -> String {
        let mut parse = match parse(&tex) {
                Ok(res)  => res,
                Err(err) => {
                    println!("Error -- {}", err);
                    return String::new();
                }
            };

        let layout = layout(&mut parse, self.layout_settings());

        if cfg!(debug_assertions) && self.debug {
            println!("Parse: {:?}\n", parse);
            println!("Layout: {:?}", layout);
        }

        let mut output = String::from(SVG_HEADER);

        let width  = layout.width  + 2.0 * self.horz_padding;   // Left and right padding
        let height = layout.height + 2.0 * self.vert_padding;   // Top and bot padding
        let depth  = layout.depth;

        output += &format!(HEAD_TEMPLATE!(), width, height - depth, "http://rex.breeden.cc/rex-xits.otf", self.font_size);
        output += &format!(G_TEMPLATE!(), self.horz_padding, self.vert_padding);

        output += &self.render_hbox(
            &layout.contents, layout.height, layout.width, Alignment::Default);
        output += "</g></g></svg>";
        output
    }

    pub fn render_hbox(&self, nodes: &[LayoutNode],
            height: Pixels, nodes_width: Pixels, alignment: Alignment) -> String {
        let mut result = String::new();
        let mut width = Pixels(0.0);

        if let Alignment::Centered(w) = alignment {
            width += (nodes_width - w)/2.0;
        }

        if cfg!(debug_assertions) && self.debug {
            result += &format!(BBOX_TEMPLATE!(), 0, 0, nodes_width, height);
        }

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
            LayoutVariant::Color(ref clr) => {
                result += &format!(COLOR_TEMPLATE!(), width, height - node.height, clr.color);
                result += &self.render_hbox(&clr.inner, node.height, node.width, Alignment::Default);
                result += "</g>";
                width += node.width;
            }
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
                result += &format!(G_TEMPLATE!(), width, height);
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
            LayoutVariant::Color(_) => {
                panic!("Shouldn't have a color in a vertical box???")
            }
            //_ => (),
        }}

        result
    }

    fn layout_settings(&self) -> LayoutSettings {
        LayoutSettings {
            font_size: self.font_size,
            style:     self.style,
        }
    }
}

#[derive(Clone, Copy)]
struct Cursor {
    x: f64,
    y: f64,
}