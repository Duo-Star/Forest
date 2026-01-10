// src/math_forest/algebra/linear/matrix2x2.rs
#![allow(dead_code)]

use std::fmt;
use std::ops::{Add, Sub, Mul, Neg, MulAssign, AddAssign, SubAssign};
use crate::math_forest::algebra::solver::linear::solve_linear_2x2;
use crate::math_forest::geometry::d2::linear::vec2::Vec2;

/// 2x2 矩阵，按行优先存储 (Row-Major)
/// [ m00, m01 ]
/// [ m10, m11 ]
/// 这个 Matrix2x2 是 2D 仿射变换 的核心。配合你的 Vec2，你可以轻松实现：
//     点的旋转：Matrix2x2::from_rotation(angle) * vec
//     点的缩放：Matrix2x2::from_scaling(sx, sy) * vec
///     常用构造：增加了 identity() (单位阵), zero(), rotation() (旋转矩阵), scaling() (缩放矩阵)。
//     核心运算：增加了 transpose() (转置), inverse() (求逆), trace() (迹)。
//     Vec2 联动：实现了 Matrix2x2 * Vec2，这是做几何变换最常用的功能。
//     运算符重载：实现了矩阵加减乘、标量乘法。
//     求解器整合：保留并优化了 solve 方法。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix2x2 {
    pub m: [f64; 4],
}

impl Matrix2x2 {
    pub const IDENTITY: Matrix2x2 = Matrix2x2 { m: [1.0, 0.0, 0.0, 1.0] };
    pub const ZERO: Matrix2x2 = Matrix2x2 { m: [0.0, 0.0, 0.0, 0.0] };

    /// 基础构造: new(m00, m01, m10, m11)
    #[inline(always)]
    pub fn new(m00: f64, m01: f64, m10: f64, m11: f64) -> Self {
        Self { m: [m00, m01, m10, m11] }
    }

    /// 构造旋转矩阵 (逆时针 theta)
    /// [ cos -sin ]
    /// [ sin  cos ]
    pub fn from_rotation(theta: f64) -> Self {
        let (sin, cos) = theta.sin_cos();
        Self::new(cos, -sin, sin, cos)
    }

    /// 构造缩放矩阵
    /// [ sx  0 ]
    /// [ 0  sy ]
    pub fn from_scaling(sx: f64, sy: f64) -> Self {
        Self::new(sx, 0.0, 0.0, sy)
    }

    /// 构造切变矩阵 (Shear)
    pub fn from_shear(kx: f64, ky: f64) -> Self {
        Self::new(1.0, kx, ky, 1.0)
    }

    // ====================== 核心属性 ======================

    /// 行列式 (Determinant)
    #[inline]
    pub fn det(&self) -> f64 {
        self.m[0] * self.m[3] - self.m[1] * self.m[2]
    }

    /// 迹 (Trace) = 主对角线之和
    #[inline]
    pub fn trace(&self) -> f64 {
        self.m[0] + self.m[3]
    }

    /// 转置 (Transpose)
    #[inline]
    pub fn transpose(&self) -> Self {
        Self::new(self.m[0], self.m[2], self.m[1], self.m[3])
    }

    /// 求逆矩阵 (Inverse)
    /// 如果行列式接近 0，返回 None (或者你可以选择返回包含 NaN 的矩阵)
    pub fn inverse(&self) -> Option<Self> {
        let det = self.det();
        if det.abs() < 1e-12 {
            return None;
        }
        let inv_det = 1.0 / det;
        // 伴随矩阵 / det
        // [  d  -b ]
        // [ -c   a ]
        Some(Self::new(
            self.m[3] * inv_det, -self.m[1] * inv_det,
            -self.m[2] * inv_det,  self.m[0] * inv_det
        ))
    }

    // ====================== 求解线性方程组 ======================

    /// 求解 Ax = b，其中 b 是 (c1, c2)，返回 (x, y)
    /// 直接复用 linear::solve_linear_2x2
    #[inline]
    pub fn solve(&self, c1: f64, c2: f64) -> (f64, f64) {
        solve_linear_2x2(self.m[0], self.m[1], c1, self.m[2], self.m[3], c2)
    }

    /// 求解 Ax = v (Vec2 版本)
    #[inline]
    pub fn solve_vec2(&self, v: Vec2) -> Vec2 {
        let (x, y) = self.solve(v.x, v.y);
        Vec2::new(x, y)
    }
}

// ====================== 运算符重载 ======================

// 1. Matrix + Matrix
impl Add for Matrix2x2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.m[0] + rhs.m[0], self.m[1] + rhs.m[1],
            self.m[2] + rhs.m[2], self.m[3] + rhs.m[3],
        )
    }
}

// 2. Matrix - Matrix
impl Sub for Matrix2x2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.m[0] - rhs.m[0], self.m[1] - rhs.m[1],
            self.m[2] - rhs.m[2], self.m[3] - rhs.m[3],
        )
    }
}

// 3. Matrix * Matrix (矩阵乘法)
impl Mul for Matrix2x2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        // Row 0 * Col 0, Row 0 * Col 1
        // Row 1 * Col 0, Row 1 * Col 1
        Self::new(
            self.m[0] * rhs.m[0] + self.m[1] * rhs.m[2],
            self.m[0] * rhs.m[1] + self.m[1] * rhs.m[3],
            self.m[2] * rhs.m[0] + self.m[3] * rhs.m[2],
            self.m[2] * rhs.m[1] + self.m[3] * rhs.m[3],
        )
    }
}

// 4. Matrix * Vec2 (变换向量) -> 极其重要
impl Mul<Vec2> for Matrix2x2 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2::new(
            self.m[0] * rhs.x + self.m[1] * rhs.y,
            self.m[2] * rhs.x + self.m[3] * rhs.y,
        )
    }
}

// 5. Matrix * f64 (标量乘法)
impl Mul<f64> for Matrix2x2 {
    type Output = Self;
    #[inline]
    fn mul(self, s: f64) -> Self {
        Self::new(
            self.m[0] * s, self.m[1] * s,
            self.m[2] * s, self.m[3] * s,
        )
    }
}

// f64 * Matrix (交换律)
impl Mul<Matrix2x2> for f64 {
    type Output = Matrix2x2;
    #[inline]
    fn mul(self, rhs: Matrix2x2) -> Matrix2x2 {
        rhs * self
    }
}

// 6. Neg (取反)
impl Neg for Matrix2x2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.m[0], -self.m[1], -self.m[2], -self.m[3])
    }
}

// ====================== 赋值运算符 (Assign) ======================

impl AddAssign for Matrix2x2 {
    fn add_assign(&mut self, rhs: Self) {
        self.m[0] += rhs.m[0]; self.m[1] += rhs.m[1];
        self.m[2] += rhs.m[2]; self.m[3] += rhs.m[3];
    }
}

impl SubAssign for Matrix2x2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.m[0] -= rhs.m[0]; self.m[1] -= rhs.m[1];
        self.m[2] -= rhs.m[2]; self.m[3] -= rhs.m[3];
    }
}

impl MulAssign<f64> for Matrix2x2 {
    fn mul_assign(&mut self, s: f64) {
        self.m[0] *= s; self.m[1] *= s;
        self.m[2] *= s; self.m[3] *= s;
    }
}

// Matrix *= Matrix
impl MulAssign for Matrix2x2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

// ====================== 格式输出 ======================

impl fmt::Display for Matrix2x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mat2x2[{}, {}; {}, {}]",
               self.m[0], self.m[1],
               self.m[2], self.m[3])
    }
}