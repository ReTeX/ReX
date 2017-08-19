use std::cmp::{min, max};

use font_types::{FontUnit, Direction, VariantGlyph, Glyph, GlyphPart, GlyphInstruction};
use stix::variants::VERT_VARIANTS;
use stix::variants::HORZ_VARIANTS;
use stix::constants::MIN_CONNECTOR_OVERLAP;
use stix::glyph_metrics;

//const GLYPH_LIMIT: u16 = 250;

pub trait Variant {
    fn successor(&self) -> Glyph;
    fn vert_variant(&self, size: FontUnit) -> VariantGlyph;
    fn horz_variant(&self, size: FontUnit) -> VariantGlyph;
}

impl Variant for Glyph {
    fn vert_variant(&self, size: FontUnit) -> VariantGlyph {
        let variants = match VERT_VARIANTS.get(&self.unicode) {
            Some(variants) => variants,
            None => {
                debug!("unable to find variant for 0x{:X}", self.unicode);
                return VariantGlyph::Replacement(*self);
            }
        };

        // Check if any replacement glyphs meet the requirement.
        for &glyph in variants.replacements {
            if glyph.advance >= size {
                trace!("using replacement 0x{:X}, size {}", 
                       glyph.unicode, glyph.advance);
                let replacement = glyph_metrics(glyph.unicode);
                return VariantGlyph::Replacement(replacement);
            }
        }

        // Otherwise we check for constructable glyphs.
        // In the scenario that none of the replacement glyphs match the desired
        // advance, and there is no constructable glyph, we return the largest
        // replacement glyph.
        let construction = match variants.constructable {
            None => {
                let replacement =
                    glyph_metrics(variants
                        .replacements
                        .last()
                        .map(|g| g.unicode)
                        .unwrap_or(self.unicode));
                trace!("no constructable glyphs, \
                        using largest replacement: 0x{:X} with size {}",
                        replacement.unicode,
                        replacement.advance);
                return VariantGlyph::Replacement(replacement);
            },
            Some(ref c) => c,
        };

        // Calculate the metrics for a variant at least as large as size.
        let (repeats, diff_ratio) = smallest_upper_bound(&construction.parts, size);

        // Construct the variant glyph
        let mut instructions = Vec::with_capacity(2*construction.parts.len());
        let mut prev_connector = FontUnit::from(0);
        let mut first = true;
        let mut size = FontUnit::from(0);

        for glyph in construction.parts {
            let repeat = if !glyph.required { repeats } else { 1 };
            for _ in 0..repeat {
                let overlap = if first {
                    first = false;
                    FontUnit::from(0)
                } else {
                    // linear interpolation: x * A + (1 - x) * B
                    //  d * max_overlap + (1 - d) * MIN_CONNECTOR_OVERLAP
                    let max = max_overlap(prev_connector, glyph);
                    let overlap = (FontUnit::from(1) - diff_ratio) * max
                        + diff_ratio * MIN_CONNECTOR_OVERLAP;
                    trace!("ret: (1 - {0:?})*{1:?} + {0:?}*{2:?} = {3:?}",
                        diff_ratio, max, MIN_CONNECTOR_OVERLAP, overlap);
                    overlap
                };
                let metrics = glyph_metrics(glyph.unicode);
                instructions.push(GlyphInstruction {
                    glyph: metrics,
                    overlap: overlap
                });
                prev_connector = min(glyph.end_connector_length, glyph.full_advance / 2);
                size += metrics.height() - metrics.depth() - overlap;
            }
        }

        trace!("Glyph height: {:?}", size);
        trace!("{:?}", instructions);
        VariantGlyph::Constructable(Direction::Vertical, instructions)
    }

