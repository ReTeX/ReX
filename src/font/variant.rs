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
        trace!("finding variant for 0x:{:X}, size: {}", self.unicode, size);
        let variants = match VERT_VARIANTS.get(&self.unicode) {
            Some(variants) => variants,
            None => {
                debug!("unable to find variants for 0x{:X}", self.unicode);
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
                trace!("no constructable glyphs found"); 
                trace!("using largest replacement 0x{:X}, size {}",
                        replacement.unicode, replacement.advance);
                return VariantGlyph::Replacement(replacement);
            },
            Some(ref constructable) => constructable,
        };

        // Calculate the metrics for a variant at least as large as size.
        let (repeats, diff_ratio) = smallest_upper_bound(&construction.parts, size);
        let instructions = construct_glyphs(&construction.parts, repeats, diff_ratio);
        VariantGlyph::Constructable(Direction::Vertical, instructions)
    }

    fn horz_variant(&self, size: FontUnit) -> VariantGlyph {
        trace!("finding variant for 0x:{:X}, size: {}", self.unicode, size);
        let variants = match HORZ_VARIANTS.get(&self.unicode) {
            Some(variants) => variants,
            None => {
                debug!("unable to find variant for 0x{:X}", self.unicode);
                return VariantGlyph::Replacement(*self);
            }
        };

        // Check for replacement glyphs that meet the desired size first.
        // We want the largest variant that is _smaller_ than the given size.
        for idx in 0..variants.replacements.len() {
            if variants.replacements[idx].advance >= size {
                if idx <= 0 {
                    trace!("replacement glyphs are too large");
                    return VariantGlyph::Replacement(*self);
                } else {
                    let replacement = variants.replacements[idx-1];
                    let glyph = glyph_metrics(replacement.unicode);
                    trace!("using replacement 0x{:X}, size {}", 
                           replacement.unicode, replacement.advance);
                    return VariantGlyph::Replacement(glyph);
                }
            }
        }

        // In case all constructable glyphs are too large or non-existant,
        // fall back to the largest replacement glyph if possible.
        let backup = variants.replacements
                .last()
                .map(|g| g.unicode)
                .unwrap_or(self.unicode);

        // otherwise check for constructable glyphs.
        let construction = match variants.constructable {
            None => {
                let replacement = glyph_metrics(backup);
                trace!("no constructable glyphs");
                trace!("using largest replacement 0x{:X}, size {}",
                        replacement.unicode,
                        replacement.advance);
                return VariantGlyph::Replacement(replacement);
            }
            Some(ref constructable) => constructable,
        };

        // Calculate the metrics for a variant at least as large as size.
        let (repeats, diff) = match greatest_lower_bound(&construction.parts, size) {
            Some(ret) => ret,
            None => {
                let replacement = glyph_metrics(backup);
                trace!("constructable glyphs are too large");
                trace!("using largest replacement 0x{:X}, size {}",
                        replacement.unicode,
                        replacement.advance);
                return VariantGlyph::Replacement(replacement);
            }
        };

        let instructions = construct_glyphs(&construction.parts, repeats, diff);
        VariantGlyph::Constructable(Direction::Horizontal, instructions)
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

fn construct_glyphs(
    parts: &[GlyphPart], 
    repeats: u16, 
    diff_ratio: FontUnit) 
-> Vec<GlyphInstruction> {
    // Construct the variant glyph
    let mut instructions = Vec::with_capacity(2*parts.len());
    let mut prev_connector = FontUnit::from(0);
    let mut first = true;
    trace!("diff: {:?}, repeats: {}", diff_ratio, repeats);

    for glyph in parts {
        let repeat = if !glyph.required { repeats } else { 1 };
        for _ in 0..repeat {
            let overlap = if first {
                first = false;
                FontUnit::from(0)
            } else {
                // linear interpolation
                //  d * max_overlap + (1 - d) * MIN_CONNECTOR_OVERLAP
                let max = max_overlap(prev_connector, glyph);
                let overlap = (FontUnit::from(1) - diff_ratio) * max
                    + diff_ratio * MIN_CONNECTOR_OVERLAP;
                overlap
            };
            let metrics = glyph_metrics(glyph.unicode);
            instructions.push(GlyphInstruction {
                glyph: metrics,
                overlap: overlap
            });
            prev_connector = min(glyph.end_connector_length, glyph.full_advance / 2);
        }
    }

    trace!("{:?}", instructions);
    instructions
}

/// Construct the smallest variant that is larger than the given size.
/// With the number of glyphs required to construct the variant is larger
/// than `ITERATION_LIMIT` we return `None`.
fn smallest_upper_bound(parts: &[GlyphPart], size: FontUnit) -> (u16, FontUnit) {
    let (small, large) = advance_without_optional(parts);
    if large >= size {
        trace!("using smallest variant glyph, {} <= smallest <= {}", small, large);
        return (0, FontUnit::from(0))
    }

    // Otherwise, check the next largest variant with optional glyphs included.
    let (mut small, mut large, opt_small, opt_large) = advance_with_optional(parts);
    if large >= size {
        let diff_ratio = f64::from(size - small) / f64::from(large - small);
        trace!("Optional glyphs: 1, Difference ratio: {:2}", diff_ratio);
        return (1, diff_ratio.into());
    } 

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
    (k as u16 + 1, difference_ratio.into())
}

/// Measure the _largest_ a glyph construction _smaller_ than the given size. 
/// If all constructions are larger than the given size, return `None`.
/// Otherwise return the number of optional glyphs required and the difference
/// ratio to obtain the desired size.
fn greatest_lower_bound(
    parts: &[GlyphPart], 
    size: FontUnit) 
-> Option<(u16, FontUnit)> {
    let (small, large) = advance_without_optional(parts);
    if small >= size {
        trace!("all constructable glyphs are too large, smallest: {}", small);
        return None;
    }

    // Otherwise calculate the size of including one set of optional glyphs.
    let (mut ssmall, mut llarge, opt_small, opt_large) = advance_with_optional(parts);

    // If the smallest constructable with optional glyphs is too large we
    // use no optional glyphs.
    // TODO: Do something better if `large == small`.
    if ssmall >= size {
        let diff_ratio = f64::from(size - small) / f64::from(large - small);
        let diff_ratio = diff_ratio.min(1.0);
        trace!("optional glyphs make construction too large, using none");
        trace!("diff_ratio = {:.2}", diff_ratio);
        return Some((0, diff_ratio.into()));
    }

    // Determine the number of additional optional glyphs required to achieve size.
    // We need to find the smallest integer k such that:
    //     ssmall + k*opt_small >= size
    // This is solved by:
    //     (size - ssmall) / opt_small <= k
    // Which is solved by: k = floor[ (size - smmal) / opt_small ]
    // Since we round towards zero, floor is not necessary.
    let k = u32::from((size - ssmall) / opt_small);
    trace!("k = ({} - {})/ {} = {}", size, ssmall, opt_small, k);

    ssmall += k * opt_small;
    llarge += k * opt_large;
    let diff_ratio = f64::from(size - ssmall) / f64::from(llarge - ssmall);
    let diff_ratio = FontUnit::from(diff_ratio.min(1.0).max(0.0));

    trace!("{} <= advance <= {}", ssmall, llarge);
    trace!("Difference ratio: {}", diff_ratio);
    Some((k as u16 + 1, diff_ratio))
}

/// Calculate the advance of the smallest variant with exactly one set of optional
/// connectors. This returns a tuple: the first element states the advance of a
/// variant with one set of optional connectors, the second element states the
/// increase in advance for each additional connector.
fn advance_with_optional( parts: &[GlyphPart]) -> (FontUnit, FontUnit, FontUnit, FontUnit) {
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

fn advance_without_optional(parts: &[GlyphPart]) -> (FontUnit, FontUnit) {
    let mut advance_small = FontUnit::from(0);
    let mut advance_large = MIN_CONNECTOR_OVERLAP;
    let mut prev_connector = FontUnit::from(0);

    for glyph in parts.iter().filter(|glyph| !glyph.required) {
        let overlap = max_overlap(prev_connector, glyph);
        advance_small += glyph.full_advance - overlap;
        advance_large += glyph.full_advance - MIN_CONNECTOR_OVERLAP;
        prev_connector = min(glyph.end_connector_length, glyph.full_advance / 2);
    }

    (advance_small, advance_large)
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
