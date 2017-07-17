extern crate rex;

use rex::*;

fn main() {
    let input = std::env::args()
        .skip(1)
        .next()
        .expect("must provide an argument!");

    let font = "rex-xits.otf";
    let output = "test.svg";

    let settings = RenderSettings::default()
        .font_size(96)
        .font_src("rex-xits.otf")
        .debug(false);

    svg::render_to_path(output, &settings, &input);
}