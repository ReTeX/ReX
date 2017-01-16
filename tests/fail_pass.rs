extern crate rex;
extern crate json;

use json::JsonValue;
use rex::*;
use std::fs::File;
use std::io::Read;

#[test]
fn pass() {
    let mut string = String::new();
    File::open("tests/pass.json").unwrap().read_to_string(&mut string).unwrap();
    
    if let JsonValue::Array(examples) =
        json::parse(&string)
        .expect("failed to parse examples.json")
    {
        for case in &examples {
            let tex = case.as_str().unwrap();
            println!("test: {}", tex);
            
            let output: String =
                SVGRenderer::new(&RenderSettings::default()).render(tex).unwrap();
            drop(output);
        }
    }
}
