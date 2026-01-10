// src/math_forest/algebra/complex/t_complex.rs
#![allow(dead_code)]

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use super::complex::Complex;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TComplex {
    pub n1: Complex,
    pub n2: Complex,
    pub n3: Complex,
}

impl TComplex {
    pub const ZERO: TComplex = TComplex::new(Complex::ZERO, Complex::ZERO, Complex::ZERO);
    pub const NAN: TComplex = TComplex::new(Complex::NAN, Complex::NAN, Complex::NAN);

    #[inline(always)]
    pub const fn new(n1: Complex, n2: Complex, n3: Complex) -> Self {
        TComplex { n1, n2, n3 }
    }

    pub const fn all(n: Complex) -> Self {
        TComplex { n1: n, n2: n, n3: n }
    }
}

// ====================== 运算符 ======================
// Struct op Struct
impl Add for TComplex {
    type Output = TComplex;
    fn add(self, rhs: Self) -> Self { TComplex::new(self.n1+rhs.n1, self.n2+rhs.n2, self.n3+rhs.n3) }
}

// Scalar
impl Add<f64> for TComplex {
    type Output = Self;
    #[inline] fn add(self, rhs: f64) -> Self { TComplex { n1: self.n1 + rhs, n2: self.n2 + rhs, n3: self.n3 + rhs } }
}
impl Add<TComplex> for f64 {
    type Output = TComplex;
    #[inline] fn add(self, rhs: TComplex) -> TComplex { rhs + self }
}
impl Sub<f64> for TComplex {
    type Output = Self;
    #[inline] fn sub(self, rhs: f64) -> Self { TComplex { n1: self.n1 - rhs, n2: self.n2 - rhs, n3: self.n3 - rhs } }
}
impl Mul<f64> for TComplex {
    type Output = Self;
    #[inline] fn mul(self, rhs: f64) -> Self { TComplex { n1: self.n1 * rhs, n2: self.n2 * rhs, n3: self.n3 * rhs } }
}
impl Div<f64> for TComplex {
    type Output = Self;
    #[inline] fn div(self, rhs: f64) -> Self { TComplex { n1: self.n1 / rhs, n2: self.n2 / rhs, n3: self.n3 / rhs } }
}
impl Neg for TComplex {
    type Output = Self;
    #[inline] fn neg(self) -> Self { TComplex { n1: -self.n1, n2: -self.n2, n3: -self.n3 } }
}

// 追加到 t_complex.rs 末尾
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

// +=
impl AddAssign for TComplex {
    #[inline] fn add_assign(&mut self, rhs: Self) {
        self.n1 += rhs.n1; self.n2 += rhs.n2; self.n3 += rhs.n3;
    }
}
impl AddAssign<f64> for TComplex {
    #[inline] fn add_assign(&mut self, rhs: f64) {
        self.n1 += rhs; self.n2 += rhs; self.n3 += rhs;
    }
}

// -=
impl SubAssign for TComplex {
    #[inline] fn sub_assign(&mut self, rhs: Self) {
        self.n1 -= rhs.n1; self.n2 -= rhs.n2; self.n3 -= rhs.n3;
    }
}
impl SubAssign<f64> for TComplex {
    #[inline] fn sub_assign(&mut self, rhs: f64) {
        self.n1 -= rhs; self.n2 -= rhs; self.n3 -= rhs;
    }
}

// *=
impl MulAssign for TComplex {
    #[inline] fn mul_assign(&mut self, rhs: Self) {
        self.n1 *= rhs.n1; self.n2 *= rhs.n2; self.n3 *= rhs.n3;
    }
}
impl MulAssign<f64> for TComplex {
    #[inline] fn mul_assign(&mut self, rhs: f64) {
        self.n1 *= rhs; self.n2 *= rhs; self.n3 *= rhs;
    }
}

// /=
impl DivAssign for TComplex {
    #[inline] fn div_assign(&mut self, rhs: Self) {
        self.n1 /= rhs.n1; self.n2 /= rhs.n2; self.n3 /= rhs.n3;
    }
}
impl DivAssign<f64> for TComplex {
    #[inline] fn div_assign(&mut self, rhs: f64) {
        self.n1 /= rhs; self.n2 /= rhs; self.n3 /= rhs;
    }
}

impl fmt::Display for TComplex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TComplex({}, {}, {})", self.n1, self.n2, self.n3)
    }
}