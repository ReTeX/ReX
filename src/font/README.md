# Font Layout

+ Font
  + symbols.rs
    - Mapping from TeX Commands to GlyphID and Type
  + metrics.rs
    - Mapping from GlyphID to Glyph metrics
  + variants.rs
    - Collection of variant glyph construction
  + kernings.rs
    - Collection of kernings data
  + input.rs
    - Mapping from Unicode input to GlyphID and Type
  + styles.rs
    - An ID to ID mapping encoding style changes.
  + constants.rs
    - MathTable constants
    - Font scaling (pointsPerEm)

# Additional considerations
  - Style Changes
    - State handled in parser.
    - Requires an ID -> ID mapping. (Parsed from unicode-math?)
    - Applied post symbols.rs, input.rs
  - Size changes
    - Handled by simply scaling?
  
