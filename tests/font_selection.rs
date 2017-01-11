#![allow(non_upper_case_globals)]
extern crate rex;

const latin: &'static str = "abcdefghijklmnopqrstuv";
const LATIN: &'static str = "ABCDEFGHIJKLMNOPQRSTUV";
const digit: &'static str = "1234567890";
const greek: &'static str =
    "\\alpha\\beta\\gamma\\delta\\epsilon\\varepsilon\\zeta\
     \\zeta\\eta\\theta\\vartheta\\iota\\kappa\\lambda\\mu\\nu\
     \\xi\\phi\\rho\\varrho\\sigma\\tau\\upsilon\\phi\\varphi\\chi\\psi\\omega";
const GREEK: &'static str =
    "\\Alpha\\Beta\\Gamme\\Delta\\Epsilon\\Zeta\\Eta\\Theta\\Iota\\Kappa\
     \\Lambda\\Mu\\Nu\\Pi\\Rho\\Sigma\\Tau\\Upsilon\\Phi\\Chi\\Psi\\Omega";
const other: &'static str = "\\nabla\\partial";

static styles: [&'static str; 14] = [
    r"\mathrm",
    r"\mathbb",
    r"\mathit",
    r"\mathbb{\mathit",
    r"\mathscr",
    r"\mathbb{\mathscr",
    r"\mathfrak",
    r"\mathbb{\mathfrak",
    r"\mathcal",
    r"\mathsf",
    r"\mathbbsf",
    r"\mathitsf",
    r"\mathbbitsf",
    r"\mathtt",
];

#[test]
fn font_selection_render() {
    let svg = rex::SVGRenderer::new().font_src("../../rex-xits.woff2").font_size(32.0);

    for &style in styles.iter() {
        svg.render_to_file(filename(style, "latin"), &tex(style, latin));
        svg.render_to_file(filename(style, "LATIN"), &tex(style, LATIN));
        svg.render_to_file(filename(style, "greek"), &tex(style, greek));
        svg.render_to_file(filename(style, "GREEK"), &tex(style, GREEK));
        svg.render_to_file(filename(style, "digit"), &tex(style, digit));
        svg.render_to_file(filename(style, "other"), &tex(style, other));
    }
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

fn filename(style: &str, class: &str) -> String {
    // \mathrm -> mathrm.svg
    // \mathrm{\mathit -> mathrm_mathit.svg
    let out = format!("tests/out/{}_{}.svg",
        style.replace(r"\", "").replace("{", "_"),
        class);

    println!("{}", out);
    out
}