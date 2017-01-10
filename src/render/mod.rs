use dimensions::{Pixels, Float};
use layout::{LayoutNode, LayoutVariant, Alignment, Style, LayoutSettings};
use parser::parse;
use layout::engine::layout;

#[derive(Clone)]
pub struct RenderSettings {
    pub font_size:    Float,
    pub font_src:     String,
    pub horz_padding: Float,
    pub vert_padding: Float,
    pub strict:       bool,
    pub style:        Style,
    pub debug:        bool
}

impl Default for RenderSettings {
    fn default() -> Self {
        RenderSettings {
            font_size:    48.0,
            font_src:     "http://rex.breeden.cc/rex-xits.otf".into(),
            horz_padding: 12.0,
            vert_padding: 5.0,
            strict:       true,
            style:        Style::Display,
            debug:        false
        }
    }
}

impl RenderSettings {
    pub fn font_size(self, size: Float) -> Self {
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
    
    pub fn horz_padding(self, size: Float) -> RenderSettings {
        RenderSettings {
            horz_padding: size,
            ..self
        }
    }

    pub fn vert_padding(self, size: Float) -> RenderSettings {
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
    fn g<F>(&mut self, off_x: Pixels, off_y: Pixels, contents: F)
    where F: FnMut(&mut Self);

    fn bbox(&mut self, width: Pixels, height: Pixels);

    fn symbol(&mut self, symbol: u32, scale: Float);
    
    fn rule(&mut self, x: Pixels, y: Pixels, width: Pixels, height: Pixels);

    fn color<F>(&mut self, color: &str, contents: F)
    where F: FnMut(&mut Self);
    
    fn render_hbox(&mut self,
        nodes: &[LayoutNode],
        height: Pixels,
        nodes_width: Pixels,
        alignment: Alignment)
    {
        let mut width = Pixels(0.0);

        if let Alignment::Centered(w) = alignment {
            width += (nodes_width - w)/2.0;
        }

        self.bbox(nodes_width, height);

        for node in nodes {
            match node.node {
                LayoutVariant::Glyph(ref gly) =>
                    self.g(width, height, |r| r.symbol(gly.unicode, gly.scale)),

                LayoutVariant::Rule =>
                    self.rule(
                        width, height - node.height,
                        node.width, node.height
                    ),

                LayoutVariant::VerticalBox(ref vbox) =>
                    self.g(width, height - node.height, |r| r.render_vbox(&vbox.contents)),

                LayoutVariant::HorizontalBox(ref hbox) =>
                    self.g(width, height - node.height, |r| {
                        r.render_hbox(
                            &hbox.contents, node.height,
                            node.width, hbox.alignment
                        )
                    }),

                LayoutVariant::Color(ref clr) =>
                    self.color(&clr.color, |r| {
                        r.render_hbox(&clr.inner,
                            node.height, node.width, Alignment::Default
                        );
                    }),

                LayoutVariant::Kern => { }
            } // End macth

            width += node.width;
        }
    }

    fn render_vbox(&mut self, nodes: &[LayoutNode]) {
        let mut height = Pixels(0.0);
        let width      = Pixels(0.0);

        for node in nodes {
            match node.node {
                LayoutVariant::Rule =>
                    self.rule(width, height, node.width, node.height),

                LayoutVariant::HorizontalBox(ref hbox) =>
                    self.g(width, height, |r| {
                        r.render_hbox(
                            &hbox.contents, node.height,
                            node.width, hbox.alignment
                        )
                    }),

                LayoutVariant::VerticalBox(ref vbox) =>
                    self.g(width, height, |r| r.render_vbox(&vbox.contents)),

                LayoutVariant::Glyph(ref gly) =>
                    self.g(width, height + node.height, |r| {
                        r.symbol(gly.unicode, gly.scale)
                    }),

                LayoutVariant::Color(_) =>
                    panic!("Shouldn't have a color in a vertical box???"),

                LayoutVariant::Kern => { }
            }

            height += node.height;
        }
    }
    
    fn prepare(&mut self, _width: Pixels, _height: Pixels) {}
    fn finish(&mut self) {}
    fn settings(&self) -> &RenderSettings;
    
    fn render(&mut self, tex: &str) {
        let mut parse = match parse(&tex) {
                Ok(res)  => res,
                Err(err) => {
                    println!("Error -- {}", err);
                    return;
                }
            };

        let layout = layout(&mut parse, self.settings().layout_settings());

        if self.settings().debug {
            println!("Parse: {:?}\n", parse);
            println!("Layout: {:?}", layout);
        }
        
        let padding = (
            self.settings().horz_padding,
            self.settings().vert_padding
        );
        
        self.prepare(
            // Left and right padding
            layout.width  + 2.0 * padding.0,
            // Top and bot padding
            layout.height + 2.0 * padding.1 - layout.depth
        );

        let x = Pixels(padding.0);
        let y = Pixels(padding.1);
        self.g(x, y, |r| {
            r.render_hbox(
                &layout.contents, layout.height,
                layout.width, Alignment::Default
            )
        });
        
        self.finish();
    }
}

pub mod svg;
pub use self::svg::SVGRenderer;
