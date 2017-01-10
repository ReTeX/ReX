extern crate clap;
extern crate rex;

use clap::{Arg, App};
use rex::*;

fn main() {
    let matches = App::new("ReX Debugging Utitlity")
        .version("0.1a")
        .author("Christopher Breeden <github@u.breeden.cc>")
        .about("Typesetting mathematics with a T-Rex")
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Enable debugging; display bounding boxes and print internals representation.")
            .takes_value(false))
        .arg(Arg::with_name("otf")
            .long("otf")
            .help("Use otf font instead of default woff2 font")
            .takes_value(false))
        .arg(Arg::with_name("output")
            .long("out")
            .short("s")
            .help("Output file for render.  Defaults to test.svg")
            .takes_value(true))
        .arg(Arg::with_name("TEX")
            .help("Input TeX to render")
            .required(true)
            .index(1))
        .get_matches();

    let input = matches.value_of("TEX").unwrap();
    let debug = matches.is_present("debug");
    let font  = if matches.is_present("otf") {
            "rex-xits.otf"
        } else {
            "rex-xits.woff2"
        };

    let out = matches.value_of("output")
        .unwrap_or("test.svg");
    
    let settings = RenderSettings::default()
    .font_size(96.0)
    .font_src(font)
    .debug(debug);
    
    svg::render_to_path(out, &settings, &input);
}
