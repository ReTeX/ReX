#![allow(non_upper_case_globals)]
extern crate rex;
extern crate font_types as font;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate bincode;

use rex::Renderer;

mod common;
use common::debug_render::DebugRenderer;
use common::debug_render::Equation;
use common::debug_render::Object;
use common::svg_diff;

static STYLE_HTML: &str = "tests/out/style.html";
static STYLE_BINCODE: &str = "tests/data/style.bincode";

const LATIN_LOWER: &'static str = "abcdefghijklmnopqrstuvwxyz";
const LATIN_UPPER: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGIT: &'static str = "1234567890";
const OTHER: &'static str = "\\nabla\\partial";
const GREEK_LOWER: &'static str = "\\alpha\\beta\\gamma\\delta\\epsilon\\varepsilon\\zeta\
     \\zeta\\eta\\theta\\vartheta\\iota\\kappa\\lambda\\mu\\nu\
     \\xi\\phi\\rho\\varrho\\sigma\\tau\\upsilon\\phi\\varphi\\chi\\psi\\omega";
const GREEK_UPPER: &'static str = "\\Alpha\\Beta\\Gamma\\Delta\\Epsilon\\Zeta\\Eta\\Theta\\Iota\\Kappa\
     \\Lambda\\Mu\\Nu\\Pi\\Rho\\Sigma\\Tau\\Upsilon\\Phi\\Chi\\Psi\\Omega";

static CONTENTS: &[&'static str] = &[LATIN_LOWER,
                                     LATIN_UPPER,
                                     DIGIT,
                                     OTHER,
                                     GREEK_LOWER,
                                     GREEK_UPPER];
static WEIGHTS: &[&'static str] = &[r"", r"\mathit{", r"\mathbf{"];
static STYLES: &[&'static str] = &[r"",
                                   r"\mathrm{",
                                   r"\mathscr{",
                                   r"\mathfrak{",
                                   r"\mathsf{",
                                   r"\mathbb{",
                                   r"\mathtt{"];

fn tex_style(weight: &str, style: &str, content: &str) -> String {
    // Calculate worst case size
    let size = weight.len() + style.len() + content.len() + 4;
    let mut result = String::with_capacity(size);


    if weight != "" {
        result += weight;
    }

    if style != "" {
        result += style;
    }

    result += content;

    if weight != "" {
        result += "}";
    }

    if style != "" {
        result += "}";
    }

    result
}

fn render_styles() -> Vec<Equation> {
    // The number of combinations for:
    //  - Upto one weight and upto one sytyle.
    //  - Two weights and upto one style.
    let size = (WEIGHTS.len() + 1) * STYLES.len() * CONTENTS.len();
    let mut vec: Vec<Equation> = Vec::with_capacity(size);
    let renderer = DebugRenderer::default();

    for weight in WEIGHTS {
        for style in STYLES {
            for content in CONTENTS {
                let tex = tex_style(weight, style, content);
                let mut canvas: Vec<Object> = Vec::new();
                renderer
                    .render_to(&mut canvas, &tex)
                    .expect("failed to parse tex");
                vec.push(Equation {
                             tex: tex,
                             description: "".to_string(),
                             width: renderer.width.take(),
                             height: renderer.height.take(),
                             render: canvas,
                         });
            }
        }
    }

    vec
}

#[test]
fn style() {
    let rendered = render_styles();
    let bincode = common::load_bincode(STYLE_BINCODE);
    let diff = common::equation_diffs(&bincode, &rendered)
        .expect("detected a change in the number of style tests");

    if diff.len() != 0 {
        let count = diff.len();
        svg_diff::write_diff(STYLE_HTML, diff);
        panic!("Detected {} formula changes. \
                Please review the changes in `{}`",
               count,
               STYLE_HTML);
    }
}

#[test]
#[ignore]
fn save_style() {
    use std::fs::File;
    use std::io::BufWriter;
    use common::svg;

    let rendered = render_styles();
    let out = File::create(STYLE_BINCODE).expect("failed to create bincode file for style tests");
    let mut writer = BufWriter::new(out);
    bincode::serialize_into(&mut writer, &rendered, bincode::Infinite)
        .expect("failed to serialize tex results to bincode");

    // With new tests, create a new html
    svg::write(STYLE_HTML, &rendered);
}