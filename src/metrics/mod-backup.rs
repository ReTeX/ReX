// Font Metrics... What do we know?

// Information redefined on a per-glyph basis.
#![allow(unused_imports)]
use opentype;

use opentype::mathematics::Mathematics;
use opentype::layout::Coverage;
use truetype::Value;

use std::io::{Seek, SeekFrom};
use std::fs::File;

macro_rules! read_math_table {
    (XITS) => {
        {
            let mut file = File::open("XITS-Math.otf").expect("Unable to find XITS-math.otf");
            file.seek(SeekFrom::Start(496284)).expect("Unable to seek MathTable");
            Value::read(&mut file).expect("Failed to read MathTable")
        }
    };
}

// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
// enum ScriptType {
//     SuperScript,
//     SubScript,
// }

// Kerning Information

// LauTex: 
//   Check if super/sub and nucleus are simple character items.  use OpenType algorithm
//   for horizontal placement.
//
//   - Vertical positions of the script is calculated.
//   - Defeault horizontal position is next to base character.
//   - For superscripts, italic corrections of base characger is added.
//   - For a superscript, the two vertical values are calculated:
//      - Bottom of the script (after shift up)    --  Same as vertical position calculation?
//      - Top of the base.                       
//     For a subscript, the two values are the top of the (shifted script)
//   - For each of the two locations:
//     - Find the math kern value at this height for the /base/
//       Subscripts: Bottom right corner & Superscripts: Top right corner
//     - Find the math kern value at this heigh for the /script/
//       Subscripts: TopLeft corner, Superscript: BottomLeft corner
//     - Add the values together to get a preliminary result.
//   - The horizontal kern is applied to the /smallest/ of the two values
//     from the previous step.

//  Note: The math kern value at a specific height the the value that is
//        specified by the /next/ higher height and kern pair, or the highest
//        one in the character (if theres is no higher one), or 0 if none.

// Horizontal offset.
// Only used for simple symbols ???
// pub fn kern_scripts(base: u32, script: u32, script_type: ScriptType) 
//         -> u32 /* Scale points? */ {
//     let vert = vert_script(base, script, script_type);
//     let mut horiz = match script_type {
//         SuperScript => italic_correction(base),
//         SubScript => 0,
//     };

//     let base_kern = match script_type {
//         SuperScript => top_right(base, height(base)),
//         SubScript   => bot_right(base, vert + height(script)),
//     };

//     let script_kern = match script_type {
//         SuperScript => bot_left(script, horiz),
//         SubScript   => top_left(script, vert + height(script)),
//     };

//     ::std::cmp::min(base_kern, script_kern)
// }

// For every kerning table, same algorithm.
// pub fn top_right(base: u32, height: u32) -> u32 {
//     get_kern_value(metrics.kerning.get_top_right(base), height);
// }

// pub fn top_left(base: u32, height: u32) -> u32 {
//     get_kern_value(metrics.kerning.get_top_left(base), height);
// }

// pub fn bot_right(base: u32, height: u32) -> u32 {
//     get_kern_value(metrics.kerning.get_bot_right(base), height);
// }

// pub fn bot_left(base: u32, height: u32) -> u32 {
//     get_kern_value(metrics.kerning.get_bot_left(base), height);
// }

// fn get_kern_value(table: KernTable, height: u32) -> u32 {
//     if (table.heights.len() == 0) ||
//        (table.heights.len() != table.values.len() + 1) {
//         // For the later, this should probably warn
//         return 0;
//     }

//     let 
//     for idx in (0..table.heights.len()) {
//         if table.heights[idx] > height { break }
//     }
// }






// Tests from OpenType MathTable support.
#[test]
fn table() {
    let math: Mathematics = read_math_table!(XITS);

    assert_eq!(math.header.constants_offset, 10);
    assert_eq!(math.header.glyph_info_offset, 224);
    assert_eq!(math.header.variants_offset, 10152);
}

#[test]
fn constants() {
    let math: Mathematics = read_math_table!(XITS);
    let constants = math.constants;

    assert_eq!(constants.script_percent_scale_down, 75);
    assert_eq!(constants.delimited_sub_formula_min_height, 1500);
    assert_eq!(constants.math_leading.value, 150);
    assert_eq!(constants.radical_kern_after_degree.value, -555);
    assert_eq!(constants.radical_degree_bottom_raise_percent, 70);
}

#[test]
fn glyphs() {
    let math: Mathematics = read_math_table!(XITS);
    let glyphs = math.glyphs;

    match glyphs.corrections.coverage {
        Coverage::Format2(_) => (),
        _ => panic!("Parsed incorrect coverage table format for italics corrections."),
    }
    assert_eq!(glyphs.corrections.count, 643);
    assert_eq!(glyphs.corrections.values[0].value, 100);
    assert_eq!(glyphs.corrections.values[642].value, 80);

    match glyphs.attachments.coverage {
        Coverage::Format2(_) => (),
        _ => panic!("Parsed incorrect coverage table format for accent attachments."),
    }
    assert_eq!(glyphs.attachments.count, 1328);
    assert_eq!(glyphs.attachments.values[0].value, 361);
    assert_eq!(glyphs.attachments.values[1327].value, 201);

    if let Some(extended_shapes) = glyphs.extended_shape_coverage {
        match extended_shapes {
            Coverage::Format2(_) => (),
            _ => panic!("Parsed incorrect coverage table format for extended shapes."),
        }
    }
}