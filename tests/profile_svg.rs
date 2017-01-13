#![feature(test)]

extern crate rex;
extern crate json;
extern crate test;

use json::JsonValue;
use rex::*;

#[test]
fn svg() {
    println!("SVG Renderer");
    if let JsonValue::Array(examples) =
        json::parse(include_str!("examples.json"))
        .expect("failed to parse examples.json")
    {
        for case in &examples {
            let title = case["title"].as_str().unwrap();
            let tex = case["latex"].as_str().expect("'tex' is not a string");
            
            let samples = test::bench::benchmark(|b| {
                b.iter(move || -> String {
                    SVGRenderer::new(&RenderSettings::default()).render(tex)
                });
            });
            println!("{:50} {}", title, test::fmt_bench_samples(&samples));
        }
    }
}
