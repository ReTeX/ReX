use fp;

pub type Float = f64;
pub type FixedPoint = fp::F24P8;
pub type FontUnit = FixedPoint;

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