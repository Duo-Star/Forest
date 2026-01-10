#![allow(dead_code)]

// 骈数
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::math_forest::geometry::d2::linear::vec2::Vec2;

//
#[derive(Clone, Copy, PartialEq, Debug)]

//
pub struct DNum {
    pub n1: f64,
    pub n2: f64,
}

impl DNum {
    #[inline]
    pub fn new(n1: f64, n2: f64) -> Self {
        DNum { n1, n2 }
    }

    // 返回较小的值
    #[inline]
    pub fn min(self) -> f64 {
        self.n1.min(self.n2)
    }

    // 返回较大的值
    #[inline]
    pub fn max(self) -> f64 {
        self.n1.max(self.n2)
    }

    pub const INF: DNum = DNum { n1: f64::INFINITY, n2: f64::INFINITY };
    pub const NAN: DNum = DNum { n1: f64::NAN, n2: f64::NAN };
}

// ====================== 运算符 ======================

// DNum + f64
impl Add<f64> for DNum {
    type Output = DNum;
    #[inline]
    fn add(self, rhs: f64) -> Self {
        DNum {
            n1: self.n1 + rhs,
            n2: self.n2 + rhs,
        }
    }
}

// f64 + DNum
impl Add<DNum> for f64 {
    type Output = DNum;
    #[inline]
    fn add(self, rhs: DNum) -> DNum {
        rhs + self
    }
}

// - DNum
impl Neg for DNum {
    type Output = DNum;
    #[inline]
    fn neg(self) -> Self {
        DNum {
            n1: -self.n1,
            n2: -self.n2,
        }
    }
}

// DNum - f64
impl Sub<f64> for DNum {
    type Output = DNum;
    #[inline]
    fn sub(self, rhs: f64) -> Self {
        DNum {
            n1: self.n1 - rhs,
            n2: self.n2 - rhs,
        }
    }
}

// f64 - DNum
impl Sub<DNum> for f64 {
    type Output = DNum;
    #[inline]
    fn sub(self, rhs: DNum) -> DNum {
        -(rhs - self)
    }
}

// DNum * f64
impl Mul<f64> for DNum {
    type Output = DNum;

    #[inline]
    fn mul(self, rhs: f64) -> Self {
        DNum {
            n1: self.n1 * rhs,
            n2: self.n2 * rhs,
        }
    }
}

// f64 * DNum
impl Mul<DNum> for f64 {
    type Output = DNum;

    #[inline]
    fn mul(self, rhs: DNum) -> DNum {
        rhs * self // 调用 DNum * f64
    }
}

// DNum / f64
impl Div<f64> for DNum {
    type Output = DNum;

    #[inline]
    fn div(self, rhs: f64) -> Self {
        DNum {
            n1: self.n1 / rhs,
            n2: self.n2 / rhs,
        }
    }
}

// f64 / DNum
impl Div<DNum> for f64 {
    type Output = DNum;

    #[inline]
    fn div(self, rhs: DNum) -> DNum {
        DNum {
            n1: self / rhs.n1,
            n2: self / rhs.n2,
        }
    }
}

// ====================== 格式输出 ======================

impl fmt::Display for DNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DNum({}, {})", self.n1, self.n2)
    }
}

// ====================== 测试示例 ======================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operations() {
        let a = DNum::new(2.0, 3.0);
        let b = DNum::new(1.0, 4.0);
        println!("a = {:?}\nb = {:?}", a, b);
    }

    #[test]
    fn test_display() {
        let d = DNum::new(1.5, -2.7);
        assert_eq!(format!("{}", d), "DNum(1.5, -2.7)");
    }
}
