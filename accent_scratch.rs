// First determine if we have the glyph in question.
let accent = get_accent();
base = acc.nucleus.as_ref();  // &[ParseNode]

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
        if glyph.attachment != Pixels(0.0) {
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
                (glyph.width + glyph.italics) / 2.0
            }
        },
        
        // TODO: Answer, will this offset modify the 
        //       construction of the wide_accent?
        //       If not, construct accent first.
        Variant::Constructed(_, _) => {
            wide_accent.width / 2.0 
        }
    };
    
