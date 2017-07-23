use dimensions::Float;
use font::FontUnit;
use font::constants::UNITS_PER_EM;
use parser::color::RGBA;
use render::{Renderer, RenderSettings, Cursor};
use std::fmt::Write;
use std::fs::File;
use std::path::Path;
use std::marker::PhantomData;

pub fn render_to_path<P: AsRef<Path>>(path: P, settings: &RenderSettings, input: &str) {
    render_to_file(&mut File::create(path.as_ref()).expect("could not create file"),
                   settings,
                   input);
}

pub fn render_to_file(file: &mut File, settings: &RenderSettings, input: &str) {
    use std::io::Write;

    let s: String = SVGRenderer::new(&settings)
        .render(input)
        .expect("failed to render");
    file.write(s.as_bytes()).expect("failed to write to file");
}

#[derive(Clone)]
pub struct SVGRenderer<'a, W: Write> {
    settings: &'a RenderSettings,
    _marker: PhantomData<W>,
}

impl<'a, W: Write> SVGRenderer<'a, W> {
    pub fn new(settings: &RenderSettings) -> SVGRenderer<W> {
        SVGRenderer {
            settings: settings,
            _marker: PhantomData,
        }
    }
}

impl<'a, W: Write> Renderer for SVGRenderer<'a, W> {
    type Out = W;

    fn settings(&self) -> &RenderSettings {
        self.settings
    }

    fn prepare(&self, out: &mut W, width: FontUnit, height: FontUnit) {
        let px_width = f64::from(width) / f64::from(UNITS_PER_EM) * self.settings.font_size as f64;
        let px_height = f64::from(height) / f64::from(UNITS_PER_EM) * self.settings.font_size as f64;

        writeln!(out,
                 r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="{:2}" height="{:2}" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">
    <defs>
    <style type="text/css">@font-face{{font-family:rex;src:url('{}');}}</style>
    </defs>
    <g font-family="rex" font-size="{}">"#,
                 px_width,
                 px_height,
                 width,
                 height,
                 self.settings.font_src,
                 f64::from(UNITS_PER_EM))
                .expect("Failed to write to buffer!");
    }

    fn finish(&self, out: &mut W) {
        writeln!(out,
                 "\
    </g>
</svg>
")
                .unwrap();
    }

    fn bbox(&self, out: &mut W, pos: Cursor, width: FontUnit, height: FontUnit) {
        if self.settings.debug {
            writeln!(out,
                r#"<rect x="{}", y="{}", width="{}" height="{}" fill="none" stroke="blue" stroke-width="0.2"/>"#,
                pos.x, pos.y,
                width, height
            ).expect("Failed to write to buffer!");
        }
    }

    fn symbol(&self, out: &mut W, pos: Cursor, symbol: u32, scale: Float) {
        use std::char;
        if scale != 1. {
            writeln!(out,
                     r#"<text transform="translate({}, {}) scale({:.2})">{}</text>"#,
                     pos.x,
                     pos.y,
                     scale,
                     char::from_u32(symbol).expect("Unabale to decode utf8 code-point!"))
                    .expect("Failed to write to buffer!");
        } else {
            writeln!(out,
                     r#"<text transform="translate({}, {})">{}</text>"#,
                     pos.x,
                     pos.y,
                     char::from_u32(symbol).expect("Unabale to decode utf8 code-point!"))
                    .expect("Failed to write to buffer!");
        }
    }

    fn rule(&self, out: &mut W, pos: Cursor, width: FontUnit, height: FontUnit) {
        writeln!(out,
                 r##"<rect x="{}" y ="{}" width="{}" height="{}" fill="#000"/>"##,
                 pos.x,
                 pos.y,
                 width,
                 height)
                .expect("Failed to write to buffer!");
    }

    fn color<F>(&self, out: &mut W, color: RGBA, mut contents: F)
        where F: FnMut(&Self, &mut W)
    {
        if color.has_alpha() {
            writeln!(out, r##"<g fill="#{}{}{}">"##, color.0, color.1, color.2)
                .expect("failed to write to buffer!");
        } else {
            writeln!(out, r#"<g fill="rgba({},{},{},{})">"#, color.0, color.1, color.2, color.3)
                .expect("Failed to write to buffer!");
        }
        contents(self, out);
        writeln!(out, "</g>").expect("Failed to write to buffer!");
    }
}
