#![feature(test)]

extern crate rex;
extern crate json;
extern crate test;

use test::Bencher;
use json::JsonValue;

#[test]
fn main() {
    if let JsonValue::Array(examples) =
        json::parse(include_str!("examples.json"))
        .expect("failed to parse examples.json")
    {
        for case in &examples {
            let title = case["title"].as_str().unwrap();
            let tex = case["latex"].as_str().expect("'tex' is not a string");
            
            let samples = test::bench::benchmark(|b| {
                b.iter(move || rex::SVGRenderer::new().render(tex));
            });
            println!("{:50} {}", title, test::fmt_bench_samples(&samples));
        }
    }
}
