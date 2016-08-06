// OpenAOE: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2016 Kevin Fuller
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use num::{Bounded, FromPrimitive, Num, One, ToPrimitive, Zero};
use std::fmt::{self, Debug, Display};
use std::ops::*;

const SCALE_BITS: i32 = 14;

/// 32-bit fixed point number
#[derive(Eq, PartialEq, Copy, Clone, Default, Ord, PartialOrd)]
pub struct Fixed {
    scaled: i32,
}

impl Fixed {
    fn new(scaled: i32) -> Fixed {
        Fixed { scaled: scaled }
    }

    pub fn abs(&self) -> Fixed {
        Fixed::new(self.scaled.abs())
    }

    pub fn sqrt(&self) -> Fixed {
        if *self == Fixed::one() {
            return *self;
        }
        if *self < Fixed::zero() {
            panic!("Tried to take the square root of negative number");
        }

        let epsilon: Fixed = 0.0001.into();
        let mut last = *self;
        let mut root = *self;
        loop {
            root = Fixed::from(0.5) * (root + *self / root);
            if (last - root).abs() <= epsilon {
                break;
            }
            last = root;
        }
        root
    }
}

impl ToPrimitive for Fixed {
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        Some((self.scaled >> SCALE_BITS) as i32)
    }

    #[inline]
    fn to_i64(&self) -> Option<i64> {
        Some((self.scaled >> SCALE_BITS) as i64)
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        Some((self.scaled >> SCALE_BITS) as u64)
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        Some((self.scaled as f64) / ((1 << SCALE_BITS) as f64))
    }

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        Some((self.scaled as f32) / ((1 << SCALE_BITS) as f32))
    }
}

impl FromPrimitive for Fixed {
    #[inline]
    fn from_i32(val: i32) -> Option<Fixed> {
        Some(Fixed::new(val << SCALE_BITS))
    }

    #[inline]
    fn from_i64(val: i64) -> Option<Fixed> {
        Some(Fixed::new((val << SCALE_BITS) as i32))
    }

    #[inline]
    fn from_u64(val: u64) -> Option<Fixed> {
        Some(Fixed::new((val << SCALE_BITS) as i32))
    }

    #[inline]
    fn from_f32(val: f32) -> Option<Fixed> {
        Some(Fixed::new((val * ((1 << SCALE_BITS) as f32)) as i32))
    }

    #[inline]
    fn from_f64(val: f64) -> Option<Fixed> {
        Some(Fixed::new((val * ((1 << SCALE_BITS) as f64)) as i32))
    }
}

impl Add for Fixed {
    type Output = Fixed;

    #[inline]
    fn add(self, rhs: Fixed) -> Fixed {
        Fixed::new(self.scaled + rhs.scaled)
    }
}

impl AddAssign for Fixed {
    #[inline]
    fn add_assign(&mut self, rhs: Fixed) {
        self.scaled += rhs.scaled;
    }
}

impl Sub for Fixed {
    type Output = Fixed;

    #[inline]
    fn sub(self, rhs: Fixed) -> Fixed {
        Fixed::new(self.scaled - rhs.scaled)
    }
}

impl SubAssign for Fixed {
    #[inline]
    fn sub_assign(&mut self, rhs: Fixed) {
        self.scaled -= rhs.scaled;
    }
}

impl Mul for Fixed {
    type Output = Fixed;

    #[inline]
    fn mul(self, rhs: Fixed) -> Fixed {
        Fixed::new(((self.scaled as i64 * rhs.scaled as i64) >> SCALE_BITS) as i32)
    }
}

impl MulAssign for Fixed {
    #[inline]
    fn mul_assign(&mut self, rhs: Fixed) {
        self.scaled = ((self.scaled as i64 * rhs.scaled as i64) >> SCALE_BITS) as i32;
    }
}

impl Div for Fixed {
    type Output = Fixed;

    #[inline]
    fn div(self, rhs: Fixed) -> Fixed {
        Fixed::new((((self.scaled as i64) << (SCALE_BITS as i64)) / (rhs.scaled as i64)) as i32)
    }
}

impl DivAssign for Fixed {
    #[inline]
    fn div_assign(&mut self, rhs: Fixed) {
        *self = *self / rhs;
    }
}

impl Rem for Fixed {
    type Output = Fixed;

    #[inline]
    fn rem(self, rhs: Fixed) -> Fixed {
        Fixed::new(self.scaled % rhs.scaled)
    }
}

impl RemAssign for Fixed {
    #[inline]
    fn rem_assign(&mut self, rhs: Fixed) {
        self.scaled = self.scaled % rhs.scaled;
    }
}

impl Neg for Fixed {
    type Output = Fixed;

    #[inline]
    fn neg(self) -> Fixed {
        Fixed::new(-self.scaled)
    }
}

impl Zero for Fixed {
    #[inline]
    fn zero() -> Fixed {
        Fixed::new(0)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.scaled == 0
    }
}

impl One for Fixed {
    #[inline]
    fn one() -> Fixed {
        Fixed::new(1 << SCALE_BITS)
    }
}

