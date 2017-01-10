#![allow(dead_code)]
// use parser::nodes::{ ParseNode };
// use font::{GLYPHS};
// use spacing::atom_spacing;
use layout::Alignment;
//use layout::boundingbox::Bounded;
use dimensions::Pixels;
use parser::parse;
use layout::engine::layout;
use render::{Renderer, RenderSettings};
use std::fmt::Write;
use std::fs::File;
use std::path::Path;

const SVG_HEADER: &'static str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?><!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">"#;

macro_rules! debug {
    ($fmt:expr, $($arg:tt)*) => (
        if cfg!(debug_assertions) {
            println!($fmt, $($arg)*);
        }
    )
}

pub fn render_to_path<P: AsRef<Path>>(path: P, settings: &RenderSettings, input: &str) {
    render_to_file(
        &mut File::create(path.as_ref())
        .expect("could not create file"),
        settings, input
    );
}

pub fn render_to_file(file: &mut File, settings: &RenderSettings, input: &str) {
    use std::io::Write;
    
    let mut s = String::new();
    SVGRenderer::new(&mut s, settings).render(input);
    file.write(s.as_bytes()).expect("failed to write to file");
}

#[derive(Clone)]
pub struct SVGRenderer<'a, W: Write> {
    pub gzip:       bool,
    out:            W,
    settings:       &'a RenderSettings
}

impl<'a, W: Write> SVGRenderer<'a, W> {
    pub fn new(output: W, settings: &RenderSettings) -> SVGRenderer<W> {
        SVGRenderer {
            gzip:         false,
            settings:     settings,
            out:          output
        }
    }

    pub fn render(&mut self, tex: &str) {
        let mut parse = match parse(&tex) {
                Ok(res)  => res,
                Err(err) => {
                    println!("Error -- {}", err);
                    return;
                }
            };

        let layout = layout(&mut parse, self.settings.layout_settings());

        if self.settings.debug {
            println!("Parse: {:?}\n", parse);
            println!("Layout: {:?}", layout);
        }

        self.out.write_str(SVG_HEADER).expect("failed to write");

        let width  = layout.width  + 2.0 * self.settings.horz_padding;   // Left and right padding
        let height = layout.height + 2.0 * self.settings.vert_padding;   // Top and bot padding
        let depth  = layout.depth;


        self.header(width, height -depth);
        
        let width = Pixels(self.settings.horz_padding);
        let height = Pixels(self.settings.vert_padding);
        self.g(width, height, |r| {
            r.render_hbox(
                &layout.contents, layout.height,
                layout.width, Alignment::Default
            )
        });
        writeln!(self.out, "</g></svg>").unwrap();
    }
    
    fn header(&mut self, width: Pixels, height: Pixels) {
        write!(self.out,
    r#"<svg width="{:.2}" height="{:.2}" xmlns="http://www.w3.org/2000/svg">
    <defs>
    <style type="text/css">@font-face{{font-family:rex;src:url('{}');}}</style>
    </defs>
    <g font-family="rex" font-size="{:.1}px">"#,
            width, height, self.settings.font_src, self.settings.font_size
        ).expect("Failed to write to buffer!");
    }
}

#[derive(Clone, Copy)]
struct Cursor {
    x: f64,
    y: f64,
}


impl<'a, W: Write> Renderer for SVGRenderer<'a, W> {
    fn g<F>(&mut self, width: Pixels, height: Pixels, mut contents: F)
        where F: FnMut(&mut Self)
    {
        if width == Pixels(0.0) && height == Pixels(0.0) {
            contents(self);
        } else {
            write!(self.out,
                r#"<g transform="translate({:.2} {:.2})">"#,
                width, height
            ).expect("Failed to write to buffer!");

            contents(self);
            write!(self.out, "</g>")
            .expect("Failed to write to buffer!");
        }
    }

    fn bbox(&mut self, width: Pixels, height: Pixels) {
        if self.settings.debug {
            write!(self.out,
                r#"<rect width="{}" height="{}" fill="none" stroke="blue" stroke-width="0.2"/>"#,
                width, height
            ).expect("Failed to write to buffer!");
        }
    }

    fn symbol(&mut self, symbol: u32, scale: f64) {
        use std::char;
        if scale != 1f64 {
            write!(self.out,
                r#"<text transform="scale({:.2})">{}</text>"#,
                scale,
                char::from_u32(symbol).expect("Unabale to decode utf8 code-point!")
            ).expect("Failed to write to buffer!");
        } else {
            write!(self.out,
                r#"<text>{}</text>"#,
                char::from_u32(symbol).expect("Unabale to decode utf8 code-point!")
            ).expect("Failed to write to buffer!");
        }
    }

    fn rule(&mut self, x: Pixels, y: Pixels, width: Pixels, height: Pixels) {
        write!(self.out,
            r##"<rect x="{}" y ="{}" width="{}" height="{}" fill="#000"/>"##,
            x, y, width, height
        ).expect("Failed to write to buffer!");
    }

    fn color<F>(&mut self, color: &str, mut contents: F)
        where F: FnMut(&mut Self)
    {
        write!(self.out, r#"<g fill="{}">"#, color)
        .expect("Failed to write to buffer!");
        contents(self);
        write!(self.out, "</g>")
        .expect("Failed to write to buffer!");
    }
}
