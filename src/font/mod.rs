mod glyphs;
mod constants;
mod symbols;
mod offsets;

pub use self::glyphs::GLYPHS;
pub use self::constants::CONSTANTS;
pub use self::symbols::SYMBOLS;
pub use self::offsets::IsAtom;

use parser::nodes::AtomType;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BBox(pub i16, pub i16, pub i16, pub i16);

#[derive(Debug, Clone)]
pub struct Glyph {
    pub unicode: u32,
    pub bbox: BBox,
    pub advance: u16,
    pub lsb: i16,
    pub italics: i16,    // design units
    pub attachment: i16, // design units
}

#[derive(Debug, Clone)]
pub struct Constants {
    pub accent_base_height: i16,
    pub axis_height: i16,
    pub delimited_sub_formula_min_height: u16,
    pub display_operator_min_height:  u16,
    pub flattened_accent_base_height: i16,
    pub fraction_denom_display_style_gap_min: i16,  
    pub fraction_denominator_display_style_shift_down: i16,  
    pub fraction_denominator_gap_min: i16,
    pub fraction_denominator_shift_down: i16,
    pub fraction_num_display_style_gap_min: i16,
    pub fraction_numerator_display_style_shift_up: i16,  
    pub fraction_numerator_gap_min: i16,
    pub fraction_numerator_shift_up: i16,
    pub fraction_rule_thickness: i16,
    pub lower_limit_baseline_drop_min: i16,  
    pub lower_limit_gap_min: i16,
    pub math_leading: i16,
    pub overbar_extra_ascender: i16, 
    pub overbar_rule_thickness: i16,
    pub overbar_vertical_gap: i16,
    pub radical_degree_bottom_raise_percent: i16, 
    pub radical_display_style_vertical_gap: i16,
    pub radical_extra_ascender: i16,
    pub radical_kern_after_degree: i16, 
    pub radical_kern_before_degree: i16,
    pub radical_rule_thickness: i16,
    pub radical_vertical_gap: i16,
    pub script_percent_scale_down: i16, 
    pub script_script_percent_scale_down: i16, 
    pub skewed_fraction_horizontal_gap: i16,
    pub skewed_fraction_vertical_gap: i16,
    pub space_after_script: i16,
    pub stack_bottom_display_style_shift_down: i16,  
    pub stack_bottom_shift_down: i16,
    pub stack_display_style_gap_min: i16, 
    pub stack_gap_min: i16,
    pub stack_top_display_style_shift_up: i16,  
    pub stack_top_shift_up: i16,
    pub stretch_stack_bottom_shift_down: i16,  
    pub stretch_stack_gap_above_min: i16,
    pub stretch_stack_gap_below_min: i16,
    pub stretch_stack_top_shift_up: i16,
    pub sub_superscript_gap_min: i16,
    pub subscript_baseline_drop_min: i16, 
    pub subscript_shift_down: i16,
    pub subscript_top_max: i16,
    pub superscript_baseline_drop_max: i16,  
    pub superscript_bottom_max_with_subscript: i16,  
    pub superscript_bottom_min: i16,
    pub superscript_shift_up: i16,
    pub superscript_shift_up_cramped: i16,  
    pub underbar_extra_descender: i16,
    pub underbar_rule_thickness: i16,
    pub underbar_vertical_gap: i16,
    pub upper_limit_baseline_rise_min: i16,  
    pub upper_limit_gap_min: i16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub id: u16,
    pub atom_type: AtomType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    Roman,
    Bold,
    Italic,
    BoldItalic,
    Caligraphic,  // Non-standard in UNICODE
                  // Many fonts treat Script <-> Caligraphic
    Script,
    ScriptBold,
    SansSerif,
    BoldSansSerif,
    ItalicSansSerif,
    BoldItalicSansSerif,
    DoubleStruck,       
    BoldDoubleStruck,       // Non-standard
    ItalicDoubleStruck,     // Non-standard
    BoldItalicDoubleStruck, // Non-standard
    Fraktur,
    BoldFraktur,
    Monospace,
}