    fn horz_variant(&self, size: FontUnit) -> VariantGlyph {
        trace!("0x{:X} -- required size: {}", self.unicode, size);
        let variants = match HORZ_VARIANTS.get(&self.unicode) {
            Some(variants) => variants,
            None => {
                debug!("unable to find variant for 0x{:X}", self.unicode);
                return VariantGlyph::Replacement(*self);
            }
        };

        // Check to see if any of the replacement glyphs meet the requirement.
        for idx in 0..variants.replacements.len() {
            if variants.replacements[idx].advance >= size {
                if idx == 0 {
                    trace!("current glyph is largest glyph smaller than required size");
                    return VariantGlyph::Replacement(*self);
                } else {
                    let glyph = variants.replacements[idx - 1];
                    trace!("using replacement 0x{:X}, size {}", glyph.unicode, glyph.advance);
                    return VariantGlyph::Replacement(glyph_metrics(glyph.unicode));
                }
            }
        }

        // Otherwise check
        unimplemented!()
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

/// Construct the smallest variant that is larger than the given size.
/// With the number of glyphs required to construct the variant is larger
/// than `ITERATION_LIMIT` we return `None`.
fn smallest_upper_bound(parts: &[GlyphPart], size: FontUnit) -> (u16, FontUnit) {
    trace!("Finding smallest variant larger than {}", size);
    let (small, large, _) = advance_without_optional(parts);
    if large >= size {
        trace!("using smallest variant glyph, {} <= smallest <= {}", small, large);
        return (0, FontUnit::from(0))
    }

    // Otherwise, check the next largest variant with optional glyphs included.
    let (mut small, mut large, opt_small, opt_large) = advance_with_optional(parts);
    if large >= size {
        let size_difference = min(size, large) - small;
        let difference_ratio = size_difference / FontUnit::from(parts.len() as u32);
        trace!("Optional glyphs: 1, Difference ratio: {}", difference_ratio);
        return (1, difference_ratio);
    } else {
        // We need to find the smallest integer k that satisfies:
        //     large + k * opt_large >= size
        // This is solved by:
        //     (size - large) / opt_large <= k
        // So take k = ceil[ (size - large) / opt_large ]
        let k = u32::from( (size - large) / opt_large ) + 1;
        trace!("k = ({} - {}) / {} = {}", size, large, opt_large, k);
        small += k * opt_small;
        large += k * opt_large;
        trace!("new size: {} <= advance <= {}", small, large);

        //  A---o---B, percentage: (o - A) / (B - A)
        // o  A-----B, percentage: 0 (smallest glyph).
        // Need small + diff_ratio * (opt_large - opt_small) = size
        if small >= size {
            return (k as u16 + 1, 0.into());
        }

        let difference_ratio = f64::from(size - small) / f64::from(large - small);
        trace!("Difference ratio: ({:?} - {:?}) / ({:?} - {:?}) = {:?}",
            size, small, large, small, difference_ratio);
        trace!("New size: {}", small 
                + k * FontUnit::from(difference_ratio) * (opt_large - opt_small));
        return (k as u16 + 1, difference_ratio.into())
    }
}

// Measure the _smallest_ size of a glyph construction with the given number
// of repeatable glyphs.  This method returns the number of connectors required
// and the largest size of the variant with that number of glyphs.
fn variant_greatest_lower_bound(parts: &[GlyphPart], size: FontUnit) -> Option<(u32, FontUnit)> {
    trace!("Finding largest variant smaller than {}", size);

    // If the smallest extendable variant is too large, we should instead take
    // the largest replacement glyph.  We return `None` to indicate this.
    let (small, _, required) = advance_without_optional(parts);
    if small >= size {
        trace!("all constructable glyphs are too large, smallest: {}", small);
        return None;
    }

    // Otherwise calculate the size of including a set of optional glyphs.
    let (mut small, mut large, opt_small, opt_large) = advance_with_optional(parts);
    if small >= size {
        let size_difference = min(size, large) - small;
        let difference_ratio = size_difference / FontUnit::from(required);
        trace!("Optional glyphs: 1, Difference ratio: {}", difference_ratio);
        return Some((1, difference_ratio));
    }

    // Otherwise determine the number of optional glyphs required to reach desired size.
    // We need to find the smallest integer k such that:
    //     advance_small + k*connector_advance >= size
    // This is solved by:
    //     (size - advance_small) / connector_advance <= k
    // The result will have k connectors with size: advance + (k-1)*connector_advance
    // Since we round towards zero, k below is k-1 mentioned above.
    let k = u32::from((size - small) / opt_small);
    small += k * opt_small;
    large += k * opt_large;
    trace!("k = ({} - {})/ {} = {}", size, small, opt_small, k);
    trace!("{} <= advance <= {}", small, large);

    let size_difference = min(size, large) - small;
    let difference_ratio = size_difference / FontUnit::from(required);
    trace!("Difference ratio: {}", difference_ratio);

    Some((k, difference_ratio))
}

/// Calculate the advance of the smallest variant with exactly one set of optional
/// connectors. This returns a tuple: the first element states the advance of a
/// variant with one set of optional connectors, the second element states the
/// increase in advance for each additional connector.
fn advance_with_optional(parts: &[GlyphPart]) -> (FontUnit, FontUnit, FontUnit, FontUnit) {
    let mut advance_small = FontUnit::from(0);
    let mut advance_large = MIN_CONNECTOR_OVERLAP;
    let mut connector_small = FontUnit::from(0);
    let mut connector_large = FontUnit::from(0);
    let mut prev_connector = FontUnit::from(0);

    // Calculate the legnth with exactly one connector
    for glyph in parts {
        let overlap = max_overlap(prev_connector, glyph);
        advance_small += glyph.full_advance - overlap;
        advance_large += glyph.full_advance - MIN_CONNECTOR_OVERLAP;
        prev_connector = min(glyph.end_connector_length, glyph.full_advance / 2);

        // Keep record of the advance each additional connector adds
        if !glyph.required {
            let overlap = max_overlap(glyph.start_connector_length, glyph);
            connector_small += glyph.full_advance - overlap;
            connector_large += glyph.full_advance - MIN_CONNECTOR_OVERLAP;
        }
    }

    trace!("variant with optional glyphs: {} <= advance <= {}", advance_small, advance_large);
    trace!("advance from optional glyphs: {} <= advance <= {}",
        connector_small, connector_large);
    (advance_small, advance_large, connector_small, connector_large)
}

fn advance_without_optional(parts: &[GlyphPart]) -> (FontUnit, FontUnit, u32) {
    let mut advance_small = FontUnit::from(0);
    let mut advance_large = MIN_CONNECTOR_OVERLAP;
    let mut prev_connector = FontUnit::from(0);
    let mut required = 0;

    for glyph in parts.iter().filter(|glyph| !glyph.required) {
        let overlap = max_overlap(prev_connector, glyph);
        advance_large += glyph.full_advance - overlap;
        advance_small += glyph.full_advance - MIN_CONNECTOR_OVERLAP;
        prev_connector = min(glyph.end_connector_length, glyph.full_advance / 2);
        required += 1;
    }

    (advance_small, advance_large, required)
}

fn max_overlap(left: FontUnit, right: &GlyphPart) -> FontUnit {
    let overlap = min(left, right.start_connector_length);
    let overlap = min(overlap, right.full_advance / 2);
    max(overlap, MIN_CONNECTOR_OVERLAP)
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
