// src/math_forest/algebra/complex/d_complex.rs
#![allow(dead_code)]

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};
use super::complex::Complex;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DComplex {
    pub n1: Complex,
    pub n2: Complex,
}

impl DComplex {
    #[inline(always)]
    pub fn new(n1: Complex, n2: Complex) -> Self {
        DComplex { n1, n2 }
    }

    // 方便构造实数对
    pub fn from_real(r1: f64, r2: f64) -> Self {
        Self { n1: Complex::from_real(r1), n2: Complex::from_real(r2) }
    }

    #[inline]
    pub fn min(self) -> Complex { self.n1.min(self.n2) }

    #[inline]
    pub fn max(self) -> Complex { self.n1.max(self.n2) }
}

// ====================== 运算符 (Struct op Struct 补全) ======================

impl Add for DComplex {
    type Output = DComplex;
    #[inline] fn add(self, rhs: Self) -> Self::Output { DComplex::new(self.n1 + rhs.n1, self.n2 + rhs.n2) }
}
impl Sub for DComplex {
    type Output = DComplex;
    #[inline] fn sub(self, rhs: Self) -> Self::Output { DComplex::new(self.n1 - rhs.n1, self.n2 - rhs.n2) }
}
impl Mul for DComplex {
    type Output = DComplex;
    #[inline] fn mul(self, rhs: Self) -> Self::Output { DComplex::new(self.n1 * rhs.n1, self.n2 * rhs.n2) }
}

// ====================== 运算符 (Struct op Scalar) ======================

impl Add<f64> for DComplex {
    type Output = DComplex;
    #[inline] fn add(self, rhs: f64) -> Self { DComplex { n1: self.n1 + rhs, n2: self.n2 + rhs } }
}
impl Add<DComplex> for f64 {
    type Output = DComplex;
    #[inline] fn add(self, rhs: DComplex) -> DComplex { rhs + self }
}
impl Sub<f64> for DComplex {
    type Output = DComplex;
    #[inline] fn sub(self, rhs: f64) -> Self { DComplex { n1: self.n1 - rhs, n2: self.n2 - rhs } }
}
impl Sub<DComplex> for f64 {
    type Output = DComplex;
    #[inline] fn sub(self, rhs: DComplex) -> DComplex { -(rhs - self) }
}
impl Mul<f64> for DComplex {
    type Output = DComplex;
    #[inline] fn mul(self, rhs: f64) -> Self { DComplex { n1: self.n1 * rhs, n2: self.n2 * rhs } }
}
impl Mul<DComplex> for f64 {
    type Output = DComplex;
    #[inline] fn mul(self, rhs: DComplex) -> DComplex { rhs * self }
}
impl Div<f64> for DComplex {
    type Output = DComplex;
    #[inline] fn div(self, rhs: f64) -> Self { DComplex { n1: self.n1 / rhs, n2: self.n2 / rhs } }
}
impl Neg for DComplex {
    type Output = DComplex;
    #[inline] fn neg(self) -> Self { DComplex { n1: -self.n1, n2: -self.n2 } }
}


// 追加到 d_complex.rs 末尾
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

// +=
impl AddAssign for DComplex {
    #[inline] fn add_assign(&mut self, rhs: Self) { self.n1 += rhs.n1; self.n2 += rhs.n2; }
}
impl AddAssign<f64> for DComplex {
    #[inline] fn add_assign(&mut self, rhs: f64) { self.n1 += rhs; self.n2 += rhs; }
}

// -=
impl SubAssign for DComplex {
    #[inline] fn sub_assign(&mut self, rhs: Self) { self.n1 -= rhs.n1; self.n2 -= rhs.n2; }
}
impl SubAssign<f64> for DComplex {
    #[inline] fn sub_assign(&mut self, rhs: f64) { self.n1 -= rhs; self.n2 -= rhs; }
}

// *=
impl MulAssign for DComplex {
    #[inline] fn mul_assign(&mut self, rhs: Self) { self.n1 *= rhs.n1; self.n2 *= rhs.n2; }
}
impl MulAssign<f64> for DComplex {
    #[inline] fn mul_assign(&mut self, rhs: f64) { self.n1 *= rhs; self.n2 *= rhs; }
}

// /=
impl DivAssign for DComplex {
    #[inline] fn div_assign(&mut self, rhs: Self) { self.n1 /= rhs.n1; self.n2 /= rhs.n2; }
}
impl DivAssign<f64> for DComplex {
    #[inline] fn div_assign(&mut self, rhs: f64) { self.n1 /= rhs; self.n2 /= rhs; }
}


impl fmt::Display for DComplex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DComplex({}, {})", self.n1, self.n2) // 修正了 DNum -> DComplex
    }
}