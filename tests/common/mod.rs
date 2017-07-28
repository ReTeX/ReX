#![allow(dead_code)]
pub mod debug_render;
pub mod svg_diff;
pub mod svg;

use std::path::Path;
use self::debug_render::Equation;
use bincode;

pub fn load_bincode<P: AsRef<Path>>(path: P) -> Vec<Equation> {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open(path.as_ref()).expect("failed to open test collection");
    let mut reader = BufReader::new(file);
    let tests: Vec<Equation> = bincode::deserialize_from(&mut reader, bincode::Infinite)
        .expect("failed to load historical test results");

    tests
}

pub fn equation_diffs(old: &[Equation], new: &[Equation]) -> Result<Vec<(Equation, Equation)>, ()> {
    if old.len() != new.len() {
        return Err(());
    }

    let mut diff: Vec<(Equation, Equation)> = Vec::new();
    for (left, right) in old.iter().zip(new.iter()) {
        if left != right {
            diff.push((left.clone(), right.clone()));
        }
    }

    Ok(diff)
}