extern crate rex;
extern crate font_types as font;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate bincode;

use std::convert::AsRef;
use std::path::Path;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;

use rex::Renderer;

mod common;
use common::debug_render::DebugRenderer;
use common::debug_render::Equation;
use common::debug_render::Object;
use common::svg_diff;

const LAYOUT_YAML: &str = "tests/data/layout.yaml";
const LAYOUT_HTML: &str = "tests/out/layout.html";
const LAYOUT_BINCODE: &str = "tests/data/layout.bincode";

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
    let file = File::open(path.as_ref()).expect("failed to open test collection");
    let reader = BufReader::new(file);
    let tests: Tests = serde_yaml::from_reader(reader).expect("failed to parse test collection");

    tests
}

fn load_history<P: AsRef<Path>>(path: P) -> Vec<Equation> {
    let file = File::open(path.as_ref()).expect("failed to open test collection");
    let mut reader = BufReader::new(file);
    let tests: Vec<Equation> = bincode::deserialize_from(&mut reader, bincode::Infinite)
        .expect("failed to load historical test results");

    tests
}

fn render_tests(tests: Tests) -> Vec<Equation> {
    let renderer = DebugRenderer::default();
    let mut equations: Vec<Equation> = Vec::new();
    for (category, collection) in tests.0.iter() {
        for snippets in collection {
            for equation in &snippets.snippets {
                let mut canvas: Vec<Object> = Vec::new();
                renderer
                    .render_to(&mut canvas, equation)
                    .expect("failed to parse tex");
                let description = format!("{}: {}", category, snippets.description);
                equations.push(Equation {
                                   tex: equation.to_string(),
                                   description: description,
                                   width: renderer.width.take(),
                                   height: renderer.height.take(),
                                   render: canvas,
                               });
            }
        }
    }

    equations
}

fn equation_diffs(old: &[Equation], new: &[Equation]) -> Vec<(Equation, Equation)> {
    if old.len() != new.len() {
        panic!("Detected a change in the number of tests. Please be sure to run \
               `cargo test --test layout -- --ignored` to update the tests first.\n\
               Note: This should only be done before there are any changes which can alter \
               the result of a test.");
    }

    let mut diff: Vec<(Equation, Equation)> = Vec::new();
    for (left, right) in old.iter().zip(new.iter()) {
        if left != right {
            diff.push((left.clone(), right.clone()));
        }
    }

    diff
}

#[test]
fn layout() {
    let tests = collect_tests(LAYOUT_YAML);
    let rendered = render_tests(tests);
    let history = load_history(LAYOUT_BINCODE);
    let diff = equation_diffs(&history, &rendered);

    if diff.len() != 0 {
        let count = diff.len();
        svg_diff::write_diff(LAYOUT_HTML, diff);
        panic!("Detected {} formula changes. \
                Please review the changes in `{}`",
               count,
               LAYOUT_HTML);
    }
}

#[test]
#[ignore]
fn save_layout() {
    use std::io::BufWriter;
    use common::svg;

    // Load the tests in yaml, and render it to bincode
    let tests = collect_tests(LAYOUT_YAML);
    let rendered = render_tests(tests);

    let out = File::create(LAYOUT_BINCODE).expect("failed to create bincode file for layout tests");
    let mut writer = BufWriter::new(out);
    bincode::serialize_into(&mut writer, &rendered, bincode::Infinite)
        .expect("failed to serialize tex results to bincode");

    // With new tests, create a new html
    svg::write(LAYOUT_HTML, &rendered);
}