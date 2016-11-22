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
enum VariantGlyph {
    Replacement   (Glyph),
    Constructable (GlyphConstruction)
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

// The connectors define the regions for which you are allowed to overlap glyphs.  In essence, they
// define the "max-overlap" for each given side.
//
// The advance of the entire constructed glyph can be calculated by looking at the full_advance of the 
// _last_ glyph, plus the offset of the last glyph.

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

// This provides instruction on construction the variant glyphs using the layout system.
#[derive(Debug, Clone)]
struct GlyphConstruction(Vec<GlyphInstruction>);

#[derive(Debug, Clone, Copy)]
struct GlyphInstruction {
    glyph:  Glyph,
    offset: f64,
}

trait Variant {
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
        let construction = match *&variants.constructable {
            None => {
                // In the scenario that none of the replacement glyphs match the desired
                // advance, and there is no constructable glyph, we return the largest
                // replacement glyph.
                let replacement = glyph_metrics(variants.replacements.last()
                    .expect("Unable to obtain last replacement glyph.  This shouldn't happen")
                    .unicode);
                return VariantGlyph::Replacement(replacement);
            }
            Some(ref c) => c,
        };


        // We start with the smallest collection of glyph instructions.
        let mut parts: Vec<GlyphPart> = Vec::new();
        for glyph in &construction.parts {
            if glyph.required { parts.push(*glyph) }
        }

        // Next, we check if this construction meets are requirements.
        // If not, we add an optional glyph, and repeat as necessary.
        loop {
            // Check if our construction meets are requirements
            let mut advance = 0;
            let mut previous_connector = 0;
            for glyph in parts {
                advance += std::cmp::min(previous_connector, glyph.start_connector_length);
                previous_connector = glyph.end_connector_length;  
            }

            
        }

        unimplemented!()
    }
}