impl Num for Fixed {
    type FromStrRadixErr = <f64 as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Fixed, Self::FromStrRadixErr> {
        let valf = try!(f64::from_str_radix(str, radix));
        Ok(Fixed::from_f64(valf).unwrap())
    }
}

impl Bounded for Fixed {
    #[inline]
    fn min_value() -> Fixed {
        Fixed::new(i32::min_value())
    }

    #[inline]
    fn max_value() -> Fixed {
        Fixed::new(i32::max_value())
    }
}

pub trait ToFixed {
    fn to_fixed(&self) -> Fixed;
}

macro_rules! impl_to_fixed {
    ($typ:ty, $from_typ:path) => {
        impl ToFixed for $typ {
            #[inline]
            fn to_fixed(&self) -> Fixed {
                $from_typ(*self).unwrap()
            }
        }
    }
}

impl_to_fixed!(isize, FromPrimitive::from_isize);
impl_to_fixed!(i8, FromPrimitive::from_i8);
impl_to_fixed!(i16, FromPrimitive::from_i16);
impl_to_fixed!(i32, FromPrimitive::from_i32);
impl_to_fixed!(i64, FromPrimitive::from_i64);
impl_to_fixed!(usize, FromPrimitive::from_usize);
impl_to_fixed!(u8, FromPrimitive::from_u8);
impl_to_fixed!(u16, FromPrimitive::from_u16);
impl_to_fixed!(u32, FromPrimitive::from_u32);
impl_to_fixed!(u64, FromPrimitive::from_u64);
impl_to_fixed!(f32, FromPrimitive::from_f32);
impl_to_fixed!(f64, FromPrimitive::from_f64);

macro_rules! impl_from {
    ($typ:ty) => {
        impl From<$typ> for Fixed {
            #[inline]
            fn from(val: $typ) -> Fixed {
                val.to_fixed()
            }
        }
    }
}

impl_from!(isize);
impl_from!(i8);
impl_from!(i16);
impl_from!(i32);
impl_from!(i64);
impl_from!(usize);
impl_from!(u8);
impl_from!(u16);
impl_from!(u32);
impl_from!(u64);
impl_from!(f32);
impl_from!(f64);

impl Display for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_f64().unwrap())
    }
}

impl Debug for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Fixed({}: {}/{})",
               self.to_f64().unwrap(),
               self.scaled,
               1 << SCALE_BITS)
    }
}

#[cfg(test)]
mod tests {
    use num::{Bounded, FromPrimitive, Num, One, ToPrimitive, Zero};
    use super::{Fixed, SCALE_BITS, ToFixed};

    #[test]
    fn test_convert() {
        let verify = |f: Fixed| assert_eq!(5 << SCALE_BITS, f.scaled);
        verify(Fixed::from_i32(5).unwrap());
        verify(Fixed::from_i64(5).unwrap());
        verify(Fixed::from_f32(5.0).unwrap());
        verify(Fixed::from_f64(5.0).unwrap());
        verify(Fixed::from(5i32));
        verify(Fixed::from(5i64));
        verify(Fixed::from(5.0f32));
        verify(Fixed::from(5.0f64));
        verify(5i32.into());
        verify(5i64.into());
        verify(5.0f32.into());
        verify(5.0f64.into());

        let verify = |f: Fixed| assert_eq!(-5 << SCALE_BITS, f.scaled);
        verify(Fixed::from_i32(-5).unwrap());
        verify(Fixed::from_i64(-5).unwrap());
        verify(Fixed::from_f32(-5.0).unwrap());
        verify(Fixed::from_f64(-5.0).unwrap());

        let verify = |f: Fixed| assert_eq!(90112, f.scaled);
        verify(Fixed::from_f32(5.5).unwrap());
        verify(Fixed::from_f64(5.5).unwrap());
    }

    #[test]
    fn test_fmt() {
        assert_eq!("5".to_string(), format!("{}", Fixed::from_i32(5).unwrap()));
        assert_eq!("5.5".to_string(),
                   format!("{}", Fixed::from_f64(5.5).unwrap()));
        assert_eq!("-5".to_string(),
                   format!("{}", Fixed::from_i32(-5).unwrap()));
        assert_eq!("-5.5".to_string(),
                   format!("{}", Fixed::from_f64(-5.5).unwrap()));

        assert_eq!("Fixed(5: 81920/16384)".to_string(),
                   format!("{:?}", Fixed::from_i32(5).unwrap()));
        assert_eq!("Fixed(5.5: 90112/16384)".to_string(),
                   format!("{:?}", Fixed::from_f64(5.5).unwrap()));
        assert_eq!("Fixed(-5: -81920/16384)".to_string(),
                   format!("{:?}", Fixed::from_i32(-5).unwrap()));
        assert_eq!("Fixed(-5.5: -90112/16384)".to_string(),
                   format!("{:?}", Fixed::from_f64(-5.5).unwrap()));
    }

    #[test]
    fn test_add() {
        assert_eq!(Fixed::new(20000), Fixed::new(5000) + Fixed::new(15000));
        assert_eq!(Fixed::new(5000), Fixed::new(-5000) + Fixed::new(10000));

        let mut a = Fixed::new(1000);
        a += Fixed::new(500);
        assert_eq!(Fixed::new(1500), a);
    }

