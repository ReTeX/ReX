use std::convert::From;
use std::cmp::{PartialOrd, PartialEq};
use std::fmt;
use std::fmt::{Display, Debug};
use std::ops::{Add, AddAssign, Deref, Div, Mul, MulAssign, Sub, SubAssign};

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

pub trait Unital: Copy + Clone + Default +
    Display + Debug +
    Add + AddAssign +
    Mul + MulAssign +
    PartialOrd + PartialEq +
    Sub + SubAssign +
    Into<Float> { }

impl Unital for u32 {}
impl Unital for i16 {}
impl Unital for u16 {}
impl Unital for f64 {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct FontUnit<U: Unital>(pub U);

impl<U: Unital> Add for FontUnit<U>
    where U: Add<Output = U>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        FontUnit(self.0 + rhs.0)
    }
}

impl<U: Unital> AddAssign for FontUnit<U>
    where U: Add<Output = U>
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0;
    }
}

impl<U: Unital> Mul for FontUnit<U>
    where U: Mul<Output = U>
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        FontUnit(self.0 * rhs.0)
    }
}

impl<U: Unital> MulAssign for FontUnit<U>
    where U: Mul<Output = U>
{
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self.0 * rhs.0;
    }
}

impl<U: Unital> Sub for FontUnit<U>
    where U: Sub<Output = U>
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        FontUnit(self.0 - rhs.0)
    }
}

impl<U: Unital> SubAssign for FontUnit<U>
    where U: Sub<Output = U>
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0 - rhs.0;
    }
}

impl<U: Unital> Deref for FontUnit<U> {
    type Target = U;
    fn deref(&self) -> &U {
        &self.0
    }
}

impl<U: Unital> Display for FontUnit<U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontUnit({})", self.0)
    }
}

impl<U: Unital> From<FontUnit<U>> for Unit {
    fn from(unit: FontUnit<U>) -> Unit {
        Unit::Font(unit.0.into())
    }
}

impl<U: Unital> From<FontUnit<U>> for f64 {
    fn from(unit: FontUnit<U>) -> f64 {
        unit.0.into()
    }
}

macro_rules! implement_fontunit {
    ( $($num:ty),* ) => {
        $(
            impl From<$num> for FontUnit<$num> {
                fn from(u: $num) -> Self { FontUnit(u) }
            }
        )*
    }
}

implement_fontunit!{ i16, u32 }

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
