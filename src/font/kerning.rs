use font::Glyph;
use dimensions::FontUnit;
use std::cmp::{max, min};
use super::kerning_table::KERNING_TABLE;

#[derive(Debug)]
enum Corner {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
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

pub fn superscript_kern(base: Glyph, script: Glyph, shift: FontUnit) -> FontUnit {
    let base_height = base.bbox.3;
    let script_depth = script.bbox.1 + shift;

    let value1 = kern_from(base, base_height, Corner::TopRight) +
                 kern_from(script, base_height, Corner::BottomLeft);

    let value2 = kern_from(base, script_depth, Corner::TopRight) +
                 kern_from(script, script_depth, Corner::BottomLeft);

    max(value1, value2)
}

pub fn subscript_kern(base: Glyph, script: Glyph, shift: FontUnit) -> FontUnit {
    let base_depth = base.bbox.1;
    let script_height = script.bbox.3 - shift;

    let value1 = kern_from(base, base_depth, Corner::BottomRight) +
                 kern_from(script, base_depth, Corner::TopLeft);

    let value2 = kern_from(base, script_height, Corner::BottomRight) +
                 kern_from(script, script_height, Corner::TopLeft);

    min(value1, value2)
}

macro_rules! try_or_nil {
    ($e:expr) => (
        match $e {
            Some(ref e) => e,
            None => return FontUnit::from(0),
        }
    )
}

fn kern_from(gly: Glyph, height: FontUnit, side: Corner) -> FontUnit {
    let record = match KERNING_TABLE.binary_search_by(|k| k.0.cmp(&gly.unicode)) {
        Ok(idx) => &KERNING_TABLE[idx].1,
        Err(_) => return 0.into(),
    };

    let table = match side {
        Corner::TopRight => try_or_nil!(record.top_right),
        Corner::TopLeft => try_or_nil!(record.top_left),
        Corner::BottomRight => try_or_nil!(record.bottom_right),
        Corner::BottomLeft => try_or_nil!(record.bottom_left),
    };

    // Assert: Correction heights are increasing.
    let mut idx: usize = 0;
    for &correction in table.correction_heights {
        if correction == FontUnit::from(0) || height <= correction {
            break;
        }
        idx += 1;
    }

    table.kern_values[idx]
}