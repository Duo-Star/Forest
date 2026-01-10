// src/math_forest/algebra/linear/matrix3x3.rs
#![allow(dead_code)]

use std::fmt;
use std::ops::{Add, Sub, Mul, Neg, AddAssign, SubAssign, MulAssign};
use crate::math_forest::algebra::solver::linear::solve_linear_3x3;
use crate::math_forest::geometry::d2::linear::vec2::Vec2;

/// 3x3 矩阵，按行优先存储 (Row-Major)
/// [ m00, m01, m02 ]
/// [ m10, m11, m12 ]
/// [ m20, m21, m22 ]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3x3 {
    pub m: [f64; 9],
}

impl Matrix3x3 {
    pub const IDENTITY: Matrix3x3 = Matrix3x3 {
        m: [1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0]
    };

    pub const ZERO: Matrix3x3 = Matrix3x3 { m: [0.0; 9] };

    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        m00: f64, m01: f64, m02: f64,
        m10: f64, m11: f64, m12: f64,
        m20: f64, m21: f64, m22: f64,
    ) -> Self {
        Self {
            m: [m00, m01, m02, m10, m11, m12, m20, m21, m22],
        }
    }

    // ====================== 2D 仿射变换构造 (Affine Transforms) ======================

    /// 构造 2D 平移矩阵
    /// [ 1  0  tx ]
    /// [ 0  1  ty ]
    /// [ 0  0  1  ]
    pub fn from_translation(tx: f64, ty: f64) -> Self {
        Self::new(
            1.0, 0.0, tx,
            0.0, 1.0, ty,
            0.0, 0.0, 1.0
        )
    }

    /// 构造 2D 旋转矩阵 (绕原点)
    /// [ c -s  0 ]
    /// [ s  c  0 ]
    /// [ 0  0  1 ]
    pub fn from_rotation(theta: f64) -> Self {
        let (sin, cos) = theta.sin_cos();
        Self::new(
            cos, -sin, 0.0,
            sin,  cos, 0.0,
            0.0,  0.0, 1.0
        )
    }

    /// 构造 2D 缩放矩阵
    /// [ sx 0  0 ]
    /// [ 0  sy 0 ]
    /// [ 0  0  1 ]
    pub fn from_scaling(sx: f64, sy: f64) -> Self {
        Self::new(
            sx, 0.0, 0.0,
            0.0, sy, 0.0,
            0.0, 0.0, 1.0
        )
    }

    /// 构造 2D 变换 (平移 + 旋转 + 缩放)
    pub fn from_transform(pos: Vec2, rotation: f64, scale: Vec2) -> Self {
        // T * R * S
        let (sin, cos) = rotation.sin_cos();
        let sx = scale.x;
        let sy = scale.y;

        Self::new(
            cos * sx, -sin * sy, pos.x,
            sin * sx,  cos * sy, pos.y,
            0.0,       0.0,      1.0
        )
    }

    // ====================== 核心数学运算 ======================

    #[inline]
    pub fn det(&self) -> f64 {
        let [a, b, c, d, e, f, g, h, i] = self.m;
        a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    }

    #[inline]
    pub fn trace(&self) -> f64 {
        self.m[0] + self.m[4] + self.m[8]
    }

    #[inline]
    pub fn transpose(&self) -> Self {
        let m = self.m;
        Self::new(
            m[0], m[3], m[6],
            m[1], m[4], m[7],
            m[2], m[5], m[8]
        )
    }

    /// 求逆矩阵
    pub fn inverse(&self) -> Option<Self> {
        let det = self.det();
        if det.abs() < 1e-12 {
            return None;
        }
        let inv_det = 1.0 / det;
        let m = self.m;

        // 计算代数余子式并转置 (Adjugate Matrix)
        Some(Self::new(
            (m[4] * m[8] - m[5] * m[7]) * inv_det, (m[2] * m[7] - m[1] * m[8]) * inv_det, (m[1] * m[5] - m[2] * m[4]) * inv_det,
            (m[5] * m[6] - m[3] * m[8]) * inv_det, (m[0] * m[8] - m[2] * m[6]) * inv_det, (m[2] * m[3] - m[0] * m[5]) * inv_det,
            (m[3] * m[7] - m[4] * m[6]) * inv_det, (m[1] * m[6] - m[0] * m[7]) * inv_det, (m[0] * m[4] - m[1] * m[3]) * inv_det
        ))
    }

    // ====================== 应用于 Vec2 (2D 变换) ======================

    /// 变换 2D 点 (视为 (x, y, 1))
    /// 结果: (x', y') = (m00*x + m01*y + m02, m10*x + m11*y + m12)
    /// 这里的除以 z (投影除法) 通常在 2D 仿射中省略，因为最后一行通常是 0 0 1
    pub fn transform_point2(&self, p: Vec2) -> Vec2 {
        let x = self.m[0] * p.x + self.m[1] * p.y + self.m[2];
        let y = self.m[3] * p.x + self.m[4] * p.y + self.m[5];
        // 如果是投影变换，可能需要除以 w:
        // let w = self.m[6] * p.x + self.m[7] * p.y + self.m[8];
        Vec2::new(x, y)
    }

    /// 变换 2D 向量 (视为 (x, y, 0)) - 忽略平移
    /// 结果: (x', y') = (m00*x + m01*y, m10*x + m11*y)
    pub fn transform_vector2(&self, v: Vec2) -> Vec2 {
        let x = self.m[0] * v.x + self.m[1] * v.y;
        let y = self.m[3] * v.x + self.m[4] * v.y;
        Vec2::new(x, y)
    }

    // ====================== 求解器 ======================

    pub fn solve(&self, d1: f64, d2: f64, d3: f64) -> (f64, f64, f64) {
        let m = self.m;
        solve_linear_3x3(
            m[0], m[1], m[2], d1,
            m[3], m[4], m[5], d2,
            m[6], m[7], m[8], d3,
        )
    }
}