    #[test]
    fn test_sub() {
        assert_eq!(Fixed::new(-10000), Fixed::new(5000) - Fixed::new(15000));
        assert_eq!(Fixed::new(-20000), Fixed::new(-5000) - Fixed::new(15000));

        let mut a = Fixed::new(1000);
        a -= Fixed::new(500);
        assert_eq!(Fixed::new(500), a);
    }

    #[test]
    fn test_mul() {
        assert_eq!(50.to_fixed(), 5.to_fixed() * 10.to_fixed());
        assert_eq!((-50).to_fixed(), 5.to_fixed() * (-10).to_fixed());
        assert_eq!(55.to_fixed(), 5.5.to_fixed() * 10.to_fixed());
        assert_eq!(2.5.to_fixed(), 5.to_fixed() * 0.5.to_fixed());

        let mut a = Fixed::from(5);
        a *= Fixed::from(2);
        assert_eq!(Fixed::from(10), a);
    }

    #[test]
    fn test_div() {
        assert_eq!(2.to_fixed(), 10.to_fixed() / 5.to_fixed());
        assert_eq!(4.to_fixed(), 1.to_fixed() / 0.25.to_fixed());
        assert_eq!((-2).to_fixed(), 10.to_fixed() / (-5).to_fixed());
        assert_eq!(0.5.to_fixed(), 1.to_fixed() / 2.to_fixed());
        assert_eq!(2.5.to_fixed(), 5.to_fixed() / 2.to_fixed());

        let mut a = Fixed::from(10);
        a /= Fixed::from(2);
        assert_eq!(Fixed::from(5), a);
    }

    #[test]
    fn test_rem() {
        assert_eq!(2.to_fixed(), 10.to_fixed() % 8.to_fixed());
        assert_eq!(0.to_fixed(), 10.to_fixed() % 10.to_fixed());
        assert_eq!(9.to_fixed(), 9.to_fixed() % 10.to_fixed());
    }

    #[test]
    fn test_zero() {
        assert_eq!(0, Fixed::zero().scaled);
        assert!(Fixed::is_zero(&Fixed::zero()));
    }

    #[test]
    fn test_one() {
        assert_eq!(1, Fixed::one().to_i32().unwrap());
    }

    #[test]
    fn test_num() {
        assert_eq!(Fixed::from_f64(4.5).unwrap(),
                   Fixed::from_str_radix("4.5", 10).unwrap());
    }

    #[test]
    fn test_bounded() {
        assert_eq!(Fixed::new(i32::min_value()), Fixed::min_value());
        assert_eq!(Fixed::new(i32::max_value()), Fixed::max_value());
    }

    #[test]
    fn test_neg() {
        assert_eq!(Fixed::from(-5), -Fixed::from(5));
        assert_eq!(Fixed::from(5), -Fixed::from(-5));
    }

    #[test]
    fn test_compare() {
        assert!(Fixed::from(5) > Fixed::from(4));
        assert!(Fixed::from(4) < Fixed::from(5));
        assert!(Fixed::from(4) <= Fixed::from(4));
        assert!(Fixed::from(4) >= Fixed::from(4));
        assert!(Fixed::from(4) == Fixed::from(4));
        assert!(Fixed::from(5) != Fixed::from(4));
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(Fixed::from(1), Fixed::from(1).sqrt());
        assert_eq!(Fixed::from(2), Fixed::from(4).sqrt());
        assert_eq!(Fixed::from(3), Fixed::from(9).sqrt());
        assert_eq!(Fixed::from(11), Fixed::from(121).sqrt());
        assert_eq!(Fixed::from(100), Fixed::from(10000).sqrt());
        assert_eq!(Fixed::from(0.5), Fixed::from(0.25).sqrt());
    }

    #[test]
    #[should_panic]
    fn test_sqrt_neg() {
        Fixed::from(-5).sqrt();
    }

    // Commented out benchmarks since they don't compile on stable rustc
    // Wrapped in a function so that rustfmt doesn't touch the comment formatting
    //use test::{self, Bencher};
    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[test]
    fn test_bench_wrapper() {
        /*
        macro_rules! ops_benchmark {
            ($name:ident, $typ:ident) => {
                #[bench]
                fn $name(bench: &mut Bencher) {
                    let a = $typ::from_i32(5).unwrap();
                    let b = $typ::from_i32(10).unwrap();
                    let c = $typ::from_i32(-1).unwrap();
                    let d = $typ::from_i32(121).unwrap();
                    let e = $typ::from_i32(9).unwrap();

                    let do_op = |a, b, c, d: $typ, e: $typ| a * b + c - d.sqrt() + e / c;
                    bench.iter(|| {
                        test::black_box(do_op(test::black_box(a),
                                              test::black_box(b),
                                              test::black_box(c),
                                              test::black_box(d),
                                              test::black_box(e)));
                    });
                }
            }
        }

        ops_benchmark!(bench_fixed_ops, Fixed);
        ops_benchmark!(bench_f32_ops, f32);
        ops_benchmark!(bench_f64_ops, f64);
        */
    }
}
