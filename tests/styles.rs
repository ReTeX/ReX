#![allow(non_upper_case_globals)]
extern crate rex;
use rex::render::svg;

use std::fs::File;
use std::io::Write;

const latin: &'static str = "abcdefghijklmnopqrstuvwxyz";
const LATIN: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const digit: &'static str = "1234567890";
const greek: &'static str = "\\alpha\\beta\\gamma\\delta\\epsilon\\varepsilon\\zeta\
     \\zeta\\eta\\theta\\vartheta\\iota\\kappa\\lambda\\mu\\nu\
     \\xi\\phi\\rho\\varrho\\sigma\\tau\\upsilon\\phi\\varphi\\chi\\psi\\omega";
const GREEK: &'static str = "\\Alpha\\Beta\\Gamma\\Delta\\Epsilon\\Zeta\\Eta\\Theta\\Iota\\Kappa\
     \\Lambda\\Mu\\Nu\\Pi\\Rho\\Sigma\\Tau\\Upsilon\\Phi\\Chi\\Psi\\Omega";
const other: &'static str = "\\nabla\\partial";

static styles: [&'static str; 14] = [r"\mathrm",
                                     r"\mathbf",
                                     r"\mathit",
                                     r"\mathbf{\mathit",
                                     r"\mathscr",
                                     r"\mathbf{\mathscr",
                                     r"\mathfrak",
                                     r"\mathbf{\mathfrak",
                                     r"\mathcal",
                                     r"\mathsf",
                                     r"\mathbf{\mathsf",
                                     r"\mathit{\mathsf",
                                     r"\mathbf{\mathit{\mathsf",
                                     r"\mathtt"];

const HEADER: &'static str = r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Font Styles</title>
</head>
<body>
<h1><center>Font Styles Rendering Tests</center></h1>
"##;

const END: &'static str = r"</body></html>";

#[test]
fn font_styles() {
    let settings = rex::RenderSettings::default()
        .font_src("rex-xits.otf")
        .font_size(32);

    let mut file = File::create("tests/out/styles.html").expect("Unable to create `styles.html`");
    let mut result = String::from(HEADER);

    for &style in styles.iter() {
        result += &format!("<h2><center>{}</center></h2>\n\
             <center>{}</center>\n\
             <center>{}</center>\n\
             <center>{}</center>\n\
             <center>{}</center>\n\
             <center>{}</center>\n\
             <center>{}</center>\n",
                style,
                svg::render_to_string(&settings, &tex(style, latin)).unwrap(),
                svg::render_to_string(&settings, &tex(style, LATIN)).unwrap(),
                svg::render_to_string(&settings, &tex(style, greek)).unwrap(),
                svg::render_to_string(&settings, &tex(style, GREEK)).unwrap(),
                svg::render_to_string(&settings, &tex(style, digit)).unwrap(),
                svg::render_to_string(&settings, &tex(style, other)).unwrap());
    }

    result += END;
    file.write_all(&result.as_bytes())
        .expect("Unable to write to `font_styles.html`");
}

fn tex(style: &str, source: &str) -> String {
    // count the number of { and match
    let num = style.chars().filter(|&c| c == '{').count() + 1;
    let out = format!("{}{{{}{}",
                      style,
                      source,
                      (0..num).map(|_| "}").collect::<String>());
    println!("{}", out);
    out
}
