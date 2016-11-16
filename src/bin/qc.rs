use std::fs::File;
use std::io::Write;
use std::env;

extern crate rex;

use rex::parser::parse;
use rex::render::render;
use rex::layout::reduce::reduce;

fn main() {
    let input = env::args().skip(2).collect::<String>();
    if input.len() == 0 {
        println!("Provide a TeX argument");
        return
    }

    let mut p = parse(&input).unwrap();
    println!("Parse: {:?}", p);
    let r = reduce(&mut p);
    println!("Reduce: {:?}", r);
    
    let output = render(r);
    let mut f = File::create("test.svg").unwrap();
    f.write_all(output.as_bytes()).unwrap();
}