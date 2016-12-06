use std::cmp::{ min, max };

use super::constants::MIN_CONNECTOR_OVERLAP;
use super::Glyph;
use super::glyph_metrics;
use super::variant_tables::{ VERT_VARIANTS, HORZ_VARIANTS };


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
    Replacement   (Glyph),
    Constructable (Direction, Vec<GlyphInstruction>)
}

#[derive(Debug, Clone)]
pub struct ReplacementGlyph {
    pub unicode: u32,
    pub advance: u16,
}

#[derive(Debug, Clone)]
pub struct ConstructableGlyph {
    pub parts: Vec<GlyphPart>,
    pub italics_correction: i16,
}

#[derive(Debug, Copy, Clone)]
pub struct GlyphPart {
    pub unicode: u32,
    pub start_connector_length: u32,
    pub end_connector_length: u32,
    pub full_advance: u32,
    pub required: bool,
}

#[derive(Clone, Copy)]
pub struct GlyphInstruction {
    pub glyph:  Glyph,
    pub overlap: f64,
}

pub trait Variant {
    fn variant(&self, f64, Direction, bool) -> VariantGlyph;
    fn successor(&self) -> Glyph;

    fn vert_variant(&self, size: f64) -> VariantGlyph {
        self.variant(size, Direction::Vertical, false)
    }

    fn horz_variant(&self, size: f64) -> VariantGlyph {
        self.variant(size, Direction::Horizontal, true)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Vertical,
    Horizontal,
}

impl Variant for Glyph {
    fn variant(&self, size: f64, direction: Direction, find_min: bool) -> VariantGlyph {
        // The size variable describes the minimum advance requirement.  We will
        // take the glpyh with the minimum height that exceeds our requirment.

        println!("PreVariants!");
        let variants = if let Some(variant) = match direction {
                Direction::Vertical   => VERT_VARIANTS.get(&self.unicode),
                Direction::Horizontal => HORZ_VARIANTS.get(&self.unicode), }
            {
                variant
            } else {
                return VariantGlyph::Replacement(*self)
            };

        println!("Variants!");
        // First check to see if any of the replacement glyphs meet the requirement.
        // It is assumed that the glyphs are in increasing advance.
        if find_min {
            for idx in 0..variants.replacements.len() {
                if variants.replacements[idx].advance as f64 >= size {
                    let replacement =
                        glyph_metrics(variants.replacements[max(0, idx - 1)].unicode);
                    return VariantGlyph::Replacement(replacement);

                }
            }
        } else {
            for glyph in &variants.replacements {
                if glyph.advance as f64 >= size {
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
        fn max_size(parts: &[GlyphPart], repeats: u8) -> f64 {
            let mut advance = 0;
            let overlap = *MIN_CONNECTOR_OVERLAP;

            for glyph in parts {
                // If this is an optional glyph, we repeat `repeats` times
                let count = if !glyph.required { repeats } else { 1 } as u32;
                advance += count*(glyph.full_advance - overlap);
            }

            advance as f64 + overlap as f64
        }

        // This function will measure the /smallest/ size of a glyph construction
        // with a given number of repeatable glyphs.
        fn min_size(parts: &[GlyphPart], repeats: u8) -> f64 {
            let mut advance = 0;
            let mut prev_connector = 0;

            for glyph in parts {
                let count = if !glyph.required { repeats } else { 1 };
                for _ in 0..count {
                    let overlap =
                        min(prev_connector, glyph.start_connector_length);
                    advance += glyph.full_advance
                        - max(overlap, *MIN_CONNECTOR_OVERLAP);
                    prev_connector = glyph.end_connector_length;
                }
            }

            // We subtracted overlap on the first glyph when we shouldn't
            advance as f64 + *MIN_CONNECTOR_OVERLAP as f64
        }

        // We check for the smallest number of repeatable glyphs
        // that are required to meet our requirements.
        let mut count = 0;
        if find_min {
            while min_size(&construction.parts, count) < size {
                count +=1 ;
                if count > 20 { panic!("Unable to construct large glyph! Max iteration hit."); }
            }
            // Current glyph is too large, go back one.
            count = ::std::cmp::max(0, count - 1);
        } else {
            while max_size(&construction.parts, count) < size {
                count += 1;
                if count > 20 { panic!("Unable to construct large glyph."); }
            }
        }

        println!("Variants count: {:?}", count);

        // We now know how mean repeatable glyphs are required for our
        // construction, so we can create the glyph instructions.
        // We start with the smallest possible glyph.
        // While we are doing this, we will calculate the advance
        // of the entire glyph.
        let mut instructions: Vec<GlyphInstruction> = Vec::new();
        let mut prev_connector = 0;
        let mut glyph_advance: f64 = 0.0;
        let mut first = true;

        for glyph in &construction.parts {
            let repeat = if !glyph.required { count } else { 1 };
            let gly = glyph_metrics(glyph.unicode);
            for _ in 0..repeat {
                let overlap =
                    (min(prev_connector, glyph.start_connector_length) as f64)
                    .max(*MIN_CONNECTOR_OVERLAP as f64);

                if first {
                    glyph_advance += glyph.full_advance as f64;
                    instructions.push(GlyphInstruction {
                        glyph:   gly,
                        overlap: 0.0,
                    });
                    first = false;
                } else {
                    glyph_advance += glyph.full_advance as f64 - overlap;
                    instructions.push(GlyphInstruction {
                        glyph:   gly,
                        overlap: overlap,
                    });
                }

                prev_connector = glyph.end_connector_length;
            }
        }

        println!("Accent advance: {:?} vs Size: {:?}", glyph_advance, size);

        // Now we will calculate how much we need to reduce our overlap
        // to construct a glyph of the desired size.
        let size_difference = size - glyph_advance;
        println!("size diff: {:?}", size_difference);

        // Provided that our constructed glyph is _still_ too large,
        // return this, otherwise distribute the overlap equally
        // amonst each part.
        if size_difference < 0.0 {
            return VariantGlyph::Constructable(direction, instructions)
        }

        let overlap = size_difference / (instructions.len() - 1) as f64;
        println!("overlap: {:?}", overlap);
        for glyph in instructions.iter_mut().skip(1) {
            glyph.overlap -= overlap
        }

        println!("{:?}", instructions);
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
            None        => *self,
        }
    }
}

use std::fmt;
impl fmt::Debug for GlyphInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GlyphInst {{ glyph: 0x{:X}, overlap: {} }}", self.glyph.unicode, self.overlap)
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

    #[test]
    fn can_find_successor() {
        let int = glyph_metrics(0x222B); // Integral
        println!("Int old: {:?}", int);
        println!("Int new: {:?}", int.successor());
    }
}