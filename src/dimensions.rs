use std::convert::From;
use std::cmp::{PartialOrd, PartialEq};
use std::fmt;
use std::fmt::{Display, Debug};
use std::ops::{Add, AddAssign, Deref, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Unit {
    // Internal font units
    Font(f64),

    // Font relative
    Em(f64),
    Ex(f64),

    // Relative to the viewport
    Px(f64),
}

pub trait Unital: Copy + Clone + Default +
    Display + Debug +
    Add + AddAssign +
    Mul + MulAssign +
    PartialOrd + PartialEq +
    Sub + SubAssign +
    Into<f64> { }

impl Unital for u32 {}
impl Unital for i16 {}
impl Unital for u16 {}

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
#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Pixels(pub f64);

impl Deref for Pixels {
    type Target = f64;
    fn deref(&self) -> &f64 { &self.0 }
}

impl Add for Pixels {
    type Output = Pixels;
    fn add(self, rhs: Pixels) -> Pixels { Pixels(self.0 + rhs.0) }
}

impl Add<f64> for Pixels {
    type Output = Pixels;
    fn add(self, rhs: f64) -> Pixels { Pixels(self.0 + rhs) }
}

impl Add<Pixels> for f64 {
    type Output = Pixels;
    fn add(self, rhs: Pixels) -> Pixels { Pixels(self + rhs.0)}
}

impl Sub for Pixels {
    type Output = Pixels;
    fn sub(self, rhs: Pixels) -> Pixels { Pixels(self.0 - rhs.0) }
}

impl AddAssign for Pixels {
    fn add_assign(&mut self, rhs: Pixels) { *self = *self + rhs; }
}

impl Mul for Pixels {
    type Output = Pixels;
    fn mul(self, rhs: Pixels) -> Pixels { Pixels(self.0 * rhs.0) }
}

impl Mul<f64> for Pixels {
    type Output = Pixels;
    fn mul(self, rhs: f64) -> Pixels { Pixels(self.0 * rhs) }
}

impl Mul<Pixels> for f64 {
    type Output = Pixels;
    fn mul(self, rhs: Pixels) -> Pixels { Pixels(self * rhs.0) }
}

impl MulAssign for Pixels {
    fn mul_assign(&mut self, rhs: Pixels) { *self = *self * rhs; }
}

impl Pixels {
    pub fn max(self, rhs: Pixels) -> Pixels { Pixels((*self).max(rhs.0)) }
    pub fn min(self, rhs: Pixels) -> Pixels { Pixels((*self).min(rhs.0)) }
}

impl Display for Pixels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", **self)
    }
}