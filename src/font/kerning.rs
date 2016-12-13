#![allow(dead_code)]
use font::Glyph;

#[derive(Debug, Clone)]
pub struct KernRecord {
    top_right:     Option<KernTable>,
    top_left:      Option<KernTable>,
    bottom_right:  Option<KernTable>,
    bottom_left:   Option<KernTable>,
}

#[derive(Debug, Clone)]
struct KernTable {
    correction_heights: Vec<i16>,   // unit::Font::<i16>()
    kern_values:        Vec<i16>,   // unit::Font::<i16>()
}

use std::collections::HashMap;

lazy_static! {
    pub static ref KERNINGS: HashMap<u32, KernRecord> = {
        let mut k = HashMap::new();

        k.insert(0x41, KernRecord { // A
            top_right: Some(KernTable {
                correction_heights: vec![ 275 ],
                kern_values: vec![ -100, -150 ],
            }),
            top_left: None,
            bottom_right: Some(KernTable {
                correction_heights: vec![],
                kern_values: vec![ 50 ],
            }),
            bottom_left: None,
        });

        k.insert(0x54, KernRecord { // T
            top_right: Some(KernTable {
                correction_heights: vec![],
                kern_values: vec![ 50 ],
            }),
            top_left: None,
            bottom_right: Some(KernTable {
                correction_heights: vec![],
                kern_values: vec![ -100 ],
            }),
            bottom_left: None,
        });

        k.insert(0x1D447, KernRecord { // T
            top_right: Some(KernTable {
                correction_heights: vec![],
                kern_values: vec![ 50 ],
            }),
            top_left: None,
            bottom_right: Some(KernTable {
                correction_heights: vec![],
                kern_values: vec![ -100 ],
            }),
            bottom_left: None,
        });

        k
    };
}

// Horizontal Position:
//     - By default, set flat to base glyph
//     - For superscript, add italics correction from base character
//     - For suprscript:
//         - Calculate bottom of script (after shiftup)
//         - Calculate top of base.
//     - For subscript:
//         - Calculate top of script (after shift down)
//         - Calculate bottom of base
//     - For each script:
//         - Find math kern value at this height for base.
//           (TopRight for superscript, BotRight for subscript)
//         - Find math kern value at this height for sciprt.
//           (BotLeft for subscript, TopRight for superscript)
//         - Add the values together together
//     - Horintal kern is applied to smallest of two results
//       from previous step.

// I question the accuracy of this algorithm.  But it's not yet clear to me what
// the relavent values should represent with respect to the "cut-ins" for the kerning.
// for now, I'm just going to port the algorithm I found in LuaTeX and XeTeX.
// If nothing else, it will at least be consistent.

#[derive(Debug)]
enum Corner {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

pub fn superscript_kern(base: Glyph, script: Glyph, shift: f64) -> f64 {
    let base_height  = base.bbox.3 as f64;
    let script_depth = script.bbox.1 as f64 + shift;

    let value1 = kern_from(base, base_height, Corner::TopRight)
        + kern_from(script, base_height, Corner::BottomLeft);

    let value2 = kern_from(base, script_depth, Corner::TopRight)
        + kern_from(script, script_depth, Corner::BottomLeft);

    value1.max(value2)
}

pub fn subscript_kern(base: Glyph, script: Glyph, shift: f64) -> f64 {
    let base_depth = base.bbox.1 as f64;
    let script_height = script.bbox.3 as f64 - shift;

    println!("{}, {}", base_depth, script_height);

    let value1 = kern_from(base, base_depth, Corner::BottomRight)
        + kern_from(script, base_depth, Corner::TopLeft);

    let value2 = kern_from(base, script_height, Corner::BottomRight)
        + kern_from(script, script_height, Corner::TopLeft);

    println!("{}, {}", value1, value2);
    value1.min(value2)
}

macro_rules! otry {
    ($e:expr) => (
        match $e {
            Some(ref e) => e,
            None => return 0.0,
        }
    )
}

fn kern_from(gly: Glyph, height: f64, side: Corner) -> f64 {
    let record = match KERNINGS.get(&gly.unicode) {
        Some(rec) => rec,
        None => return 0.0,
    };

    println!("Got kerning: {:?}", record);
    println!("Corner: {:?}", side);
    let table = match side {
        Corner::TopRight    => otry!(record.top_right),
        Corner::TopLeft     => otry!(record.top_left),
        Corner::BottomRight => otry!(record.bottom_right),
        Corner::BottomLeft  => otry!(record.bottom_left),
    };

    println!("Getting val");
    if table.correction_heights.is_empty() || height < (table.correction_heights[0] as f64) {
        let v = table.kern_values[0] as f64;
        println!("{}", v);
        v
    } else {
        let mut value = **otry!(table.kern_values.last()) as f64;

        for (idx, &h) in table.correction_heights.iter().enumerate() {
            if height <= (h as f64) {
                value = **otry!(table.kern_values.get(idx)) as f64;
                break;
            }
        }

        value
    }
}