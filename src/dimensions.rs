use std::convert::From;
use std::cmp::{PartialOrd, PartialEq};
use std::fmt;
use std::fmt::{Display, Debug};
use std::ops::{Add, AddAssign, Deref, Div, Mul, MulAssign, Sub, SubAssign};
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

// At some point in time, everything will be in Pixels for computer display renderings.
#[derive(Copy, Debug, Clone, Default, PartialOrd, PartialEq)]
pub struct Pixels(pub Float);

impl Deref for Pixels {
    type Target = Float;
    fn deref(&self) -> &Float { &self.0 }
}

impl Add for Pixels {
    type Output = Pixels;
    fn add(self, rhs: Pixels) -> Pixels { Pixels(self.0 + rhs.0) }
}

impl Add<Float> for Pixels {
    type Output = Pixels;
    fn add(self, rhs: Float) -> Pixels { Pixels(self.0 + rhs) }
}

impl Add<Pixels> for Float {
    type Output = Pixels;
    fn add(self, rhs: Pixels) -> Pixels { Pixels(self + rhs.0)}
}

impl Sub for Pixels {
    type Output = Pixels;
    fn sub(self, rhs: Pixels) -> Pixels { Pixels(self.0 - rhs.0) }
}

impl SubAssign for Pixels {
    fn sub_assign(&mut self, rhs: Pixels) { *self = *self - rhs; }
}

impl AddAssign for Pixels {
    fn add_assign(&mut self, rhs: Pixels) { *self = *self + rhs; }
}

impl Mul for Pixels {
    type Output = Pixels;
    fn mul(self, rhs: Pixels) -> Pixels { Pixels(self.0 * rhs.0) }
}

impl Mul<Float> for Pixels {
    type Output = Pixels;
    fn mul(self, rhs: Float) -> Pixels { Pixels(self.0 * rhs) }
}

impl Mul<Pixels> for Float {
    type Output = Pixels;
    fn mul(self, rhs: Pixels) -> Pixels { Pixels(self * rhs.0) }
}

impl MulAssign for Pixels {
    fn mul_assign(&mut self, rhs: Pixels) { *self = *self * rhs; }
}

impl Div<Float> for Pixels {
    type Output = Pixels;
    fn div(self, rhs: Float) -> Pixels { Pixels(self.0 / rhs) }
}

impl Pixels {
    pub fn max(self, rhs: Pixels) -> Pixels { Pixels((*self).max(rhs.0)) }
    pub fn min(self, rhs: Pixels) -> Pixels { Pixels((*self).min(rhs.0)) }
}

impl Display for Pixels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}", **self)
    }
}
