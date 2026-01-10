#![allow(dead_code)]

use super::d_num::DNum;

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct QNum {
    pub n1: f64,
    pub n2: f64,
    pub n3: f64,
    pub n4: f64,
}

impl QNum {
    #[inline]
    pub fn new(n1: f64, n2: f64, n3: f64, n4: f64) -> Self {
        QNum { n1, n2, n3, n4 }
    }

    #[inline]
    pub fn harmonic(dn: DNum, t: f64) -> Self {
        let a = dn.n1;
        let b = dn.n2;
        let m = (t * b - a) / (t - 1.0);
        let n = (t * b + a) / (t + 1.0);
        QNum::new(a, m, b, n)
    }
}

// ====================== 运算符重载 ======================

// QNum + f64
impl Add<f64> for QNum {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f64) -> Self {
        QNum {
            n1: self.n1 + rhs,
            n2: self.n2 + rhs,
            n3: self.n3 + rhs,
            n4: self.n4 + rhs,
        }
    }
}

// f64 + QNum（加法交换）
impl Add<QNum> for f64 {
    type Output = QNum;

    #[inline]
    fn add(self, rhs: QNum) -> QNum {
        rhs + self
    }
}

// -QNum
impl Neg for QNum {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        QNum {
            n1: -self.n1,
            n2: -self.n2,
            n3: -self.n3,
            n4: -self.n4,
        }
    }
}

// QNum - f64
impl Sub<f64> for QNum {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f64) -> Self {
        QNum {
            n1: self.n1 - rhs,
            n2: self.n2 - rhs,
            n3: self.n3 - rhs,
            n4: self.n4 - rhs,
        }
    }
}

// f64 - QNum → -(QNum - f64)
impl Sub<QNum> for f64 {
    type Output = QNum;

    #[inline]
    fn sub(self, rhs: QNum) -> QNum {
        -(rhs - self)
    }
}

// QNum * f64
impl Mul<f64> for QNum {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self {
        QNum {
            n1: self.n1 * rhs,
            n2: self.n2 * rhs,
            n3: self.n3 * rhs,
            n4: self.n4 * rhs,
        }
    }
}

// f64 * QNum（乘法交换）
impl Mul<QNum> for f64 {
    type Output = QNum;

    #[inline]
    fn mul(self, rhs: QNum) -> QNum {
        rhs * self
    }
}

// QNum / f64
impl Div<f64> for QNum {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self {
        QNum {
            n1: self.n1 / rhs,
            n2: self.n2 / rhs,
            n3: self.n3 / rhs,
            n4: self.n4 / rhs,
        }
    }
}

// f64 / QNum（标量被每个分量除）
impl Div<QNum> for f64 {
    type Output = QNum;

    #[inline]
    fn div(self, rhs: QNum) -> QNum {
        QNum {
            n1: self / rhs.n1,
            n2: self / rhs.n2,
            n3: self / rhs.n3,
            n4: self / rhs.n4,
        }
    }
}

// ====================== 格式输出 ======================

impl fmt::Display for QNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "QNum({}, {}, {}, {})",
            self.n1, self.n2, self.n3, self.n4
        )
    }
}

// ====================== 测试示例 ======================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_operations() {
        let q = QNum::new(1.0, 2.0, 3.0, 4.0);
        let s = 5.0;

        assert_eq!(q + s, QNum::new(6.0, 7.0, 8.0, 9.0));
        assert_eq!(s + q, QNum::new(6.0, 7.0, 8.0, 9.0));

        assert_eq!(q - s, QNum::new(-4.0, -3.0, -2.0, -1.0));
        assert_eq!(s - q, QNum::new(4.0, 3.0, 2.0, 1.0));

        assert_eq!(q * s, QNum::new(5.0, 10.0, 15.0, 20.0));
        assert_eq!(s * q, QNum::new(5.0, 10.0, 15.0, 20.0));

        assert_eq!(q / s, QNum::new(0.2, 0.4, 0.6, 0.8));
        assert_eq!(s / q, QNum::new(5.0, 2.5, 5.0 / 3.0, 1.25));

        assert_eq!(-q, QNum::new(-1.0, -2.0, -3.0, -4.0));
    }

    #[test]
    fn test_display() {
        let q = QNum::new(1.5, -2.0, 3.14, 0.0);
        assert_eq!(format!("{}", q), "QNum(1.5, -2, 3.14, 0)");
    }

    #[test]
    fn test_chain_operations() {
        let q = QNum::new(10.0, 20.0, 30.0, 40.0);
        let result = (q * 2.0 + 3.0) / 4.0 - 1.0;
        assert_eq!(
            result,
            QNum::new(
                (20.0 + 3.0) / 4.0 - 1.0,
                (40.0 + 3.0) / 4.0 - 1.0,
                (60.0 + 3.0) / 4.0 - 1.0,
                (80.0 + 3.0) / 4.0 - 1.0
            )
        );
    }
}
