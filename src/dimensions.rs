// Units of measurements
//
// pt - point - physical unit: 1/72 inch.
// sp - scaled point - 65536 sp -> 1pt  (1/2^16).
// em - An em is a unit in the field of typography,
//      equal to the currently specified point size.
//      For example, one em in a 16-point typeface is 16 points.
//
// DesignUnits - Specifies the font metric to be converted to device units.
//               This value can be any font metric, including the width of a
//               character or the ascender value for an entire font.
//
// DeviceUnits - Specifies the DesignUnits font metric converted to device units.
//               This value is in the same units as the value specified for DeviceResolution.
//
// DeviceResolution - Specifies number of device units (pixels) per inch.
//                    Typical values might be 300 for a laser printer or 96 for a VGA screen.

/// Font independent units of measurment

// Recommended units for screen: em, px.
// Recommended units for print: em, cm, mm, in, pt, pc
// Not recommended for screen: Absolute (pt, cm, mm, in, pc)

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

// At some point in time, everything will be in Pixels for computer display renderings.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Pixels(pub f64);

use std::ops::Deref;
impl Deref for Pixels {
    type Target = f64;
    fn deref(&self) -> &f64 { &self.0 }
}

use std::ops::{ Add, AddAssign, Mul, MulAssign, Sub };
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

use std::fmt;
impl fmt::Display for Pixels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", **self)
    }
}