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
use std::fmt;

macro_rules! HEAD_TEMPLATE { () => { "<svg width=\"{:.2}\" height=\"{:.2}\" xmlns=\"http://www.w3.org/2000/svg\"><defs><style type=\"text/css\">@font-face{{font-family: rex;src: url('{}');}}</style></defs><g font-family=\"rex\" font-size=\"{:.1}px\">" } }
macro_rules! G_TEMPLATE { () => { "<g transform=\"translate({:.2},{:.2})\">\n" } }
macro_rules! BBOX_TEMPLATE { () => { "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"none\" stroke=\"blue\" stroke-width=\"0.2\"/>\n" } }
macro_rules! SYM_TEMPLATE { () => { "<text>{}</text></g>\n" } }
macro_rules! RULE_TEMPLATE { () => { r##"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" fill="#000"/>"## } }
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

#[derive(Clone)]
pub struct SVGRenderer {
    pub font_size:    f64,
    pub font_src:     String,
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
            font_src:     "http://rex.breeden.cc/rex-xits.otf".into(),
        }
    }

    pub fn font_src(self, src: &str) -> Self {
        SVGRenderer {
            font_src: src.into(),
            ..self
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


        header(&mut output, width, height -depth, &self.font_src, self.font_size);
        g(&mut output, Pixels(self.horz_padding), Pixels(self.vert_padding));
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
            bbox(&mut result, nodes_width, height);
        }

        for node in nodes { match node.node {
            LayoutVariant::Glyph(ref gly) => {
                g(&mut result, width, height);
                symbol(&mut result, gly.unicode, gly.scale);
                if width != Pixels(0.0) || height != Pixels(0.0) {
                    result += "</g>";
                }
                width += node.width;
            },
            LayoutVariant::Rule => {
                rule(&mut result, width, height - node.height, node.width, node.height);
                width += node.width;
            },
            LayoutVariant::VerticalBox(ref vbox) => {
                g(&mut result, width, height - node.height);
                result += &self.render_vbox(&vbox.contents);
                if width != Pixels(0.0) || (height - node.height) != Pixels(0.0) {
                    result += "</g>";
                }
                width += node.width;
            },
            LayoutVariant::HorizontalBox(ref hbox) => {
                g(&mut result, width, height - node.height);
                result += &self.render_hbox(&hbox.contents, node.height, node.width, hbox.alignment);
                if width != Pixels(0.0) || (height - node.height) != Pixels(0.0) {
                    result += "</g>";
                }
                width += node.width;
            },
            LayoutVariant::Kern =>
                width += node.width,
            LayoutVariant::Color(ref clr) => {
                color(&mut result, /*width, height - node.height,*/ &clr.color);
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
                rule(&mut result, width, height, node.width, node.height);
                height += node.height;
            },
            LayoutVariant::HorizontalBox(ref hbox) => {
                g(&mut result, width, height);
                result += &self.render_hbox(&hbox.contents, node.height, node.width, hbox.alignment);
                if width != Pixels(0.0) || height != Pixels(0.0) {
                    result += "</g>";
                }
                height += node.height;
            },
            LayoutVariant::VerticalBox(ref vbox) => {
                g(&mut result, width, height);
                result += &self.render_vbox(&vbox.contents);
                if width != Pixels(0.0) || height != Pixels(0.0) {
                    result += "</g>";
                }
                height += node.height;
            },
            LayoutVariant::Kern =>
                height += node.height,
            LayoutVariant::Glyph(ref gly) => {
                g(&mut result, width, height + node.height);
                symbol(&mut result, gly.unicode, gly.scale);
                if width != Pixels(0.0) || (height + node.height) != Pixels(0.0) {
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

fn header<W: fmt::Write>(w: &mut W, width: Pixels, height: Pixels, font: &str, font_size: f64) {
    w.write_fmt(format_args!(
r#"<svg width="{:.2}" height="{:.2}" xmlns="http://www.w3.org/2000/svg">
<defs>
  <style type="text/css">@font-face{{font-family:rex;src:url('{}');}}</style>
</defs>
<g font-family="rex" font-size="{:.1}px">"#,
        width, height, font, font_size))
        .expect("Failed to write to buffer!");
}

fn g<W: fmt::Write>(w: &mut W, width: Pixels, height: Pixels) {
    if width == Pixels(0.0) && height == Pixels(0.0) { return }
    w.write_fmt(format_args!(
        r#"<g transform="translate({:.2} {:.2})">"#,
        width, height))
        .expect("Failed to write to buffer!");
}

fn bbox<W: fmt::Write>(w: &mut W, width: Pixels, height: Pixels) {
    w.write_fmt(format_args!(
        r#"<rect width="{}" height="{}" fill="none" stroke="blue" stroke-width="0.2"/>"#,
        width, height))
        .expect("Failed to write to buffer!");
}

fn symbol<W: fmt::Write>(w: &mut W, symbol: u32, scale: f64) {
    use std::char;
    if scale != 1f64 {
        w.write_fmt(format_args!(
            r#"<text transform="scale({:.2})">{}</text>"#,
            scale,
            char::from_u32(symbol).expect("Unabale to decode utf8 code-point!")))
            .expect("Failed to write to buffer!");
    } else {
        w.write_fmt(format_args!(
            r#"<text>{}</text>"#,
            char::from_u32(symbol).expect("Unabale to decode utf8 code-point!")))
            .expect("Failed to write to buffer!");
    }
}

fn rule<W: fmt::Write>(w: &mut W, x: Pixels, y: Pixels, width: Pixels, height: Pixels) {
    w.write_fmt(format_args!(
        r##"<rect x="{}" y ="{}" width="{}" height="{}" fill="#000"/>"##,
        x, y, width, height))
        .expect("Failed to write to buffer!");
}

fn color<W: fmt::Write>(w: &mut W, color: &str) {
    w.write_fmt(format_args!(
        r#"<g fill="{}">"#, color))
        .expect("Failed to write to buffer!");
}