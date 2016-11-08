use std::fs::File;
use std::io::Write;
use std::env;

extern crate rex;

use rex::parser::parse;
use rex::render::render;

fn main() {
    let input = env::args().skip(2).collect::<String>();
    if input.len() == 0 {
        println!("Provide a TeX argument");
        return
    }

    let output = render(parse(&input).unwrap());
    let mut f = File::create("test.svg").unwrap();
    f.write_all(output.as_bytes()).unwrap();
}