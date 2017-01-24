use std::convert::From;
use std::cmp::{PartialOrd, PartialEq};
use std::fmt;
use std::fmt::{Display, Debug};
use std::ops::{Add, AddAssign, Deref, Div, Mul, MulAssign, Sub, SubAssign};
use fp;

pub type Float = f64;
pub type FixedPoint = fp::F24P8;

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

// #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub type FontUnit = FixedPoint;

// impl From<FixedPoint> for FontUnit {
//     fn from(fp: FixedPoint) -> FontUnit {
//         FontUnit(fp)
//     }
// }

// // Convenience conversion to FontUnit
// macro_rules! impl_fontunit_conv {
//     ($($ty:ty),*) => (
//         $(
//         impl From<$ty> for FontUnit {
//             fn from(n: $ty) -> FontUnit {
//                 FontUnit::from(FixedPoint::from(n as i32))
//             }
//         }
//         )*
//     )
// }

// impl_fontunit_conv! (u8, i8, u16, i16, i32);

// impl Add for FontUnit
// {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         FontUnit(self.0 + rhs.0)
//     }
// }

// impl AddAssign for FontUnit
// {
//     fn add_assign(&mut self, rhs: Self) {
//         self.0 = self.0 + rhs.0;
//     }
// }

// impl Mul for FontUnit
// {
//     type Output = Self;
//     fn mul(self, rhs: Self) -> Self {
//         FontUnit(self.0 * rhs.0)
//     }
// }

// impl MulAssign for FontUnit
// {
//     fn mul_assign(&mut self, rhs: Self) {
//         self.0 = self.0 * rhs.0;
//     }
// }

// impl Sub for FontUnit
// {
//     type Output = Self;
//     fn sub(self, rhs: Self) -> Self {
//         FontUnit(self.0 - rhs.0)
//     }
// }

// impl SubAssign for FontUnit
// {
//     fn sub_assign(&mut self, rhs: Self) {
//         self.0 = self.0 - rhs.0;
//     }
// }

// impl Display for FontUnit {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "FontUnit({})", self.0)
//     }
// }

// impl From<FontUnit> for Unit {
//     fn from(unit: FontUnit) -> Unit {
//         Unit::Font(unit.0.into())
//     }
// }

// impl From<FontUnit> for f64 {
//     fn from(unit: FontUnit) -> f64 {
//         unit.0.into()
//     }
// }

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
