use std::cmp::{min, max};

use super::constants::MIN_CONNECTOR_OVERLAP;
use super::Glyph;
use super::glyph_metrics;
use super::variant_tables::{VERT_VARIANTS, HORZ_VARIANTS};
use dimensions::FontUnit;


#[derive(Debug, Clone)]
pub struct GlyphVariants {
    pub replacements: Vec<ReplacementGlyph>,
    pub constructable: Option<ConstructableGlyph>,
}

// There are two types of variant glyphs:
//    Replacement glyphs and constructables that require multiple glpyhs.

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum VariantGlyph {
    Replacement(Glyph),
    Constructable(Direction, Vec<GlyphInstruction>),
}

#[derive(Debug, Clone)]
pub struct ReplacementGlyph {
    pub unicode: u32,
    pub advance: FontUnit,
}

#[derive(Debug, Clone)]
pub struct ConstructableGlyph {
    pub parts: Vec<GlyphPart>,
    pub italics_correction: FontUnit,
}

#[derive(Debug, Copy, Clone)]
pub struct GlyphPart {
    pub unicode: u32,
    pub start_connector_length: FontUnit,
    pub end_connector_length: FontUnit,
    pub full_advance: FontUnit,
    pub required: bool,
}

#[derive(Clone, Copy)]
pub struct GlyphInstruction {
    pub glyph: Glyph,
    pub overlap: FontUnit,
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Vertical,
    Horizontal,
}

pub trait Variant {
    fn variant(&self, FontUnit, Direction, bool) -> VariantGlyph;
    fn successor(&self) -> Glyph;

    fn vert_variant(&self, size: FontUnit) -> VariantGlyph {
        self.variant(size, Direction::Vertical, false)
    }

    fn horz_variant(&self, size: FontUnit) -> VariantGlyph {
        self.variant(size, Direction::Horizontal, true)
    }
}

