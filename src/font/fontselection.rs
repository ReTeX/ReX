#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

mod offset {
    const LATIN_UPPER_IT = 
    const LATIN_UPPER_BB = 
    const LATIN_UPPER_BFIT = 
    const GREEK_UPPER_IT =


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

pub enum Weight {
    None,
    Bold,
    Italic,
    BoldItalic,
}

pub fn style_offset(unicode: u32) -> u32 {
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
                (_, Weight::None) => offset::LATIN_UPPER_NORMAL_NONE,
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
    }
}
