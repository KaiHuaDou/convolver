use num::*;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Rem, Sub};

macro_rules! impl_color_type {
    ($type:ident, $base:tt) => {
        #[derive(Copy, Clone, PartialEq)]
        pub struct $type(pub $base);

        impl PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl Num for $type {
            type FromStrRadixErr = <$base as Num>::FromStrRadixErr;
            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                <$base as Num>::from_str_radix(str, radix).map($type)
            }
        }

        impl Zero for $type {
            fn zero() -> Self {
                $type($base::zero())
            }
            fn is_zero(&self) -> bool {
                self.0 == $base::zero()
            }
        }

        impl One for $type {
            fn one() -> Self {
                $type($base::one())
            }
        }

        impl Add for $type {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                $type(self.0 + rhs.0)
            }
        }

        impl Sub for $type {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                $type(self.0 - rhs.0)
            }
        }

        impl Mul for $type {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                $type(self.0 * rhs.0)
            }
        }

        impl Div for $type {
            type Output = Self;
            fn div(self, rhs: Self) -> Self {
                $type(self.0 / rhs.0)
            }
        }

        impl Rem for $type {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self {
                $type(self.0 % rhs.0)
            }
        }

        impl NumCast for $type {
            fn from<T: ToPrimitive>(n: T) -> Option<Self> {
                <$base as NumCast>::from(n).map($type)
            }
        }

        impl ToPrimitive for $type {
            fn to_i64(&self) -> Option<i64> {
                self.0.to_i64()
            }
            fn to_u64(&self) -> Option<u64> {
                self.0.to_u64()
            }
            fn to_f64(&self) -> Option<f64> {
                self.0.to_f64()
            }
        }

        impl AsRef<$base> for $type {
            fn as_ref(&self) -> &$base {
                &self.0
            }
        }

        impl From<$base> for $type {
            fn from(v: $base) -> Self {
                $type(v)
            }
        }

        impl From<$type> for $base {
            fn from(v: $type) -> Self {
                v.0
            }
        }
    };
}

impl_color_type!(Rgba, u8);
impl_color_type!(Hsla, f32);
impl_color_type!(Luva, f32);
