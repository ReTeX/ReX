pub type Float = f64;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Unit {
    // Internal font units
    Font(Float),

    // Font relative
    Em(Float),
    Ex(Float),

    // Relative to the viewport
    Px(Float),
}