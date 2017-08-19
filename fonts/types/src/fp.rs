use std::ops::*;
use std::fmt::{Formatter, Display, Debug, Error};

const FRACTION_BITS: u8 = 8;
const FRACTION_MASK: i32 = 0xFF;
const FRACTION_VALUE: i32 = 256;
const SIGN_MASK: u32 = 0x8000_0000;
const INT_BITS: u8 = 24;
const BIT_VALUE: f64 = 0.00390625;

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct F24P8 {
    pub bits: i32
}

impl Add for F24P8 {
    type Output = F24P8;
    fn add(self, rhs: Self) -> Self {
        F24P8 { bits: self.bits + rhs.bits }
    }
}

impl AddAssign for F24P8 {
    fn add_assign(&mut self, rhs: Self) {
        self.bits = self.bits + rhs.bits
    }
}

impl Sub for F24P8 {
    type Output = F24P8;
    fn sub(self, rhs: Self) -> Self {
        F24P8 { bits: self.bits - rhs.bits }
    }
}

impl SubAssign for F24P8 {
    fn sub_assign(&mut self, rhs: Self) {
        self.bits = self.bits - rhs.bits
    }
}

impl Mul for F24P8 {
    type Output = F24P8;
    fn mul(self, rhs: Self) -> Self {
        F24P8 { bits: ((self.bits as i64 * rhs.bits as i64) / FRACTION_VALUE as i64) as i32 }
    }
}

impl Div for F24P8 {
    type Output = F24P8;
    fn div(self, rhs: Self) -> Self::Output {
        F24P8 { bits: ((self.bits as i64 * FRACTION_VALUE as i64) / rhs.bits as i64) as i32 }
    }
}

impl Neg for F24P8 {
    type Output = F24P8;
    fn neg(self) -> Self::Output {
        F24P8 { bits: -self.bits }
    }
}

macro_rules! impl_fixed_muldiv {
    ($($ty:ty),*) => (
        $(
        impl Mul<$ty> for F24P8 {
            type Output = F24P8;
            fn mul(self, rhs: $ty) -> Self::Output {
                F24P8 { bits: self.bits * (rhs as i32) }
            }
        }

        impl Mul<F24P8> for $ty {
            type Output = F24P8;
            fn mul(self, rhs: F24P8) -> Self::Output {
                F24P8 { bits: (self as i32) * rhs.bits }
            }
        }

        impl Div<$ty> for F24P8 {
            type Output = F24P8;
            fn div(self, rhs: $ty) -> Self::Output {
                F24P8 { bits: self.bits / (rhs as i32) }
            }
        }
        )*
    )
}

impl_fixed_muldiv! ( u8, i8, u16, i16, u32, i32 );

macro_rules! impl_fixed_conv {
    ($($ty:ty),*) => (
        $(
        impl From<$ty> for F24P8 {
            fn from(n: $ty) -> F24P8 {
                F24P8 { bits: (n as i32) * FRACTION_VALUE }
            }
        }
        )*
    )
}

impl_fixed_conv! ( u8, i8, u16, i16, u32, i32 );

impl From<f64> for F24P8 {
    fn from(f: f64) -> F24P8 {
        assert!(f >= 0.);
        F24P8 { bits: (f * FRACTION_VALUE as f64) as i32 }
    }
}

impl From<F24P8> for f64 {
    fn from(fp: F24P8) -> f64 {
        fp.bits as f64 * BIT_VALUE
    }
}

impl From<F24P8> for u32 {
    fn from(fp: F24P8) -> u32 {
        fp.bits as u32 >> FRACTION_BITS
    }
}

impl Display for F24P8 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}",
            self.bits >> FRACTION_BITS)
    }
}

impl Debug for F24P8 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:2}", self.bits as f64 / (1 << 8) as f64)
    }
}

impl F24P8 {
    pub fn twice(self) -> F24P8 {
        F24P8 { bits: self.bits * 2 }
    }

    pub fn half(self) -> F24P8 {
        F24P8 { bits: self.bits / 2 }
    }

    pub fn ceil(self) -> u32 {
        if self.bits as u32 & 0xFFFF == 0 {
            self.bits as u32 >> FRACTION_BITS
        } else {
            self.bits as u32 + 1 >> FRACTION_BITS
        }
    }

    pub fn from_raw(n: i32) -> F24P8 {
        F24P8 { bits: n }
    }
}
