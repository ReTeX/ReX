// BMP unicode points of each symbol group.
const LATIN_UPPER_A: u32 = 0x41;
const LATIN_UPPER_Z: u32 = 0x5A;
const LATIN_LOWER_A: u32 = 0x61;
const LATIN_LOWER_Z: u32 = 0x7A;

const LATIN_LOWER_OFFSET: u32 = 0x61 - 0x41;

const GREEK_UPPER_ALPHA: u32 = 0x391;
const GREEK_UPPER_OMEGA: u32 = 0x3A9;
const GREEK_LOWER_ALPHA: u32 = 0x3B1;
const GREEK_LOWER_OMEGA: u32 = 0x3C9;

// TODO: Verify this -1
const GREEK_LOWER_OFFSET: u32 = 0x3B1 - 0x391 - 1;

const DIGIT_0: u32 = 0x30;
const DIGIT_9: u32 = 0x39;

// Exceptional glyphs that are not found in BMP
// and don't belong in the previous groups
// The handling of these values can be reference
//   unicode-math-usv.dtx in unicode-math
//   RenderMathMLToken.cpp in WebKit
const VAR_UPPER_THETA:      u32 = 0x3F4;
//const VAR_DIGAMMA:        u32 = 0x3DC;  ??
const VAR_EPSILON:          u32 = 0x3F5;
const VAR_THETA:            u32 = 0x3D1;
const VAR_KAPPA:            u32 = 0x3F0;
const VAR_PHI:              u32 = 0x3D5;
const VAR_RHO:              u32 = 0x3F1;
const VAR_PI:               u32 = 0x3D6;
//const DIGAMMA:            u32 = 0x3DD; ??
const PARTIAL_DIFFERENTIAL: u32 = 0x2202;
const NABLA:                u32 = 0x2207;

// Base-case offsets used for each group.
// Each family is laid out in continguous
// regions.  See the README to see this pattern.
// TODO: Add additional STIX support.  This will
//   require some extra logic somewhere.
const LATIN_BOLD_ROMAN: u32 = 0x1D400;
const GREEK_BOLD_ROMAN: u32 = 0x1D6A8;
const DIGIT_BOLD_ROMAN: u32 = 0x1D7CE;

const BOLD_VAR_UPPER_THETA:      u32 = 0x1D6B9;
const BOLD_VAR_EPSILON:          u32 = 0x1D6DC;
const BOLD_VAR_THETA:            u32 = 0x1D6DD;
const BOLD_VAR_KAPPA:            u32 = 0x1D6DE;
const BOLD_VAR_PHI:              u32 = 0x1D6DF;
const BOLD_VAR_RHO:              u32 = 0x1D6E0;
const BOLD_VAR_PI:               u32 = 0x1D6E1;
const BOLD_NABLA:                u32 = 0x1D6C1;
const BOLD_PARTIAL_DIFFERENTIAL: u32 = 0x1D6DB;

// style_symbol behavior:
// Commands will map to BMP unicode values.
// Characters in these BMP ranges will have their respective stlyes applied.

pub fn style_symbol(unicode: u32, style: Style) {
    match unicode {
        LATIN_UPPER_A...LATIN_UPPER_Z =>
            style_latin(unicode - LATIN_UPPER_A, style),

        LATIN_LOWER_A...LATIN_LOWER_Z =>
            style_latin(unicode - LATIN_LOWER_A + LATIN_LOWER_OFFSET, style),

        GREEK_UPPER_ALPHA...GREEK_UPPER_OMEGA =>
            style_greek(unicode - GREEK_UPPER_ALPHA, style),

        GREEK_LOWER_ALPHA...GREEK_LOWER_OMEGA =>
            style_greek(
                unicode - GREEK_LOWER_ALPHA + GREEK_LOWER_OFFSET,
                style),

        DIGIT_0...DIGIT_9 =>
            style_digit(unicode - DIGIT_0, style),

        _ => style_exceptions(unicode, style),
        }
    }
}

fn latin_offset(FAMILY, WEIGHT) -> u32 {
    // Determine the jump from the base case of BOLD
    let jump = match (FAMILY, WEIGHT) {
        (BOLD,       ROMAN) =>   0,
        (ITALIC,     ROMAN) =>   1,
        (BOLD_ITALC, ROMAN) =>   2,
        (NONE,       ROMAN) =>   1,
        (BOLD,       SCRIPT) =>  4,
        (_,          SCRIPT) =>  3,
        (BOLD,       FRAKTUR) => 7,
        (_,          FRAKTUR) => 5,
        (_,          BLACKBOARD) => 6,
        (NONE, SANS_SERIF) => 8,
        (BOLD, SANS_SERIF) => 9,
        (ITALIC, SANS_SERIF) => 10,
        (BOLD_ITALIC, SANS_SERIF) => 11
        (_, MONOSPACE) => 12,
    };
}

enum Family {
    BoldRoman,
    ItalicRoman,
    BoldItalicRoman,
    Script,
    BoldScript,
    Frakture
    Roman,
    Script,
    Fraktur,
    Blackboard,
    SansSerif,
    Monospace,
}