impl Variant for Glyph {
    fn variant(&self, size: FontUnit, direction: Direction, find_min: bool) -> VariantGlyph {
        // The size variable describes the minimum advance requirement.  We will
        // take the glpyh with the minimum height that exceeds our requirment.

        let variants = if let Some(variant) = match direction {
               Direction::Vertical => VERT_VARIANTS.get(&self.unicode),
               Direction::Horizontal => HORZ_VARIANTS.get(&self.unicode),
           } {
            variant
        } else {
            return VariantGlyph::Replacement(*self);
        };

        // First check to see if any of the replacement glyphs meet the requirement.
        // It is assumed that the glyphs are in increasing advance.
        if find_min {
            for idx in 0..variants.replacements.len() {
                if variants.replacements[idx].advance >= size {
                    if idx == 0 {
                        return VariantGlyph::Replacement(*self);
                    } else {
                        return VariantGlyph::Replacement(glyph_metrics(variants.replacements[idx -
                                                                       1]
                                                                               .unicode));
                    }
                }
            }
        } else {
            for glyph in &variants.replacements {
                if glyph.advance >= size {
                    let replacement = glyph_metrics(glyph.unicode);
                    return VariantGlyph::Replacement(replacement);
                }
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

        // This function will measure the /maximum/ size of a glyph construction
        // with a given number of repeatable glyphs.
        fn max_size(parts: &[GlyphPart], repeats: u8) -> FontUnit {
            let mut advance = FontUnit::from(0);
            let overlap = MIN_CONNECTOR_OVERLAP;

            for glyph in parts {
                // If this is an optional glyph, we repeat `repeats` times
                let count = if !glyph.required { repeats } else { 1 } as u8;
                advance += count * (glyph.full_advance - overlap);
            }

            advance + overlap
        }

        // This function will measure the /smallest/ size of a glyph construction
        // with a given number of repeatable glyphs.
        fn min_size(parts: &[GlyphPart], repeats: u8) -> FontUnit {
            let mut advance = FontUnit::from(0);
            let mut prev_connector = FontUnit::from(0);

            for glyph in parts {
                let count = if !glyph.required { repeats } else { 1 };
                for _ in 0..count {
                    let overlap = min(prev_connector, glyph.start_connector_length);
                    advance += glyph.full_advance - max(overlap, MIN_CONNECTOR_OVERLAP);
                    prev_connector = glyph.end_connector_length;
                }
            }

            // We subtracted overlap on the first glyph when we shouldn't
            advance + MIN_CONNECTOR_OVERLAP
        }

        // We check for the smallest number of repeatable glyphs
        // that are required to meet our requirements.
        let mut count = 0;
        if find_min {
            while min_size(&construction.parts, count) < size {
                count += 1;
                if count > 100 {
                    panic!("Unable to construct large glyph! Max iteration hit.");
                }
            }
            // Current glyph is too large, go back one.
            count = ::std::cmp::max(0, count - 1);
        } else {
            while max_size(&construction.parts, count) < size {
                count += 1;
                if count > 100 {
                    panic!("Unable to construct large glyph.");
                }
            }
        }

        // We now know how many repeatable glyphs are required for our
        // construction, so we can create the glyph instructions.
        // We start with the smallest possible glyph.
        // While we are doing this, we will calculate the advance
        // of the entire glyph.
        let mut instructions: Vec<GlyphInstruction> = Vec::new();
        let mut prev_connector = FontUnit::from(0);
        let mut glyph_advance = FontUnit::from(0);
        let mut first = true;

        for glyph in &construction.parts {
            let repeat = if !glyph.required { count } else { 1 };
            let gly = glyph_metrics(glyph.unicode);
            for _ in 0..repeat {
                let overlap = max(min(prev_connector, glyph.start_connector_length),
                                  MIN_CONNECTOR_OVERLAP);

                if first {
                    glyph_advance += glyph.full_advance;
                    instructions.push(GlyphInstruction {
                                          glyph: gly,
                                          overlap: FontUnit::from(0),
                                      });
                    first = false;
                } else {
                    glyph_advance += glyph.full_advance - overlap;
                    instructions.push(GlyphInstruction {
                                          glyph: gly,
                                          overlap: overlap,
                                      });
                }

                prev_connector = glyph.end_connector_length;
            }
        }

        // Now we will calculate how much we need to reduce our overlap
        // to construct a glyph of the desired size.
        let size_difference = size - glyph_advance;

        // Provided that our constructed glyph is _still_ too large,
        // return this, otherwise distribute the overlap equally
        // amonst each part.
        if size_difference < FontUnit::from(0) {
            return VariantGlyph::Constructable(direction, instructions);
        }

        let overlap = size_difference / (instructions.len() as u16 - 1);
        for glyph in instructions.iter_mut().skip(1) {
            glyph.overlap -= overlap
        }

        VariantGlyph::Constructable(direction, instructions)
    }

    /// This method will look for a successor of a given glyph if there
    /// exits one.  This is how operators like `\int` and `\sum` become
    /// larger while in Display mode.

    fn successor(&self) -> Glyph {
        // If there are no variant glyphs, return itself.
        let variants = match VERT_VARIANTS.get(&self.unicode) {
            None => return *self,
            Some(g) => g,
        };

        // First check to see if any of the replacement glyphs meet the requirement.
        // It is assumed that the glyphs are in increasing advance.
        match variants.replacements.get(1) {
            Some(ref g) => glyph_metrics(g.unicode),
            None => *self,
        }
    }
}

use std::fmt;
impl fmt::Debug for GlyphInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "GlyphInst {{ glyph: 0x{:X}, overlap: {} }}",
               self.glyph.unicode,
               self.overlap)
    }
}

#[cfg(test)]
mod tests {
    use super::Variant;
    use font::glyph_metrics;

    #[test]
    fn can_find_successor() {
        let int = glyph_metrics(0x222B); // Integral
        println!("Int old: {:?}", int);
        println!("Int new: {:?}", int.successor());
    }
}