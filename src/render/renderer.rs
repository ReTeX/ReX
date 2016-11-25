use std::fmt;

/// Used to determine direciton
/// of the current renderer

enum Orientation {
    Horizontal,
    Vertical,
}

/// This struct will drive the rendering
/// of SVGs.

struct SvgRenderer {
    /// The current `x` position of the cursor.
    x: f64,
    /// The current `y` position of the cursor.
    y: f64,
    /// The `LayoutNode`s which need to be rendered.
    nodes: Vec<LayoutNode>,
    /// The `Orientation` of the rendered.  This changes
    /// whether we are within a `HorizontalBox` which places
    /// glyphs from left to right or a `VerticalBox` which
    /// places glyphs from top to bottom.
    orientation: Orientation,

    font_size: Pixels,
    horz_padding: Pixels,
    vert_padding: Pixels,
}

impl Renderer {
    fn new() -> Renderer {

    }

    fn render(&self) {

    }
}

fn render(&self)

pub trait Render {
    fn render(&self, &mut String) -> Result<(), String>;
}