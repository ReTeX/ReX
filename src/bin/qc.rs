use std::fs::File;
use std::io::Write;
use std::env;

extern crate rex;

use rex::parser::parse;
use rex::render::Renderer;
use rex::layout::engine::layout;
use rex::layout::Style;

fn main() {
    let input = env::args().skip(1).collect::<String>();
    if input.len() == 0 {
        println!("Provide a TeX argument");
        return
    }

    let mut p = parse(&input).unwrap();
    //println!("Parse: {:?}", p);
    let r = layout(&mut p, Style::Display);
    println!("layout: {:#?}", r);

    let output = Renderer::new(r).render();
    let mut f = File::create("test.svg").unwrap();
    f.write_all(output.as_bytes()).unwrap();
}