// ====================== 运算符重载 ======================

// Matrix + Matrix
impl Add for Matrix3x3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut m = [0.0; 9];
        for i in 0..9 { m[i] = self.m[i] + rhs.m[i]; }
        Self { m }
    }
}

// Matrix - Matrix
impl Sub for Matrix3x3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut m = [0.0; 9];
        for i in 0..9 { m[i] = self.m[i] - rhs.m[i]; }
        Self { m }
    }
}

// Matrix * f64
impl Mul<f64> for Matrix3x3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        let mut m = [0.0; 9];
        for i in 0..9 { m[i] = self.m[i] * rhs; }
        Self { m }
    }
}

// f64 * Matrix
impl Mul<Matrix3x3> for f64 {
    type Output = Matrix3x3;
    fn mul(self, rhs: Matrix3x3) -> Matrix3x3 { rhs * self }
}

// Matrix * Matrix
impl Mul for Matrix3x3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let l = self.m;
        let r = rhs.m;
        Self::new(
            l[0]*r[0]+l[1]*r[3]+l[2]*r[6], l[0]*r[1]+l[1]*r[4]+l[2]*r[7], l[0]*r[2]+l[1]*r[5]+l[2]*r[8],
            l[3]*r[0]+l[4]*r[3]+l[5]*r[6], l[3]*r[1]+l[4]*r[4]+l[5]*r[7], l[3]*r[2]+l[4]*r[5]+l[5]*r[8],
            l[6]*r[0]+l[7]*r[3]+l[8]*r[6], l[6]*r[1]+l[7]*r[4]+l[8]*r[7], l[6]*r[2]+l[7]*r[5]+l[8]*r[8]
        )
    }
}

// Neg
impl Neg for Matrix3x3 {
    type Output = Self;
    fn neg(self) -> Self { self * -1.0 }
}

// ====================== Assign Ops ======================

impl AddAssign for Matrix3x3 {
    fn add_assign(&mut self, rhs: Self) { for i in 0..9 { self.m[i] += rhs.m[i]; } }
}
impl SubAssign for Matrix3x3 {
    fn sub_assign(&mut self, rhs: Self) { for i in 0..9 { self.m[i] -= rhs.m[i]; } }
}
impl MulAssign<f64> for Matrix3x3 {
    fn mul_assign(&mut self, rhs: f64) { for i in 0..9 { self.m[i] *= rhs; } }
}
impl MulAssign for Matrix3x3 {
    fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
}

// ====================== Display ======================

impl fmt::Display for Matrix3x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mat3x3[{},{},{}; {},{},{}; {},{},{}]",
               self.m[0], self.m[1], self.m[2],
               self.m[3], self.m[4], self.m[5],
               self.m[6], self.m[7], self.m[8])
    }
}