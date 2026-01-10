// src/math_forest/geometry/d2/linear/vec2.rs
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const EPSILON: f64 = 1e-10;
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub const INF: Vec2 = Vec2 { x: f64::INFINITY, y: f64::INFINITY };
    pub const NAN: Vec2 = Vec2 { x: f64::NAN, y: f64::NAN };

    pub const I: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    pub const J: Vec2 = Vec2 { x: 0.0, y: 1.0 };
    pub const A: Vec2 = Vec2 { x: 1.0, y: 1.0 }; // (1, 1) 通常用于缩放

    // 构造
    #[inline(always)]
    pub fn new(x: f64, y: f64) -> Self { Vec2 { x, y } }

    #[inline(always)]
    pub fn from_angle_length(theta: f64, l: f64) -> Self {
        Vec2 { x: theta.cos() * l, y: theta.sin() * l }
    }

    // --- 核心几何计算 (全部使用传值，性能最优) ---

    #[inline]
    pub fn dot(self, other: Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn cross(self, other: Vec2) -> f64 {
        self.x * other.y - self.y * other.x
    }

    #[inline]
    pub fn cross_len(self, other: Vec2) -> f64 {
        (self.x * other.y - self.y * other.x).abs()
    }

    #[inline]
    pub fn pow2(self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn len(self) -> f64 {
        self.pow2().sqrt()
    }

    // 两个点之间的距离
    #[inline]
    pub fn dis(self, p: Vec2) -> f64 {
        (self - p).len()
    }

    #[inline]
    pub fn dis_pow2(self, p: Vec2) -> f64 {
        (self - p).pow2()
    }

    #[inline]
    pub fn unit(self) -> Vec2 {
        let l = self.len();
        if l > Self::EPSILON { self / l } else { Vec2::ZERO }
    }

    // 投影：self 投影到 other 上
    #[inline]
    pub fn project_vec(self, other: Vec2) -> Vec2 {
        let other_u = other.unit();
        other_u * self.dot(other_u) // 使用单位向量点积，无需再除模长
    }

    #[inline]
    pub fn project(self, other: Vec2) -> f64 {
        self.dot(other) / other.len()
    }

    // RSV (Resolve Vector) - 分解向量
    // 返回 (lambda, mu) 使得 self = lambda * a + mu * b
    // 优化：使用克拉默法则，无需调用 powf
    pub fn rsv(self, a: Vec2, b: Vec2) -> (f64, f64) {
        let det = a.cross(b);
        if det.abs() < Self::EPSILON {
            return (f64::NAN, f64::NAN);
        }
        let lam = self.cross(b) / det;
        let mu = a.cross(self) / det;
        (lam, mu)
    }

    // 垂直判定
    #[inline]
    pub fn is_vertical(self, other: Vec2) -> bool {
        self.dot(other).abs() < Self::EPSILON
    }

    // 平行判定
    #[inline]
    pub fn is_parallel(self, other: Vec2) -> bool {
        self.cross_len(other) < Self::EPSILON
    }

    // 角平分线
    pub fn angle_bisector(self, other: Vec2) -> Vec2 {
        self.unit() + other.unit()
    }

    // 旋转 90 度 (逆时针)
    #[inline]
    pub fn roll90(self) -> Vec2 {
        Vec2 { x: -self.y, y: self.x }
    }

    // 夹角余弦值
    pub fn cos(self, other: Vec2) -> f64 {
        let den = self.len() * other.len();
        if den < Self::EPSILON { 0.0 } else { self.dot(other) / den }
    }
}

// ====================== 运算符重载 (宏魔法) ======================

// 1. 实现基础的 Value op Value
impl Add for Vec2 {
    type Output = Vec2;
    #[inline] fn add(self, rhs: Vec2) -> Vec2 { Vec2::new(self.x + rhs.x, self.y + rhs.y) }
}
impl Sub for Vec2 {
    type Output = Vec2;
    #[inline] fn sub(self, rhs: Vec2) -> Vec2 { Vec2::new(self.x - rhs.x, self.y - rhs.y) }
}
impl Neg for Vec2 {
    type Output = Vec2;
    #[inline] fn neg(self) -> Vec2 { Vec2::new(-self.x, -self.y) }
}

// 2. 定义宏来自动生成 &T op T, T op &T, &T op &T
macro_rules! impl_bin_op_permutations {
    ($Trait:ident, $method:ident) => {
        // &Vec2 op Vec2
        impl $Trait<Vec2> for &Vec2 {
            type Output = Vec2;
            #[inline] fn $method(self, rhs: Vec2) -> Vec2 { (*self).$method(rhs) }
        }
        // Vec2 op &Vec2
        impl $Trait<&Vec2> for Vec2 {
            type Output = Vec2;
            #[inline] fn $method(self, rhs: &Vec2) -> Vec2 { self.$method(*rhs) }
        }
        // &Vec2 op &Vec2
        impl $Trait<&Vec2> for &Vec2 {
            type Output = Vec2;
            #[inline] fn $method(self, rhs: &Vec2) -> Vec2 { (*self).$method(*rhs) }
        }
    };
}

// 应用宏
impl_bin_op_permutations!(Add, add);
impl_bin_op_permutations!(Sub, sub);

// ====================== 乘法与除法 (涉及 f64) ======================

// Vec2 * f64
impl Mul<f64> for Vec2 {
    type Output = Vec2;
    #[inline] fn mul(self, rhs: f64) -> Vec2 { Vec2::new(self.x * rhs, self.y * rhs) }
}
// f64 * Vec2 (交换律)
impl Mul<Vec2> for f64 {
    type Output = Vec2;
    #[inline] fn mul(self, rhs: Vec2) -> Vec2 { rhs * self }
}
// Vec2 / f64
impl Div<f64> for Vec2 {
    type Output = Vec2;
    #[inline] fn div(self, rhs: f64) -> Vec2 { Vec2::new(self.x / rhs, self.y / rhs) }
}

// 为引用类型也实现乘除 (复用 Value 的实现)
impl Mul<f64> for &Vec2 { type Output = Vec2; #[inline] fn mul(self, rhs: f64) -> Vec2 { *self * rhs } }
impl Mul<&Vec2> for f64 { type Output = Vec2; #[inline] fn mul(self, rhs: &Vec2) -> Vec2 { *rhs * self } }
impl Div<f64> for &Vec2 { type Output = Vec2; #[inline] fn div(self, rhs: f64) -> Vec2 { *self / rhs } }

// ====================== 赋值运算符 (+=, -=, *=, /=) ======================
// 物理引擎非常需要 pos += vel * dt;

impl AddAssign for Vec2 {
    #[inline] fn add_assign(&mut self, rhs: Self) { self.x += rhs.x; self.y += rhs.y; }
}
impl SubAssign for Vec2 {
    #[inline] fn sub_assign(&mut self, rhs: Self) { self.x -= rhs.x; self.y -= rhs.y; }
}
impl MulAssign<f64> for Vec2 {
    #[inline] fn mul_assign(&mut self, rhs: f64) { self.x *= rhs; self.y *= rhs; }
}
impl DivAssign<f64> for Vec2 {
    #[inline] fn div_assign(&mut self, rhs: f64) { self.x /= rhs; self.y /= rhs; }
}

// ====================== Display ======================
impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 使用精度控制，避免出现 -0.0000 这种奇怪的显示
        let x = if self.x.abs() < 1e-10 { 0.0 } else { self.x };
        let y = if self.y.abs() < 1e-10 { 0.0 } else { self.y };
        write!(f, "({:.4}, {:.4})", x, y)
    }
}