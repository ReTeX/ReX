// First determine if we have the glyph in question.
// If there isn't, we will typeset the base as normal.
let base = layout(&mut [ acc.base.borrow_mut() ], style.cramped());

let accent_glyph = match glyph_metrics[acc.unicode] {
    Some(acc) => acc,
    None      => {
        warn!("Unable to find glyph for {:?}", CMD);
        result.add_node(&mut [ acc.nucleus.borrow_mut() ], style);
    };

let accent_variant = accent_glyph
    .horz_variant(base.width, Variant::Smaller);
let accent = accent_variant.layout(style);

// The attachement point of the base will be
// determined by
//   (a) None symbol:  width / 2.0,
//   (b) Symbol:
//      1. Attachment point (if there is one)
//      2. Otherwise: (width + ic) / 2.0

let base_offset = if base.len() != 1 {
        base_layout.width / 2.0
    } else if let Some(ref sym) = base[0].is_symbol() {
        let glyph = glyph_metrics[sym.unicode];
        if glyph.attachment != 0 {
            glyph.attachment
        } else {
            (glyph.width + sym.italics) / 2.0
        }
    } else {
        base_layout.width / 2.0
    };

// The attachment point of the accent will be
//   (a) None symbol: width / 2.0 [ from constructed glyphs ]
//   (b) Symbol:
//      1. Attachment point (if there is one)
//      2. Otherwise width / 2.0

let acc_offset = match accent {
        Variant::Glyph(ref sym)  => {
            let glyph = glyph_metrics[sym.unicode];
            if glyph.attachment != Pixels(0.0) {
                glyph.attachment
            } else {
                // For glyphs without attachmens, we must
                // also account for combining glyphs
                ((sym.bbox.3 - sym.bbox.1) + sym.italics) / 2.0
            }
        },

        Variant::Constructed(_, _) =>
            wide_accent.width / 2.0,
    };

// Do not place the accent any _further_ than you would if given
// an `x` character in the current style.
let delta = -1. * nucleus.height
    .min(ACCENT_BASE_HEIGHT.scaled(style));

// By not placing an offset on this vbox, we are assured that the 
// baseline will match the baseline of `base.as_node()`
result.add_node(vbox![
    hbox!(kern!(horz: acc_offset - base_offset), accent_layout),
    kern!(vert: delta),
    base.as_node()]);
