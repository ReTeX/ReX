extern crate log;
extern crate env_logger;
extern crate rex;

use rex::*;

fn main() {

    let input = std::env::args()
        .skip(1)
        .next()
        .expect("must provide an argument!");

    let font = "rex-xits.otf";
    let output = "test.svg";

    env_logger::init().unwrap();
    let settings = RenderSettings::default()
        .font_size(96)
        .font_src(font)
        .debug(true);

    svg::render_to_path(output, &settings, &input);
}