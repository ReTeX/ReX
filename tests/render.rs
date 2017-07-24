#![feature(test)]
extern crate test;
use test::Bencher;

extern crate rex;
extern crate font_types as font;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::convert::AsRef;
use std::path::Path;
use std::collections::BTreeMap;

use rex::Renderer;

mod common;
use common::debug_render::DebugRenderer;
use common::debug_render::Equation;
use common::debug_render::Object;

// Parseing the `render.md` test collection.
// - Categories start with a #.  ie: `# Scripts`.
// - Descriptions start with a number followed by a period.
// - Each test begins with a `>`.

#[derive(Debug, Serialize, Deserialize)]
struct Tests(BTreeMap<String, Vec<Category>>);

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Snippets")]
    snippets: Vec<String>,
}

fn collect_tests<P: AsRef<Path>>(path: P) -> Tests {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open(path.as_ref())
        .expect("failed to open test collection");
    let reader = BufReader::new(file);
    let tests: Tests = serde_yaml::from_reader(reader)
        .expect("failed to parse test collection");

    tests
}

#[test]
fn render() {
    let tests = test::black_box(collect_tests("tests/data/render.yaml"));
    let renderer = DebugRenderer::default();
    let mut results: Vec<Equation> = Vec::new();
    for (category, collection) in tests.0.iter() {
        for snippets in collection {
            for equation in &snippets.snippets {
                let mut render: Vec<Object> = Vec::new();
                renderer.render_to(&mut render, equation)
                    .expect("failed to parse tex");
                let description = format!("{}: {}", category, snippets.description);
                results.push(
                    Equation {
                        tex: equation.to_string(),
                        description: description,
                        render: render,
                    });
            }
        }
    }
}