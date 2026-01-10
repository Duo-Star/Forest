// src/math_forest/algebra/complex/q_complex.rs
#![allow(dead_code)]

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use super::complex::Complex;
use super::d_complex::DComplex;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct QComplex {
    pub n1: Complex,
    pub n2: Complex,
    pub n3: Complex,
    pub n4: Complex,
}

impl QComplex {
    #[inline(always)]
    pub fn new(n1: Complex, n2: Complex, n3: Complex, n4: Complex) -> Self {
        QComplex { n1, n2, n3, n4 }
    }

    // 复数域的调和分割
    #[inline]
    pub fn harmonic(dn: DComplex, t: f64) -> Self {
        let a = dn.n1;
        let b = dn.n2;
        // 注意：复数混合运算已通过 Complex 的 Trait 支持
        let m = (b * t - a) / (t - 1.0);
        let n = (b * t + a) / (t + 1.0);
        QComplex::new(a, m, b, n)
    }
}

// ====================== 运算符 (Struct op Struct 补全) ======================
impl Add for QComplex {
    type Output = QComplex;
    fn add(self, rhs: Self) -> Self { QComplex::new(self.n1+rhs.n1, self.n2+rhs.n2, self.n3+rhs.n3, self.n4+rhs.n4) }
}
impl Mul for QComplex {
    type Output = QComplex;
    fn mul(self, rhs: Self) -> Self { QComplex::new(self.n1*rhs.n1, self.n2*rhs.n2, self.n3*rhs.n3, self.n4*rhs.n4) }
}

// ====================== 运算符 (Scalar) ======================

impl Add<f64> for QComplex {
    type Output = Self;
    #[inline] fn add(self, rhs: f64) -> Self { QComplex { n1: self.n1 + rhs, n2: self.n2 + rhs, n3: self.n3 + rhs, n4: self.n4 + rhs } }
}
impl Add<QComplex> for f64 {
    type Output = QComplex;
    #[inline] fn add(self, rhs: QComplex) -> QComplex { rhs + self }
}
impl Sub<f64> for QComplex {
    type Output = Self;
    #[inline] fn sub(self, rhs: f64) -> Self { QComplex { n1: self.n1 - rhs, n2: self.n2 - rhs, n3: self.n3 - rhs, n4: self.n4 - rhs } }
}
impl Mul<f64> for QComplex {
    type Output = Self;
    #[inline] fn mul(self, rhs: f64) -> Self { QComplex { n1: self.n1 * rhs, n2: self.n2 * rhs, n3: self.n3 * rhs, n4: self.n4 * rhs } }
}
impl Div<f64> for QComplex {
    type Output = Self;
    #[inline] fn div(self, rhs: f64) -> Self { QComplex { n1: self.n1 / rhs, n2: self.n2 / rhs, n3: self.n3 / rhs, n4: self.n4 / rhs } }
}
impl Neg for QComplex {
    type Output = Self;
    #[inline] fn neg(self) -> Self { QComplex { n1: -self.n1, n2: -self.n2, n3: -self.n3, n4: -self.n4 } }
}


// 追加到 q_complex.rs 末尾
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

// +=
impl AddAssign for QComplex {
    #[inline] fn add_assign(&mut self, rhs: Self) {
        self.n1 += rhs.n1; self.n2 += rhs.n2; self.n3 += rhs.n3; self.n4 += rhs.n4;
    }
}
impl AddAssign<f64> for QComplex {
    #[inline] fn add_assign(&mut self, rhs: f64) {
        self.n1 += rhs; self.n2 += rhs; self.n3 += rhs; self.n4 += rhs;
    }
}

// -=
impl SubAssign for QComplex {
    #[inline] fn sub_assign(&mut self, rhs: Self) {
        self.n1 -= rhs.n1; self.n2 -= rhs.n2; self.n3 -= rhs.n3; self.n4 -= rhs.n4;
    }
}
impl SubAssign<f64> for QComplex {
    #[inline] fn sub_assign(&mut self, rhs: f64) {
        self.n1 -= rhs; self.n2 -= rhs; self.n3 -= rhs; self.n4 -= rhs;
    }
}

// *=
impl MulAssign for QComplex {
    #[inline] fn mul_assign(&mut self, rhs: Self) {
        self.n1 *= rhs.n1; self.n2 *= rhs.n2; self.n3 *= rhs.n3; self.n4 *= rhs.n4;
    }
}
impl MulAssign<f64> for QComplex {
    #[inline] fn mul_assign(&mut self, rhs: f64) {
        self.n1 *= rhs; self.n2 *= rhs; self.n3 *= rhs; self.n4 *= rhs;
    }
}

// /=
impl DivAssign for QComplex {
    #[inline] fn div_assign(&mut self, rhs: Self) {
        self.n1 /= rhs.n1; self.n2 /= rhs.n2; self.n3 /= rhs.n3; self.n4 /= rhs.n4;
    }
}
impl DivAssign<f64> for QComplex {
    #[inline] fn div_assign(&mut self, rhs: f64) {
        self.n1 /= rhs; self.n2 /= rhs; self.n3 /= rhs; self.n4 /= rhs;
    }
}

impl fmt::Display for QComplex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "QComplex({}, {}, {}, {})", self.n1, self.n2, self.n3, self.n4)
    }
}