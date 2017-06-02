use super::Style;
use super::Family::*;
use super::Weight::*;

// BMP unicode points of each symbol group.
const UPPER_A: u32 = 0x41;
const UPPER_Z: u32 = 0x5A;
const LOWER_A: u32 = 0x61;
const LOWER_Z: u32 = 0x7A;

const UPPER_ALPHA: u32 = 0x391;
const UPPER_OMEGA: u32 = 0x3A9;
const LOWER_ALPHA: u32 = 0x3B1;
const LOWER_OMEGA: u32 = 0x3C9;

const DIGIT_0: u32 = 0x30;
const DIGIT_9: u32 = 0x39;

static LATIN_UPPER_LUT: [u32; 28] = [
//  None,    Italic,  Bold,    BoldItalic
    65,      0x1D434, 0x1D400, 0x1D468,   // Roman
    0x1D49C, 0x1D49C, 0x1D4D0, 0x1D4D0,   // Script
    0x1D504, 0x1D504, 0x1D56C, 0x1D56C,   // Fractur
    0x1D5A0, 0x1D608, 0x1D5D4, 0x1D63C,   // SansSerif
    0x1D538, 0x1D538, 0x1D538, 0x1D538,   // Blackboard
    0x1D670, 0x1D670, 0x1D670, 0x1D670,   // Monospace
    0x1D434, 0x1D434, 0x1D400, 0x1D468,   // Normal
];

static LATIN_LOWER_LUT: [u32; 28] = [
//  None,    Italic,  Bold,    BoldItalic
    97,      0x1D44E, 0x1D41A, 0x1D482,   // Roman
    0x1D4B6, 0x1D4B6, 0x1D4EA, 0x1D4EA,   // Script
    0x1D51E, 0x1D51E, 0x1D586, 0x1D586,   // Fracture
    0x1D5BA, 0x1D622, 0x1D5EE, 0x1D656,   // SansSerif
    0x1D552, 0x1D552, 0x1D552, 0x1D552,   // Blackboard
    0x1D68A, 0x1D68A, 0x1D68A, 0x1D68A,   // Monospace
    0x1D44E, 0x1D44E, 0x1D41A, 0x1D482,   // Normal
];

// Exceptional glyphs that are not found in BMP
// and don't belong in the previous groups
// The handling of these values can be reference
//   unicode-math-usv.dtx in unicode-math
//   RenderMathMLToken.cpp in WebKit
// const VAR_UPPER_THETA:      u32 = 0x3F4;
// //const VAR_DIGAMMA:        u32 = 0x3DC;  ??
// const VAR_EPSILON:          u32 = 0x3F5;
// const VAR_THETA:            u32 = 0x3D1;
// const VAR_KAPPA:            u32 = 0x3F0;
// const VAR_PHI:              u32 = 0x3D5;
// const VAR_RHO:              u32 = 0x3F1;
// const VAR_PI:               u32 = 0x3D6;
// //const DIGAMMA:            u32 = 0x3DD; ??
// const PARTIAL_DIFFERENTIAL: u32 = 0x2202;
// const NABLA:                u32 = 0x2207;

// const BOLD_VAR_UPPER_THETA:      u32 = 0x1D6B9;
// const BOLD_VAR_EPSILON:          u32 = 0x1D6DC;
// const BOLD_VAR_THETA:            u32 = 0x1D6DD;
// const BOLD_VAR_KAPPA:            u32 = 0x1D6DE;
// const BOLD_VAR_PHI:              u32 = 0x1D6DF;
// const BOLD_VAR_RHO:              u32 = 0x1D6E0;
// const BOLD_VAR_PI:               u32 = 0x1D6E1;
// const BOLD_NABLA:                u32 = 0x1D6C1;
// const BOLD_PARTIAL_DIFFERENTIAL: u32 = 0x1D6DB;

// style_symbol behavior:
// Commands will map to BMP unicode values.
// Characters in these BMP ranges will have their respective stlyes applied.

// TODO: Investigate how this relates to mapping standard symbols
//    to unicode + atomtype.  Construct a public API here.

impl Style {
    pub fn style_symbol(&self, unicode: u32) -> u32 {
        match unicode {
            LOWER_A...LOWER_Z => self.style_lower_latin(unicode - LOWER_A),

            UPPER_A...UPPER_Z => self.style_upper_latin(unicode - UPPER_A),

            UPPER_ALPHA...UPPER_OMEGA => self.style_upper_greek(unicode - UPPER_ALPHA),

            LOWER_ALPHA...LOWER_OMEGA => self.style_lower_greek(unicode - LOWER_ALPHA),

            DIGIT_0...DIGIT_9 => self.style_digit(unicode - DIGIT_0),

            _ => self.style_other(unicode),
        }
    }

    pub fn style_upper_latin(&self, offset: u32) -> u32 {
        let y = self.family as usize;
        let x = self.weight as usize;
        let base = LATIN_UPPER_LUT[4 * y + x];
        base + offset
    }

    pub fn style_lower_latin(&self, offset: u32) -> u32 {
        let y = self.family as usize;
        let x = self.weight as usize;
        let base = LATIN_LOWER_LUT[4 * y + x];
        base + offset
    }

    fn style_upper_greek(&self, offset: u32) -> u32 {
        let base = match self.family {
            Normal => {
                match self.weight {
                    None => 0x1D6E2,
                    Italic => 0x1D6E2,
                    Bold => 0x1D6A8,
                    BoldItalic => 0x1D71C,
                }
            }

            SansSerif => {
                match self.weight {
                    Italic => 0x1D6E2,
                    Bold => 0x1D756,
                    BoldItalic => 0x1D790,
                    None => 0x391,
                }
            }

            // fallback to Roman
            _ => {
                match self.weight {
                    Italic => 0x1D6E2,
                    Bold => 0x1D6A8,
                    BoldItalic => 0x1D71C,
                    None => 0x391,
                }
            }
        };

        base + offset
    }

    fn style_lower_greek(&self, offset: u32) -> u32 {
        let base = match self.family {
            Normal => {
                match self.weight {
                    None => 0x1D6FC,
                    Italic => 0x1D6FC,
                    Bold => 0x1D6C2,
                    BoldItalic => 0x1D736,
                }
            }

            SansSerif => {
                match self.weight {
                    Italic => 0x1D6FC,
                    Bold => 0x1D770,
                    BoldItalic => 0x1D7AA,
                    None => 0x3B1,
                }
            }

            // fallback to Roman
            _ => {
                match self.weight {
                    Italic => 0x1D6FC,
                    Bold => 0x1D6C2,
                    BoldItalic => 0x1D736,
                    None => 0x3B1,
                }
            }
        };

        base + offset
    }

    fn style_digit(&self, offset: u32) -> u32 {
        let base = match self.family {
            SansSerif => {
                match self.weight {
                    BoldItalic | Bold => 0x1D7EC,
                    _ => 0x1D7E2,
                }
            }

            Blackboard => 0x1D7D8,
            Monospace => 0x1D7F6,

            // fallback to Roman
            _ => {
                match self.weight {
                    BoldItalic | Bold => 0x1D7CE,
                    _ => 48,
                }
            }
        };

        base + offset
    }

    fn style_other(&self, sym: u32) -> u32 {
        // TODO add symbols that require styling
        // like varepsilon, etc...
        match sym {
            // Minus Sign
            0x2D => 0x2212,
            _ => sym,
        }
    }
}