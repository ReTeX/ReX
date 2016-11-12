#[allow(dead_code)]

mod offset {
   pub const LATIN_UPPER_BLACKBOARD_NONE: u32 = 120055;
   pub const LATIN_UPPER_CALLIGRAPHIC_NONE: u32 = 119899;
   pub const LATIN_UPPER_CALLIGRAPHIC_BOLD: u32 = 119951;
   pub const LATIN_UPPER_FRAKTUR_NONE: u32 = 120003;
   pub const LATIN_UPPER_FRAKTUR_BOLD: u32 = 120107;
   pub const LATIN_UPPER_NORMAL_NONE: u32 = 119795;
   pub const LATIN_UPPER_NORMAL_BOLD: u32 = 119743;
   pub const LATIN_UPPER_NORMAL_BOLDITALIC: u32 = 119847;
   pub const LATIN_UPPER_NORMAL_ITALIC: u32 = 119795;
   pub const LATIN_UPPER_ROMAN_NONE: u32 = 0;
   pub const LATIN_UPPER_ROMAN_BOLD: u32 = 119743;
   pub const LATIN_UPPER_ROMAN_BOLDITALIC: u32 = 119847;
   pub const LATIN_UPPER_ROMAN_ITALIC: u32 = 119795;
   pub const LATIN_UPPER_SCRIPT_NONE: u32 = 119899;
   pub const LATIN_UPPER_SCRIPT_BOLD: u32 = 119951;
   pub const LATIN_UPPER_SANSSERIF_NONE: u32 = 120159;
   pub const LATIN_UPPER_SANSSERIF_BOLD: u32 = 120211;
   pub const LATIN_UPPER_SANSSERIF_BOLDITALIC: u32 = 120315;
   pub const LATIN_UPPER_SANSSERIF_ITALIC: u32 = 120263;
   pub const LATIN_UPPER_TELETYPE_NONE: u32 = 120367;
   pub const GREEK_UPPER_NORMAL_NONE: u32 = 119633;
   pub const GREEK_UPPER_NORMAL_BOLD: u32 = 119575;
   pub const GREEK_UPPER_NORMAL_BOLDITALIC: u32 = 119691;
   pub const GREEK_UPPER_NORMAL_ITALIC: u32 = 119633;
   pub const GREEK_UPPER_ROMAN_NONE: u32 = 0;
   pub const GREEK_UPPER_ROMAN_BOLD: u32 = 119575;
   pub const GREEK_UPPER_ROMAN_BOLDITALIC: u32 = 119691;
   pub const GREEK_UPPER_ROMAN_ITALIC: u32 = 119633;
   pub const GREEK_UPPER_SANSSERIF_BOLD: u32 = 119749;
   pub const GREEK_UPPER_SANSSERIF_ITALIC: u32 = 119807;
   pub const GREEK_LOWER_NORMAL_NONE: u32 = 119627;
   pub const GREEK_LOWER_NORMAL_BOLD: u32 = 119569;
   pub const GREEK_LOWER_NORMAL_BOLDITALIC: u32 = 119685;
   pub const GREEK_LOWER_NORMAL_ITALIC: u32 = 119627;
   pub const GREEK_LOWER_ROMAN_NONE: u32 = 0;
   pub const GREEK_LOWER_ROMAN_BOLD: u32 = 119569;
   pub const GREEK_LOWER_ROMAN_BOLDITALIC: u32 = 119685;
   pub const GREEK_LOWER_ROMAN_ITALIC: u32 = 119627;
   pub const GREEK_LOWER_SANSSERIF_BOLD: u32 = 119743;
   pub const GREEK_LOWER_SANSSERIF_BOLDITALIC: u32 = 119801;
   pub const DIGIT_BLACKBOARD_NONE: u32 = 120744;
   pub const DIGIT_NORMAL_NONE: u32 = 0;
   pub const DIGIT_NORMAL_BOLD: u32 = 120734;
   pub const DIGIT_NORMAL_BOLDITALIC: u32 = 120734;
   pub const DIGIT_NORMAL_ITALIC: u32 = 120754;
   pub const DIGIT_ROMAN_NONE: u32 = 0;
   pub const DIGIT_ROMAN_BOLD: u32 = 120734;
   pub const DIGIT_ROMAN_BOLDITALIC: u32 = 120734;
   pub const DIGIT_SANSSERIF_NONE: u32 = 120754;
   pub const DIGIT_SANSSERIF_BOLD: u32 = 120734;
   pub const DIGIT_SANSSERIF_BOLDITALIC: u32 = 120734;
   pub const DIGIT_SANSSERIF_ITALIC: u32 = 120754;
   pub const DIGIT_TELETYPE_NONE: u32 = 120774;
   pub const LATIN_LOWER_BLACKBOARD_NONE: u32 = 120049;
   pub const LATIN_LOWER_FRAKTUR_NONE: u32 = 119997;
   pub const LATIN_LOWER_FRAKTUR_BOLD: u32 = 120101;
   pub const LATIN_LOWER_NORMAL_NONE: u32 = 119789;
   pub const LATIN_LOWER_NORMAL_BOLD: u32 = 119737;
   pub const LATIN_LOWER_NORMAL_BOLDITALIC: u32 = 119841;
   pub const LATIN_LOWER_NORMAL_ITALIC: u32 = 119789;
   pub const LATIN_LOWER_ROMAN_NONE: u32 = 0;
   pub const LATIN_LOWER_ROMAN_BOLD: u32 = 119737;
   pub const LATIN_LOWER_ROMAN_BOLDITALIC: u32 = 119841;
   pub const LATIN_LOWER_ROMAN_ITALIC: u32 = 119789;
   pub const LATIN_LOWER_SCRIPT_NONE: u32 = 119893;
   pub const LATIN_LOWER_SCRIPT_BOLD: u32 = 119945;
   pub const LATIN_LOWER_SANSSERIF_NONE: u32 = 120153;
   pub const LATIN_LOWER_SANSSERIF_BOLD: u32 = 120205;
   pub const LATIN_LOWER_SANSSERIF_BOLDITALIC: u32 = 120309;
   pub const LATIN_LOWER_SANSSERIF_ITALIC: u32 = 120257;
   pub const LATIN_LOWER_TELETYPE_NONE: u32 = 120361;
}

