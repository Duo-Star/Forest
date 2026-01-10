// src/math_forest/geometry/d3/linear/vec3.rs
#![allow(dead_code)]

use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const EPSILON: f64 = 1e-10;

    // 常用常量
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    }; // 全1向量
    pub const I: Vec3 = Vec3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    }; // X轴
    pub const J: Vec3 = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    }; // Y轴
    pub const K: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    }; // Z轴
    pub const INF: Vec3 = Vec3 {
        x: f64::INFINITY,
        y: f64::INFINITY,
        z: f64::INFINITY,
    };
    pub const NAN: Vec3 = Vec3 {
        x: f64::NAN,
        y: f64::NAN,
        z: f64::NAN,
    };

    /// 基础构造
    #[inline(always)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    /// 球坐标构造 (ISO约定: theta为极角(与Z轴夹角), phi为方位角(XY平面))
    /// x = r sin(theta) cos(phi)
    /// y = r sin(theta) sin(phi)
    /// z = r cos(theta)
    #[inline]
    pub fn from_spherical(theta: f64, phi: f64, r: f64) -> Self {
        let (sin_t, cos_t) = theta.sin_cos();
        let (sin_p, cos_p) = phi.sin_cos();
        Vec3 {
            x: r * sin_t * cos_p,
            y: r * sin_t * sin_p,
            z: r * cos_t,
        }
    }

    #[inline]
    pub fn rand() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    // ====================== 核心几何计算 ======================

    /// 点积 (Dot Product)
    #[inline]
    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// 叉积 (Cross Product) - 3D中返回向量
    #[inline]
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// 模的平方
    #[inline]
    pub fn pow2(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// 模长
    #[inline]
    pub fn len(self) -> f64 {
        self.pow2().sqrt()
    }

    /// 两个点之间的距离
    #[inline]
    pub fn dis(self, p: Vec3) -> f64 {
        (self - p).len()
    }

    /// 距离平方
    #[inline]
    pub fn dis_pow2(self, p: Vec3) -> f64 {
        (self - p).pow2()
    }

    /// 归一化 (Unit Vector)
    #[inline]
    pub fn unit(self) -> Vec3 {
        let l = self.len();
        if l > Self::EPSILON {
            self / l
        } else {
            Vec3::ZERO
        }
    }

    /// 投影向量: self 投影到 other 上
    #[inline]
    pub fn project_vec(self, other: Vec3) -> Vec3 {
        let other_u = other.unit();
        other_u * self.dot(other_u)
    }

    /// 投影长度 (带符号)
    #[inline]
    pub fn project(self, other: Vec3) -> f64 {
        self.dot(other) / other.len()
    }

    /// 混合积 (Scalar Triple Product): (self x a) . b
    /// 代表以 self, a, b 为边的平行六面体体积
    #[inline]
    pub fn triple_product(self, a: Vec3, b: Vec3) -> f64 {
        self.cross(a).dot(b)
    }

    /// RSV (Resolve Vector) - 3D版本
    /// 将向量分解到基向量 a, b, c
    /// 返回 (lam, mu, nu) 使得 self = lam*a + mu*b + nu*c
    /// 原理：克拉默法则 / 混合积
    pub fn rsv(self, a: Vec3, b: Vec3, c: Vec3) -> (f64, f64, f64) {
        // 行列式 D = [a, b, c]
        let det = a.cross(b).dot(c);

        if det.abs() < Self::EPSILON {
            return (f64::NAN, f64::NAN, f64::NAN);
        }

        // lam = [self, b, c] / D
        let lam = self.cross(b).dot(c) / det;
        // mu = [a, self, c] / D = [c, a, self] / D
        let mu = c.cross(a).dot(self) / det;
        // nu = [a, b, self] / D
        let nu = a.cross(b).dot(self) / det;

        (lam, mu, nu)
    }

    /// 垂直判定
    #[inline]
    pub fn is_vertical(self, other: Vec3) -> bool {
        self.dot(other).abs() < Self::EPSILON
    }

    /// 平行判定 (叉积模长接近0)
    #[inline]
    pub fn is_parallel(self, other: Vec3) -> bool {
        self.cross(other).pow2() < Self::EPSILON * Self::EPSILON
    }

    /// 夹角余弦值
    pub fn cos(self, other: Vec3) -> f64 {
        let den = self.len() * other.len();
        if den < Self::EPSILON {
            0.0
        } else {
            self.dot(other) / den
        }
    }
}

// ====================== 运算符重载 (宏魔法) ======================

// 1. 基础实现 Value op Value
impl Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

// 2. 宏生成引用组合
macro_rules! impl_bin_op_permutations {
    ($Trait:ident, $method:ident) => {
        impl $Trait<Vec3> for &Vec3 {
            type Output = Vec3;
            #[inline]
            fn $method(self, rhs: Vec3) -> Vec3 {
                (*self).$method(rhs)
            }
        }
        impl $Trait<&Vec3> for Vec3 {
            type Output = Vec3;
            #[inline]
            fn $method(self, rhs: &Vec3) -> Vec3 {
                self.$method(*rhs)
            }
        }
        impl $Trait<&Vec3> for &Vec3 {
            type Output = Vec3;
            #[inline]
            fn $method(self, rhs: &Vec3) -> Vec3 {
                (*self).$method(*rhs)
            }
        }
    };
}

impl_bin_op_permutations!(Add, add);
impl_bin_op_permutations!(Sub, sub);

// ====================== 乘法与除法 (涉及 f64) ======================

// Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}
// Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

// 引用支持
impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f64) -> Vec3 {
        *self * rhs
    }
}
impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Vec3 {
        *rhs * self
    }
}
impl Div<f64> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f64) -> Vec3 {
        *self / rhs
    }
}

// ====================== 赋值运算符 ======================

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// ====================== Display ======================
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = if self.x.abs() < 1e-10 { 0.0 } else { self.x };
        let y = if self.y.abs() < 1e-10 { 0.0 } else { self.y };
        let z = if self.z.abs() < 1e-10 { 0.0 } else { self.z };
        write!(f, "({:.4}, {:.4}, {:.4})", x, y, z)
    }
}
