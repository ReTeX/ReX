pub mod svg;
pub use self::svg::SVGRenderer;

use font::FontUnit;
use dimensions::Float;
use layout::{LayoutNode, LayoutVariant, Alignment, Style, LayoutSettings};
use parser::parse;
use parser::color::RGBA;
use layout::engine::layout;

#[derive(Clone)]
pub struct RenderSettings {
    pub font_size: u16,
    pub font_src: String,
    pub horz_padding: FontUnit,
    pub vert_padding: FontUnit,
    pub strict: bool,
    pub style: Style,
    pub debug: bool,
}

#[derive(Copy, Clone, Default)]
pub struct Cursor {
    pub x: FontUnit,
    pub y: FontUnit,
}
impl Cursor {
    pub fn translate(self, dx: FontUnit, dy: FontUnit) -> Cursor {
        Cursor {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
    pub fn left(self, dx: FontUnit) -> Cursor {
        Cursor {
            x: self.x - dx,
            y: self.y,
        }
    }
    pub fn right(self, dx: FontUnit) -> Cursor {
        Cursor {
            x: self.x + dx,
            y: self.y,
        }
    }
    pub fn up(self, dy: FontUnit) -> Cursor {
        Cursor {
            x: self.x,
            y: self.y - dy,
        }
    }
    pub fn down(self, dy: FontUnit) -> Cursor {
        Cursor {
            x: self.x,
            y: self.y + dy,
        }
    }
}

impl Default for RenderSettings {
    fn default() -> Self {
        RenderSettings {
            font_size: 48,
            font_src: "http://rex.breeden.cc/rex-xits.otf".into(),

            // TODO: Think about these defaults.  The 0.250, and 0.100
            // were taken from a default font_size of 48, and a
            // UNITS_PER_EM of 1000.  This should probably depend on
            // UNITS_PER_EM.  Maybe UNITS_PER_EM/4, and UNITS_PER_EM/10?
            horz_padding: FontUnit::from(250),
            vert_padding: FontUnit::from(100),
            strict: true,
            style: Style::Display,
            debug: false,
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

    pub fn horz_padding(self, size: FontUnit) -> RenderSettings {
        RenderSettings {
            horz_padding: size,
            ..self
        }
    }

    pub fn vert_padding(self, size: FontUnit) -> RenderSettings {
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
            style: self.style,
        }
    }
}

pub trait Renderer {
    type Out;

    fn bbox(&self, _out: &mut Self::Out, _pos: Cursor, _width: FontUnit, _height: FontUnit) {}

    fn symbol(&self, out: &mut Self::Out, pos: Cursor, symbol: u32, scale: Float);

    fn rule(&self, out: &mut Self::Out, pos: Cursor, width: FontUnit, height: FontUnit);

    fn color<F>(&self, out: &mut Self::Out, color: RGBA, contents: F)
        where F: FnMut(&Self, &mut Self::Out);

    fn render_hbox(&self,
                   out: &mut Self::Out,
                   mut pos: Cursor,
                   nodes: &[LayoutNode],
                   height: FontUnit,
                   nodes_width: FontUnit,
                   alignment: Alignment) {
        if let Alignment::Centered(w) = alignment {
            pos.x += (nodes_width - w) / 2;
        }

        self.bbox(out, pos, nodes_width, height);

        for node in nodes {
            match node.node {
                LayoutVariant::Glyph(ref gly) => {
                    self.symbol(out, pos, gly.unicode, f64::from(gly.scale))
                }

                LayoutVariant::Rule => self.rule(out, pos.up(node.height), node.width, node.height),

                LayoutVariant::VerticalBox(ref vbox) => {
                    self.render_vbox(out, pos.up(node.height), &vbox.contents)
                }

                LayoutVariant::HorizontalBox(ref hbox) => {
                    self.render_hbox(out,
                                     pos,
                                     &hbox.contents,
                                     node.height,
                                     node.width,
                                     hbox.alignment)
                }

                LayoutVariant::Color(ref clr) => {
                    self.color(out, clr.color, |r, out| {
                        r.render_hbox(out,
                                      pos,
                                      &clr.inner,
                                      node.height,
                                      node.width,
                                      Alignment::Default);
                    })
                }

                LayoutVariant::Kern => { /* NOOP */ }
            } // End macth

            pos.x += node.width;
        }
    }

    fn render_vbox(&self, out: &mut Self::Out, mut pos: Cursor, nodes: &[LayoutNode]) {
        for node in nodes {
            match node.node {
                LayoutVariant::Rule => self.rule(out, pos, node.width, node.height),

                LayoutVariant::HorizontalBox(ref hbox) => {
                    self.render_hbox(out,
                                     pos.down(node.height),
                                     &hbox.contents,
                                     node.height,
                                     node.width,
                                     hbox.alignment)
                }

                LayoutVariant::VerticalBox(ref vbox) => self.render_vbox(out, pos, &vbox.contents),

                LayoutVariant::Glyph(ref gly) => {
                    self.symbol(out,
                                pos.down(node.height),
                                gly.unicode,
                                f64::from(gly.scale))
                }

                LayoutVariant::Color(_) => panic!("Shouldn't have a color in a vertical box???"),

                LayoutVariant::Kern => { /* NOOP */ }
            }

            pos.y += node.height;
        }
    }

    fn prepare(&self, _out: &mut Self::Out, _width: FontUnit, _height: FontUnit) {}
    fn finish(&self, _out: &mut Self::Out) {}
    fn settings(&self) -> &RenderSettings;

    fn render_to(&self, out: &mut Self::Out, tex: &str) -> Result<(), String> {
        let mut parse = match parse(&tex) {
            Ok(ret) => ret,
            Err(err) => panic!("failed to parse with: {}", err),
        };

        let layout = layout(&mut parse, self.settings().layout_settings());

        if self.settings().debug {
            println!("Parse: {:?}\n", parse);
            println!("Layout: {:?}", layout);
        }

        let padding = (self.settings().horz_padding, self.settings().vert_padding);

        self.prepare(out,
                     // Left and right padding
                     layout.width + 2 * padding.0,
                     // Top and bot padding
                     layout.height - layout.depth + 2 * padding.1);

        let pos = Cursor {
            x: padding.0,
            y: padding.1 + layout.height,
        };
        self.render_hbox(out,
                         pos,
                         &layout.contents,
                         layout.height,
                         layout.width,
                         Alignment::Default);

        self.finish(out);
        Ok(())
    }

    fn render(&self, tex: &str) -> Result<Self::Out, String>
        where Self::Out: Default
    {
        let mut out = Self::Out::default();
        self.render_to(&mut out, tex)?;
        Ok(out)
    }
}
