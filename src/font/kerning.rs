#![allow(dead_code)]

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

        k
    };
}

// Vertical position of the script is calculated.  This depends on the following
// information: (see page 26 of https://www.tug.org/tugboat/tb30-1/tb94vieth.pdf)
//     - CONSTNATS:
//         - sub_superscript_gap_min: 264,
//         - subscript_baseline_drop_min: 50,
//         - subscript_shift_down: 250,
//         - subscript_top_max: 400,
//         - superscript_baseline_drop_max: 375,
//         - superscript_bottom_max_with_subscript: 400,
//         - superscript_bottom_min: 125,
//         - superscript_shift_up_cramped: 275,
//         - superscript_shift_up: 400,
//     - Base glyph height
//     - Base glyph depth
//     - For collisions
//         - subscript height
//         - subscript depth

Horizontal Position:
    - By default, set flat to base glyph
    - For superscript, add italics correction from base character
    - For suprscript:
        - Calculate bottom of script (after shiftup)
        - Calculate top of base.
    - For subscript:
        - Calculate top of script (after shift down)
        - Calculate bottom of base
    - For each script:
        - Find math kern value at this height for base.
          (TopRight for superscript, BotRight for subscript)
        - Find math kern value at this height for sciprt.
          (BotLeft for subscript, TopRight for superscript)
        - Add the values together together
    - Horintal kern is applied to smallest of two results
      from previous step.