const DIGIT_START: u32 = 48;
const DIGIT_END: u32 = 48 + 10;
const LATIN_LOWER_START: u32 = 97;
const LATIN_LOWER_END: u32 = 97 + 26;
const LATIN_UPPER_START: u32 = 65;
const LATIN_UPPER_END: u32 = 65 + 26;
const GREEK_LOWER_START: u32 = 0x3B1;
const GREEK_LOWER_END: u32 = 0x3B1 + 25;
const GREEK_UPPER_START: u32 = 0x391;
const GREEK_UPPER_END: u32 = 0x391 + 24;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Family {
    Normal,
    Roman,
    Script,
    Calligraphic,
    SansSerif,
    Fraktur,
    Blackboard,
    Teletype,
}

use ::std::convert::TryFrom;
impl<'a> TryFrom<&'a str> for Family {
    type Err = ();
    fn try_from(s: &str) -> Result<Family, Self::Err> {
        match s {
            "mathbb"     => Ok(Family::Blackboard),
            "mathrm"     => Ok(Family::Roman),
            "mathcal"    => Ok(Family::Calligraphic),
            "mathfrak"   => Ok(Family::Fraktur),
            "mathnormal" => Ok(Family::Normal),
            "mathsf"     => Ok(Family::SansSerif),
            "mathscr"    => Ok(Family::Script),
            "mathtt"     => Ok(Family::Teletype),
            _ => Err(())
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Weight {
    None,
    Bold,
    Italic,
    BoldItalic,
}

impl<'a> TryFrom<&'a str> for Weight {
    type Err = ();
    fn try_from(s: &str) -> Result<Weight, Self::Err> {
        match s {
            "bf" | "mathbf" => Ok(Weight::Bold),
            "it" | "mathit" => Ok(Weight::Italic),
            _ => Err(())
        }
    }
}

pub fn style_offset(unicode: u32, family: Family, weight: Weight) -> u32 {
    match unicode {
        DIGIT_START...DIGIT_END => {
            match (family, weight) {
                (Family::Blackboard, Weight::None) => offset::DIGIT_BLACKBOARD_NONE,
                (Family::Normal, Weight::None) => offset::DIGIT_NORMAL_NONE,
                (Family::Normal, Weight::Bold) => offset::DIGIT_NORMAL_BOLD,
                (Family::Normal, Weight::BoldItalic) => offset::DIGIT_NORMAL_BOLDITALIC,
                (Family::Normal, Weight::Italic) => offset::DIGIT_NORMAL_ITALIC,
                (Family::Roman, Weight::None) => offset::DIGIT_ROMAN_NONE,
                (Family::Roman, Weight::Bold) => offset::DIGIT_ROMAN_BOLD,
                (Family::Roman, Weight::BoldItalic) => offset::DIGIT_ROMAN_BOLDITALIC,
                (Family::SansSerif, Weight::None) => offset::DIGIT_SANSSERIF_NONE,
                (Family::SansSerif, Weight::Bold) => offset::DIGIT_SANSSERIF_BOLD,
                (Family::SansSerif, Weight::BoldItalic) => offset::DIGIT_SANSSERIF_BOLDITALIC,
                (Family::SansSerif, Weight::Italic) => offset::DIGIT_SANSSERIF_ITALIC,
                (Family::Teletype, Weight::None) => offset::DIGIT_TELETYPE_NONE,
                (_, Weight::None) => offset::DIGIT_NORMAL_NONE,
                (_, Weight::Bold) => offset::DIGIT_NORMAL_BOLD,
                (_, Weight::Italic) => offset::DIGIT_NORMAL_ITALIC,
                (_, Weight::BoldItalic) => offset::DIGIT_NORMAL_BOLDITALIC,
            }
        },
        LATIN_LOWER_START...LATIN_LOWER_END => {
            match (family, weight) {
                (Family::Blackboard, Weight::None) => offset::LATIN_LOWER_BLACKBOARD_NONE,
                (Family::Fraktur, Weight::None) => offset::LATIN_LOWER_FRAKTUR_NONE,
                (Family::Fraktur, Weight::Bold) => offset::LATIN_LOWER_FRAKTUR_BOLD,
                (Family::Normal, Weight::None) => offset::LATIN_LOWER_NORMAL_NONE,
                (Family::Normal, Weight::Bold) => offset::LATIN_LOWER_NORMAL_BOLD,
                (Family::Normal, Weight::BoldItalic) => offset::LATIN_LOWER_NORMAL_BOLDITALIC,
                (Family::Normal, Weight::Italic) => offset::LATIN_LOWER_NORMAL_ITALIC,
                (Family::Roman, Weight::None) => offset::LATIN_LOWER_ROMAN_NONE,
                (Family::Roman, Weight::Bold) => offset::LATIN_LOWER_ROMAN_BOLD,
                (Family::Roman, Weight::BoldItalic) => offset::LATIN_LOWER_ROMAN_BOLDITALIC,
                (Family::Roman, Weight::Italic) => offset::LATIN_LOWER_ROMAN_ITALIC,
                (Family::Script, Weight::None) => offset::LATIN_LOWER_SCRIPT_NONE,
                (Family::Script, Weight::Bold) => offset::LATIN_LOWER_SCRIPT_BOLD,
                (Family::SansSerif, Weight::None) => offset::LATIN_LOWER_SANSSERIF_NONE,
                (Family::SansSerif, Weight::Bold) => offset::LATIN_LOWER_SANSSERIF_BOLD,
                (Family::SansSerif, Weight::BoldItalic) => offset::LATIN_LOWER_SANSSERIF_BOLDITALIC,
                (Family::SansSerif, Weight::Italic) => offset::LATIN_LOWER_SANSSERIF_ITALIC,
                (Family::Teletype, Weight::None) => offset::LATIN_LOWER_TELETYPE_NONE,
                (_, Weight::None) => offset::LATIN_LOWER_NORMAL_NONE,
                (_, Weight::Bold) => offset::LATIN_LOWER_NORMAL_BOLD,
                (_, Weight::Italic) => offset::LATIN_LOWER_NORMAL_ITALIC,
                (_, Weight::BoldItalic) => offset::LATIN_LOWER_NORMAL_BOLDITALIC,
            }
        },
        GREEK_UPPER_START...GREEK_UPPER_END => {
            match (family, weight) {
                (Family::Normal, Weight::None) => offset::GREEK_UPPER_NORMAL_NONE,
                (Family::Normal, Weight::Bold) => offset::GREEK_UPPER_NORMAL_BOLD,
                (Family::Normal, Weight::BoldItalic) => offset::GREEK_UPPER_NORMAL_BOLDITALIC,
                (Family::Normal, Weight::Italic) => offset::GREEK_UPPER_NORMAL_ITALIC,
                (Family::Roman, Weight::None) => offset::GREEK_UPPER_ROMAN_NONE,
                (Family::Roman, Weight::Bold) => offset::GREEK_UPPER_ROMAN_BOLD,
                (Family::Roman, Weight::BoldItalic) => offset::GREEK_UPPER_ROMAN_BOLDITALIC,
                (Family::Roman, Weight::Italic) => offset::GREEK_UPPER_ROMAN_ITALIC,
                (Family::SansSerif, Weight::Bold) => offset::GREEK_UPPER_SANSSERIF_BOLD,
                (Family::SansSerif, Weight::Italic) => offset::GREEK_UPPER_SANSSERIF_ITALIC,
                (_, Weight::None) => offset::GREEK_UPPER_NORMAL_NONE,
                (_, Weight::Bold) => offset::GREEK_UPPER_NORMAL_BOLD,
                (_, Weight::Italic) => offset::GREEK_UPPER_NORMAL_ITALIC,
                (_, Weight::BoldItalic) => offset::GREEK_UPPER_NORMAL_BOLDITALIC,
            }
        },
        LATIN_UPPER_START...LATIN_UPPER_END => {
            match (family, weight) {
                (Family::Blackboard, Weight::None) => offset::LATIN_UPPER_BLACKBOARD_NONE,
                (Family::Calligraphic, Weight::None) => offset::LATIN_UPPER_CALLIGRAPHIC_NONE,
                (Family::Calligraphic, Weight::Bold) => offset::LATIN_UPPER_CALLIGRAPHIC_BOLD,
                (Family::Fraktur, Weight::None) => offset::LATIN_UPPER_FRAKTUR_NONE,
                (Family::Fraktur, Weight::Bold) => offset::LATIN_UPPER_FRAKTUR_BOLD,
                (Family::Normal, Weight::None) => offset::LATIN_UPPER_NORMAL_NONE,
                (Family::Normal, Weight::Bold) => offset::LATIN_UPPER_NORMAL_BOLD,
                (Family::Normal, Weight::BoldItalic) => offset::LATIN_UPPER_NORMAL_BOLDITALIC,
                (Family::Normal, Weight::Italic) => offset::LATIN_UPPER_NORMAL_ITALIC,
                (Family::Roman, Weight::None) => offset::LATIN_UPPER_ROMAN_NONE,
                (Family::Roman, Weight::Bold) => offset::LATIN_UPPER_ROMAN_BOLD,
                (Family::Roman, Weight::BoldItalic) => offset::LATIN_UPPER_ROMAN_BOLDITALIC,
                (Family::Roman, Weight::Italic) => offset::LATIN_UPPER_ROMAN_ITALIC,
                (Family::Script, Weight::None) => offset::LATIN_UPPER_SCRIPT_NONE,
                (Family::Script, Weight::Bold) => offset::LATIN_UPPER_SCRIPT_BOLD,
                (Family::SansSerif, Weight::None) => offset::LATIN_UPPER_SANSSERIF_NONE,
                (Family::SansSerif, Weight::Bold) => offset::LATIN_UPPER_SANSSERIF_BOLD,
                (Family::SansSerif, Weight::BoldItalic) => offset::LATIN_UPPER_SANSSERIF_BOLDITALIC,
                (Family::SansSerif, Weight::Italic) => offset::LATIN_UPPER_SANSSERIF_ITALIC,
                (Family::Teletype, Weight::None) => offset::LATIN_UPPER_TELETYPE_NONE,
                //(_, Weight::None) => offset::LATIN_UPPER_NORMAL_NONE,
                (_, Weight::Bold) => offset::LATIN_UPPER_NORMAL_BOLD,
                (_, Weight::Italic) => offset::LATIN_UPPER_NORMAL_ITALIC,
                (_, Weight::BoldItalic) => offset::LATIN_UPPER_NORMAL_BOLDITALIC,
            }
        },
        GREEK_LOWER_START...GREEK_LOWER_END => {
            match (family, weight) {
                (Family::Normal, Weight::None) => offset::GREEK_LOWER_NORMAL_NONE,
                (Family::Normal, Weight::Bold) => offset::GREEK_LOWER_NORMAL_BOLD,
                (Family::Normal, Weight::BoldItalic) => offset::GREEK_LOWER_NORMAL_BOLDITALIC,
                (Family::Normal, Weight::Italic) => offset::GREEK_LOWER_NORMAL_ITALIC,
                (Family::Roman, Weight::None) => offset::GREEK_LOWER_ROMAN_NONE,
                (Family::Roman, Weight::Bold) => offset::GREEK_LOWER_ROMAN_BOLD,
                (Family::Roman, Weight::BoldItalic) => offset::GREEK_LOWER_ROMAN_BOLDITALIC,
                (Family::Roman, Weight::Italic) => offset::GREEK_LOWER_ROMAN_ITALIC,
                (Family::SansSerif, Weight::Bold) => offset::GREEK_LOWER_SANSSERIF_BOLD,
                (Family::SansSerif, Weight::BoldItalic) => offset::GREEK_LOWER_SANSSERIF_BOLDITALIC,
                (_, Weight::None) => offset::GREEK_LOWER_NORMAL_NONE,
                (_, Weight::Bold) => offset::GREEK_LOWER_NORMAL_BOLD,
                (_, Weight::Italic) => offset::GREEK_LOWER_NORMAL_ITALIC,
                (_, Weight::BoldItalic) => offset::GREEK_LOWER_NORMAL_BOLDITALIC,
            }
        },
        _ => 0,
    }
}
