use super::constants::MIN_CONNECTOR_OVERLAP;
use super::Glyph;
use super::glyph_metrics;

#[derive(Debug, Clone)]
pub struct GlyphVariants {
    replacements: Vec<ReplacementGlyph>,
    constructable: Option<ConstructableGlyph>,
}

// There are two types of variant glyphs:
//    Replacement glyphs and constructables that require multiple glpyhs.

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum VariantGlyph {
    Replacement   (Glyph),
    Constructable (Vec<GlyphInstruction>)
}

#[derive(Debug, Clone)]
struct ReplacementGlyph {
    unicode: u32,
    advance: u16,
}

#[derive(Debug, Clone)]
struct ConstructableGlyph {
    parts: Vec<GlyphPart>,
    italics_correction: i16,
}

#[derive(Debug, Copy, Clone)]
struct GlyphPart {
    unicode: u32,
    start_connector_length: u32,
    end_connector_length: u32,
    full_advance: u32,
    required: bool,
}

use std::collections::HashMap;

#[allow(dead_code)]
lazy_static! {
    #[allow(dead_code)]
    pub static ref VARIANTS: HashMap<u32, GlyphVariants> = {
        let mut m = HashMap::new();

        m.insert(0x28, GlyphVariants {
            constructable: Some(ConstructableGlyph {
                italics_correction: 0,
                parts: vec![
                    GlyphPart {
                        unicode: 0x239D,
                        start_connector_length: 0,
                        end_connector_length:   150,
                        full_advance:           1005,
                        required:               true,
                    },
                    GlyphPart {
                        unicode: 0x239C,
                        start_connector_length: 150,
                        end_connector_length:   150,
                        full_advance:           1010,
                        required:               false,
                    },
                    GlyphPart {
                        unicode: 0x239B,
                        start_connector_length: 150,
                        end_connector_length:   0,
                        full_advance:           1005,
                        required:               true,
                    },
                ]
            }),
            replacements: vec![
                ReplacementGlyph { unicode: 0x28,   advance: 854  }, // parenleft
                ReplacementGlyph { unicode: 0xE718, advance: 1231 }, // parenleft.size1
                ReplacementGlyph { unicode: 0xE719, advance: 1846 }, // parenleft.size2
                ReplacementGlyph { unicode: 0xE71A, advance: 2461 }, // parenleft.size3
                ReplacementGlyph { unicode: 0xE71B, advance: 3076 }, // parenleft.size4
            ],
        });

        m
    };
}


#[derive(Debug, Clone, Copy)]
struct GlyphInstruction {
    pub glyph:  Glyph,
    pub overlap: f64,
}

pub trait Variant {
    fn variant(&self, size: f64) -> VariantGlyph;
}

impl Variant for Glyph {
    fn variant(&self, size: f64) -> VariantGlyph {
        // The size variable describes the minimum advance requirement.  We will
        // take the glpyh with the minimum height that exceeds our requirment.

        let variants = match VARIANTS.get(&self.unicode) {
            None => return VariantGlyph::Replacement(*self),
            Some(g) => g,
        };

        // First check to see if any of the replacement glyphs meet the requirement.
        // It is assumed that the glyphs are in increasing advance.
        for glyph in &variants.replacements {
            if glyph.advance as f64 >= size {
                let replacement = glyph_metrics(glyph.unicode);
                return VariantGlyph::Replacement(replacement);
            }
        }

        // Next we check for a constructable glyph.
        // In the scenario that none of the replacement glyphs match the desired
        // advance, and there is no constructable glyph, we return the largest
        // replacement glyph.
        let construction = match *&variants.constructable {
            None => {
                let replacement = glyph_metrics(variants.replacements.last()
                    .expect("Unable to obtain last replacement glyph.  This shouldn't happen")
                    .unicode);
                return VariantGlyph::Replacement(replacement);
            }
            Some(ref c) => c,
        };

        // This function will measure the maximum size of a glyph construction
        // with a given number of repeatable glyphs.
        fn advance_with_glyphs(parts: &[GlyphPart], repeats: u8) -> f64 {
            use ::std::cmp::min;
            let mut advance = 0;
            let mut previous_connector = 0;
            for glyph in parts {
                // If this is an optional glyph, we repeat `repeats` times
                let count = if !glyph.required { repeats } else { 1 };
                for _ in 0..count {
                    let overlap =
                        min(previous_connector, glyph.start_connector_length);
                    advance += glyph.full_advance
                        - min(MIN_CONNECTOR_OVERLAP as u32, overlap);
                    previous_connector = glyph.end_connector_length;
                }
            }

            advance as f64
        }

        // We check for the smallest number of repeatable glyphs
        // that are required to meet our requirements.
        let mut count = 0;
        while advance_with_glyphs(&construction.parts, count) < size {
            count += 1;
            if count > 10 { panic!("Unable to construct large glyph."); }
        }

        // We now know how mean repeatable glyphs are required for our
        // construction, so we can create the glyph instructions.
        // We start with the smallest possible glyph.
        // While we are doing this, we will calculate the advance
        // of the entire glyph.
        let mut instructions: Vec<GlyphInstruction> = Vec::new();
        let mut previous_connector = 0;
        let mut glyph_advance: f64 = 0.0;
        for glyph in &construction.parts {
            use ::std::cmp::min;
            let repeat = if !glyph.required { count } else { 1 };
            let gly = glyph_metrics(glyph.unicode);
            for _ in 0..repeat {
                let overlap =
                    min(previous_connector, glyph.start_connector_length)
                    as f64;
                glyph_advance += glyph.full_advance as f64 - overlap;
                instructions.push(GlyphInstruction {
                    glyph:   gly,
                    overlap: overlap,
                });
            }
            previous_connector = glyph.end_connector_length;
        }

        // Now we will calculate how much we need to reduce our overlap
        // to construct a glyph of the desired size.
        let size_difference = size - glyph_advance;
        println!("size_difference: {:2}", size_difference);

        // Provided that our constructed glyph is _still_ too large,
        // return this, otherwise distribute the overlap equally
        // amonst each part.
        if size_difference < 0.0 {
            return VariantGlyph::Constructable(instructions)
        }

        let overlap = size_difference / (instructions.len() - 1) as f64;
        println!("Compensation Required: {:2}", overlap);
        for glyph in instructions.iter_mut().skip(1) {
            glyph.overlap -= overlap
        }

        VariantGlyph::Constructable(instructions)
    }
}

#[cfg(test)]
mod tests {
    use super::Variant;
    use font::glyph_metrics;

    #[test]
    fn can_extend_parenthesis() {
        let paren = glyph_metrics(0x28); // Left parantheses
        println!("800:  {:#?}", paren.variant(800 as f64));
        println!("1200: {:#?}", paren.variant(1200 as f64));
        println!("1800: {:#?}", paren.variant(1800 as f64));
        println!("2400: {:#?}", paren.variant(2400 as f64));
        println!("3000: {:#?}", paren.variant(3000 as f64));
        println!("3100: {:#?}", paren.variant(3100 as f64));
        println!("3600: {:#?}", paren.variant(3600 as f64));
        println!("3700: {:#?}", paren.variant(3700 as f64));
        println!("3800: {:#?}", paren.variant(3800 as f64));
        println!("3900: {:#?}", paren.variant(3900 as f64));
    }
}