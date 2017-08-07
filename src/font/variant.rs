use std::cmp::{min, max};

use font_types::{FontUnit, Direction, VariantGlyph, Glyph, GlyphPart, GlyphInstruction};
use stix::variants::VERT_VARIANTS;
use stix::variants::HORZ_VARIANTS;
use stix::constants::MIN_CONNECTOR_OVERLAP;
use stix::glyph_metrics;

const ITERATION_LIMIT: u16 = 500;

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
        trace!(
            "0x{:X}, required size: {}, direction: {:?}",
            self.unicode,
            size,
            direction
        );
        let variants = match direction {
            Direction::Vertical => VERT_VARIANTS.get(&self.unicode),
            Direction::Horizontal => HORZ_VARIANTS.get(&self.unicode),
        };
        let variants = match variants {
            Some(variant) => variant,
            None => {
                debug!("Unable to find variant for 0x{:X}", self.unicode);
                return VariantGlyph::Replacement(*self);
            }
        };

        // First check to see if any of the replacement glyphs meet the requirement.
        // It is assumed that the glyphs are in increasing advance.
        if find_min {
            for idx in 0..variants.replacements.len() {
                if variants.replacements[idx].advance >= size {
                    if idx == 0 {
                        trace!("current glyph is largest glyph smaller than required size");
                        return VariantGlyph::Replacement(*self);
                    } else {
                        let glyph = variants.replacements[idx - 1];
                        trace!(
                            "using replacement 0x{:X} with size {}",
                            glyph.unicode,
                            glyph.advance
                        );
                        return VariantGlyph::Replacement(glyph_metrics(glyph.unicode));
                    }
                }
            }
        } else {
            for &glyph in variants.replacements {
                if glyph.advance >= size {
                    trace!(
                        "using replacement 0x{:X} with size {}",
                        glyph.unicode,
                        glyph.advance,
                    );
                    let replacement = glyph_metrics(glyph.unicode);
                    return VariantGlyph::Replacement(replacement);
                }
            }
        }

        // Next we check for a constructable glyph.
        // In the scenario that none of the replacement glyphs match the desired
        // advance, and there is no constructable glyph, we return the largest
        // replacement glyph.
        let construction = match variants.constructable {
            None => {
                let replacement = glyph_metrics(
                    variants
                        .replacements
                        .last()
                        .map(|g| g.unicode)
                        .unwrap_or(self.unicode),
                );
                trace!(
                    "no constructable glyphs, using largest replacement: 0x{:X} with size {}",
                    replacement.unicode,
                    replacement.advance
                );
                return VariantGlyph::Replacement(replacement);
            }
            Some(ref c) => c,
        };

        // We check for the smallest number of repeatable glyphs
        // that are required to meet our requirements.
        let mut count = 0;
        if find_min {
            while min_size(&construction.parts, count) < size {
                count += 1;
                if count > ITERATION_LIMIT {
                    error!("failed to construct glyph variant; max iteration hit.");
                    return VariantGlyph::Replacement(*self);
                }
            }
            // Current glyph is too large, go back one.
            count = max(0, count - 1);
        } else {
            while max_size(&construction.parts, count) < size {
                count += 1;
                if count > ITERATION_LIMIT {
                    error!("failed to construct glyph variant; max iteration hit.");
                    return VariantGlyph::Replacement(*self);
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

        for &glyph in construction.parts {
            let repeat = if !glyph.required { count } else { 1 };
            let gly = glyph_metrics(glyph.unicode);
            for _ in 0..repeat {
                let overlap = max(
                    min(prev_connector, glyph.start_connector_length),
                    MIN_CONNECTOR_OVERLAP,
                );

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
        trace!("Looking for successor: 0x{:X}", self.unicode);

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

// Measure the _maximum_ size of a glyph construction
// with the given number of repeatable glyphs.
fn max_size(parts: &[GlyphPart], repeats: u16) -> FontUnit {
    // The first glyphs overlap will not count, so start with MIN_CONNECTOR_OVERLAP
    let mut advance = MIN_CONNECTOR_OVERLAP;

    // Add the advance of each glyph, minus their overlap with others.
    // Minimal overlap occurs when each glyph has MIN_CONNECTOR_OVERLAP
    for glyph in parts {
        let count = if !glyph.required { repeats } else { 1 };
        advance += count * (glyph.full_advance - MIN_CONNECTOR_OVERLAP);
    }

    advance
}

// TODO: Double check this.  Sketch it out.
// Measure the _smallest_ size of a glyph construction
// with the given number of repeatable glyphs.
fn min_size(parts: &[GlyphPart], repeats: u16) -> FontUnit {
    let mut advance = MIN_CONNECTOR_OVERLAP;
    let mut prev_connector = FontUnit::from(0);

    // Add the advance of each glyph with _maximal_ overlap.
    for glyph in parts {
        let count = if !glyph.required { repeats } else { 1 };
        for _ in 0..count {
            let overlap = min(prev_connector, glyph.start_connector_length);
            advance += glyph.full_advance - max(overlap, MIN_CONNECTOR_OVERLAP);
            prev_connector = glyph.end_connector_length;
        }
    }

    // We subtracted overlap on the first glyph when we shouldn't
    advance
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
