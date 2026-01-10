#![allow(dead_code)]
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TNum {
    pub n1: f64,
    pub n2: f64,
    pub n3: f64,
}

impl TNum {
    pub fn new(n1: f64, n2: f64, n3: f64) -> Self {
        TNum { n1, n2, n3 }
    }

    pub fn all(n: f64) -> Self {
        TNum { n1:n, n2:n, n3:n }
    }
}

// ====================== 运算符重载 ======================

// TNum + f64
impl Add<f64> for TNum {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f64) -> Self {
        TNum {
            n1: self.n1 + rhs,
            n2: self.n2 + rhs,
            n3: self.n3 + rhs,
        }
    }
}

// f64 + TNum（加法交换）
impl Add<TNum> for f64 {
    type Output = TNum;

    #[inline]
    fn add(self, rhs: TNum) -> TNum {
        rhs + self
    }
}

// -TNum
impl Neg for TNum {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        TNum {
            n1: -self.n1,
            n2: -self.n2,
            n3: -self.n3,
        }
    }
}

// TNum - f64
impl Sub<f64> for TNum {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f64) -> Self {
        TNum {
            n1: self.n1 - rhs,
            n2: self.n2 - rhs,
            n3: self.n3 - rhs,
        }
    }
}

// f64 - TNum → -(TNum - f64)
impl Sub<TNum> for f64 {
    type Output = TNum;

    #[inline]
    fn sub(self, rhs: TNum) -> TNum {
        -(rhs - self)
    }
}

// TNum * f64
impl Mul<f64> for TNum {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self {
        TNum {
            n1: self.n1 * rhs,
            n2: self.n2 * rhs,
            n3: self.n3 * rhs,
        }
    }
}

// f64 * TNum（乘法交换）
impl Mul<TNum> for f64 {
    type Output = TNum;

    #[inline]
    fn mul(self, rhs: TNum) -> TNum {
        rhs * self
    }
}

// TNum / f64
impl Div<f64> for TNum {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self {
        TNum {
            n1: self.n1 / rhs,
            n2: self.n2 / rhs,
            n3: self.n3 / rhs,
        }
    }
}

// f64 / TNum（标量被每个分量除）
impl Div<TNum> for f64 {
    type Output = TNum;

    #[inline]
    fn div(self, rhs: TNum) -> TNum {
        TNum {
            n1: self / rhs.n1,
            n2: self / rhs.n2,
            n3: self / rhs.n3,
        }
    }
}

// ====================== 格式输出 ======================

impl fmt::Display for TNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TNum({}, {}, {})", self.n1, self.n2, self.n3)
    }
}

// ====================== 测试示例 ======================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_operations() {
        let t = TNum::new(1.0, 2.0, 3.0);
        let s = 4.0;

        assert_eq!(t + s, TNum::new(5.0, 6.0, 7.0));
        assert_eq!(s + t, TNum::new(5.0, 6.0, 7.0));

        assert_eq!(t - s, TNum::new(-3.0, -2.0, -1.0));
        assert_eq!(s - t, TNum::new(3.0, 2.0, 1.0));

        assert_eq!(t * s, TNum::new(4.0, 8.0, 12.0));
        assert_eq!(s * t, TNum::new(4.0, 8.0, 12.0));

        assert_eq!(t / s, TNum::new(0.25, 0.5, 0.75));
        assert_eq!(s / t, TNum::new(4.0, 2.0, 4.0 / 3.0));

        assert_eq!(-t, TNum::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_display() {
        let t = TNum::new(1.5, -2.0, 3.14);
        assert_eq!(format!("{}", t), "TNum(1.5, -2, 3.14)");
    }

    #[test]
    fn test_debug() {
        let t = TNum::new(1.0, 2.0, 3.0);
        println!("Debug: {:?}", t);
        println!("Display: {}", t);
    }
}
