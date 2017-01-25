use dimensions::{Pixels, Float};
use layout::{LayoutNode, LayoutVariant, Alignment, Style, LayoutSettings};
use parser::parse;
use layout::engine::layout;

#[derive(Clone)]
pub struct RenderSettings {
    pub font_size:    u16,
    pub font_src:     String,
    pub horz_padding: Pixels,
    pub vert_padding: Pixels,
    pub strict:       bool,
    pub style:        Style,
    pub debug:        bool
}

#[derive(Copy, Clone, Default)]
pub struct Cursor {
    pub x:  Pixels,
    pub y:  Pixels
}
impl Cursor {
    pub fn translate(self, dx: Pixels, dy: Pixels) -> Cursor {
        Cursor {
            x:  self.x + dx,
            y:  self.y + dy
        }
    }
    pub fn left(self, dx: Pixels) -> Cursor {
        Cursor {
            x:  self.x - dx,
            y:  self.y
        }
    }
    pub fn right(self, dx: Pixels) -> Cursor {
        Cursor {
            x:  self.x + dx,
            y:  self.y
        }
    }
    pub fn up(self, dy: Pixels) -> Cursor {
        Cursor {
            x:  self.x,
            y:  self.y - dy
        }
    }
    pub fn down(self, dy: Pixels) -> Cursor {
        Cursor {
            x:  self.x,
            y:  self.y + dy
        }
    }
}

impl Default for RenderSettings {
    fn default() -> Self {
        RenderSettings {
            font_size:    48,
            font_src:     "http://rex.breeden.cc/rex-xits.otf".into(),
            horz_padding: Pixels(12.0),
            vert_padding: Pixels(5.0),
            strict:       true,
            style:        Style::Display,
            debug:        false
        }
    }
}

impl RenderSettings {
    pub fn font_size(self, size: u16) -> Self {
        RenderSettings {
            font_size: size,
            ..self
        }
    }

    pub fn font_src(self, src: &str) -> Self {
        RenderSettings {
            font_src: src.into(),
            ..self
        }
    }

    pub fn horz_padding(self, size: Pixels) -> RenderSettings {
        RenderSettings {
            horz_padding: size,
            ..self
        }
    }

    pub fn vert_padding(self, size: Pixels) -> RenderSettings {
        RenderSettings {
            vert_padding: size,
            ..self
        }
    }

    pub fn style(self, style: Style) -> RenderSettings {
        RenderSettings {
            style: style,
            ..self
        }
    }

    pub fn debug(self, debug: bool) -> RenderSettings {
        RenderSettings {
            debug: debug,
            ..self
        }
    }

    pub fn layout_settings(&self) -> LayoutSettings {
        LayoutSettings {
            font_size: self.font_size,
            style:     self.style
        }
    }
}

pub trait Renderer {
    type Out;

    fn bbox(&self, _out: &mut Self::Out, _pos: Cursor, _width: Pixels, _height: Pixels) {}

    fn symbol(&self, out: &mut Self::Out, pos: Cursor, symbol: u32, scale: Float);

    fn rule(&self, out: &mut Self::Out, pos: Cursor, width: Pixels, height: Pixels);

    fn color<F>(&self, out: &mut Self::Out, color: &str, contents: F)
    where F: FnMut(&Self, &mut Self::Out);

    fn render_hbox(&self,
        out: &mut Self::Out,
        mut pos: Cursor,
        nodes: &[LayoutNode],
        height: Pixels,
        nodes_width: Pixels,
        alignment: Alignment)
    {
        if let Alignment::Centered(w) = alignment {
            pos.x += (nodes_width - w)/2.0;
        }

        self.bbox(out, pos, nodes_width, height);

        for node in nodes {
            match node.node {
                LayoutVariant::Glyph(ref gly) =>
                    self.symbol(out, pos, gly.unicode, gly.scale),

                LayoutVariant::Rule =>
                    self.rule(out,
                        pos.up(node.height),
                        node.width, node.height
                    ),

                LayoutVariant::VerticalBox(ref vbox) =>
                    self.render_vbox(out, pos.up(node.height), &vbox.contents),

                LayoutVariant::HorizontalBox(ref hbox) =>
                    self.render_hbox(out, pos,
                        &hbox.contents, node.height,
                        node.width, hbox.alignment
                    ),

                LayoutVariant::Color(ref clr) =>
                    self.color(out, &clr.color, |r, out| {
                        r.render_hbox(out, pos, &clr.inner,
                            node.height, node.width, Alignment::Default
                        );
                    }),

                LayoutVariant::Kern => { }
            } // End macth

            pos.x += node.width;
        }
    }

    fn render_vbox(&self, out: &mut Self::Out, mut pos: Cursor, nodes: &[LayoutNode]) {
        for node in nodes {
            match node.node {
                LayoutVariant::Rule =>
                    self.rule(out, pos, node.width, node.height),

                LayoutVariant::HorizontalBox(ref hbox) =>
                    self.render_hbox(out, pos.down(node.height),
                        &hbox.contents, node.height,
                        node.width, hbox.alignment
                    ),

                LayoutVariant::VerticalBox(ref vbox) =>
                    self.render_vbox(out, pos, &vbox.contents),

                LayoutVariant::Glyph(ref gly) =>
                    self.symbol(out, pos.down(node.height), gly.unicode, gly.scale),

                LayoutVariant::Color(_) =>
                    panic!("Shouldn't have a color in a vertical box???"),

                LayoutVariant::Kern => { }
            }

            pos.y += node.height;
        }
    }

    fn prepare(&self, _out: &mut Self::Out, _width: Pixels, _height: Pixels) {}
    fn finish(&self, _out: &mut Self::Out) {}
    fn settings(&self) -> &RenderSettings;

    fn render_to(&self, out: &mut Self::Out, tex: &str) -> Result<(), String> {
        let mut parse = parse(&tex)?;

        let layout = layout(&mut parse, self.settings().layout_settings());

        if self.settings().debug {
            println!("Parse: {:?}\n", parse);
            println!("Layout: {:?}", layout);
        }

        let padding = (
            self.settings().horz_padding,
            self.settings().vert_padding
        );

        self.prepare(out,
            // Left and right padding
            layout.width  + 2.0 * padding.0,
            // Top and bot padding
            layout.height + 2.0 * padding.1 - layout.depth
        );

        let pos = Cursor {
            x: Pixels(padding.0),
            y: Pixels(padding.1) + layout.height
        };
        self.render_hbox(out, pos,
            &layout.contents, layout.height,
            layout.width, Alignment::Default
        );

        self.finish(out);
        Ok(())
    }

    fn render(&self, tex: &str) -> Result<Self::Out, String> where Self::Out: Default {
        let mut out = Self::Out::default();
        self.render_to(&mut out, tex)?;
        Ok(out)
    }
}

pub mod svg;
pub use self::svg::SVGRenderer;
