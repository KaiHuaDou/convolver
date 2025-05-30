use std::cmp::{PartialEq, PartialOrd};
use std::fmt::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

pub trait ColorValue = Copy
    + Clone
    + Sync
    + Send
    + PartialOrd
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + From<u8>
    + From<f32>
    + Into<f32>
    + Clamp;

pub trait Clamp {
    fn clamp(&self, channel: u8) -> Self;
}

macro_rules! impl_newtype {
    ($type_name:ident, $inner_type:ty, $max0:expr, $max1:expr, $max2:expr, $min0:expr, $min1:expr, $min2:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $type_name(pub $inner_type);

        impl Clamp for $type_name {
            #[inline]
            fn clamp(&self, channel: u8) -> $type_name {
                match channel {
                    0 => $type_name(self.0.min($max0).max($min0)),
                    1 => $type_name(self.0.min($max1).max($min1)),
                    2 => $type_name(self.0.min($max2).max($min2)),
                    _ => unreachable!(),
                }
            }
        }

        impl Add for $type_name {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                $type_name(self.0 + other.0)
            }
        }

        impl Sub for $type_name {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                $type_name(self.0 - other.0)
            }
        }

        impl Mul for $type_name {
            type Output = Self;
            fn mul(self, other: Self) -> Self {
                $type_name(self.0 * other.0)
            }
        }

        impl Div for $type_name {
            type Output = Self;
            fn div(self, other: Self) -> Self {
                $type_name(self.0 / other.0)
            }
        }

        impl Rem for $type_name {
            type Output = Self;
            fn rem(self, other: Self) -> Self {
                $type_name(self.0 % other.0)
            }
        }

        impl Mul<$inner_type> for $type_name {
            type Output = Self;
            fn mul(self, scalar: $inner_type) -> Self {
                $type_name(self.0 * scalar)
            }
        }

        impl Div<$inner_type> for $type_name {
            type Output = Self;
            fn div(self, scalar: $inner_type) -> Self {
                $type_name(self.0 / scalar)
            }
        }

        impl AddAssign for $type_name {
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }

        impl SubAssign for $type_name {
            fn sub_assign(&mut self, other: Self) {
                self.0 -= other.0;
            }
        }

        impl MulAssign for $type_name {
            fn mul_assign(&mut self, other: Self) {
                self.0 *= other.0;
            }
        }

        impl DivAssign for $type_name {
            fn div_assign(&mut self, other: Self) {
                self.0 /= other.0;
            }
        }

        impl RemAssign for $type_name {
            fn rem_assign(&mut self, other: Self) {
                self.0 %= other.0;
            }
        }

        impl MulAssign<$inner_type> for $type_name {
            fn mul_assign(&mut self, scalar: $inner_type) {
                self.0 *= scalar;
            }
        }

        impl DivAssign<$inner_type> for $type_name {
            fn div_assign(&mut self, scalar: $inner_type) {
                self.0 /= scalar;
            }
        }

        impl From<u8> for $type_name {
            fn from(x: u8) -> Self {
                $type_name(<$inner_type>::from(x))
            }
        }

        impl From<$type_name> for f32 {
            fn from(val: $type_name) -> f32 {
                val.0 as f32
            }
        }

        impl From<f32> for $type_name {
            fn from(x: f32) -> Self {
                $type_name(x as $inner_type)
            }
        }

        impl Display for $type_name {
            fn fmt(&self, f: &mut Formatter) -> Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

impl_newtype!(Rgba, u8, 255u8, 255u8, 255u8, 0u8, 0u8, 0u8);
impl_newtype!(Hsla, f32, 180.0f32, 1.0f32, 1.0f32, -180.0f32, 0.0f32, 0.0f32);
impl_newtype!(Luva, f32, 100.0f32, 176.0f32, 108.0f32, 0.0f32, -84.0f32, -135.0f32);
