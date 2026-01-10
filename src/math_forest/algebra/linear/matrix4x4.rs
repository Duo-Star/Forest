// src/math_forest/algebra/linear/matrix4x4.rs
#![allow(dead_code)]

use std::fmt;
use std::ops::{Add, Sub, Mul, Neg, AddAssign, SubAssign, MulAssign};
use crate::math_forest::algebra::solver::linear::{det4x4, solve_linear_4x4};
use crate::math_forest::geometry::d3::linear::vec3::Vec3;

/// 4x4 矩阵，按行优先存储 (Row-Major)
/// [ m00, m01, m02, m03 ]
/// [ m10, m11, m12, m13 ]
/// [ m20, m21, m22, m23 ]
/// [ m30, m31, m32, m33 ]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4x4 {
    pub m: [f64; 16],
}

impl Matrix4x4 {
    pub const IDENTITY: Matrix4x4 = Matrix4x4 {
        m: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ]
    };

    pub const ZERO: Matrix4x4 = Matrix4x4 { m: [0.0; 16] };
    pub const NAN: Matrix4x4 = Matrix4x4 { m: [f64::NAN; 16] };

    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        m00: f64, m01: f64, m02: f64, m03: f64,
        m10: f64, m11: f64, m12: f64, m13: f64,
        m20: f64, m21: f64, m22: f64, m23: f64,
        m30: f64, m31: f64, m32: f64, m33: f64,
    ) -> Self {
        Self {
            m: [
                m00, m01, m02, m03,
                m10, m11, m12, m13,
                m20, m21, m22, m23,
                m30, m31, m32, m33,
            ],
        }
    }

    /// 从四个列向量构造 (适配 glam 风格，但在内部转为 Row-Major 存储)
    pub fn from_cols(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3, w_axis: Vec3) -> Self {
        // 注意：Vec3 只有 x,y,z，这里假设 w 分量为 0,0,0,1
        Self::new(
            x_axis.x, y_axis.x, z_axis.x, w_axis.x,
            x_axis.y, y_axis.y, z_axis.y, w_axis.y,
            x_axis.z, y_axis.z, z_axis.z, w_axis.z,
            0.0,      0.0,      0.0,      1.0
        )
    }

    /// 创建对角矩阵
    pub fn from_diagonal(d: Vec3) -> Self {
        Self::new(
            d.x, 0.0, 0.0, 0.0,
            0.0, d.y, 0.0, 0.0,
            0.0, 0.0, d.z, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    // ====================== 3D 仿射变换构造 ======================

    /// 构造平移矩阵
    #[inline]
    pub fn from_translation(t: Vec3) -> Self {
        // Row-Major: 平移在第4列 (indices 3, 7, 11)
        Self::new(
            1.0, 0.0, 0.0, t.x,
            0.0, 1.0, 0.0, t.y,
            0.0, 0.0, 1.0, t.z,
            0.0, 0.0, 0.0, 1.0
        )
    }

    /// 构造缩放矩阵
    #[inline]
    pub fn from_scale(s: Vec3) -> Self {
        Self::new(
            s.x, 0.0, 0.0, 0.0,
            0.0, s.y, 0.0, 0.0,
            0.0, 0.0, s.z, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    /// 绕任意轴旋转 (Axis-Angle)
    pub fn from_axis_angle(axis: Vec3, angle: f64) -> Self {
        let axis = axis.unit(); // 确保归一化
        let (sin, cos) = angle.sin_cos();
        let omc = 1.0 - cos;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        Self::new(
            cos + x*x*omc,   x*y*omc - z*sin, x*z*omc + y*sin, 0.0,
            y*x*omc + z*sin, cos + y*y*omc,   y*z*omc - x*sin, 0.0,
            z*x*omc - y*sin, z*y*omc + x*sin, cos + z*z*omc,   0.0,
            0.0,             0.0,             0.0,             1.0
        )
    }

    /// 构造复合变换: T * R * S (先缩放，再旋转，再平移)
    /// 注意：由于暂无 Quat，这里 rotation 使用 Axis-Angle
    pub fn from_scale_rotation_translation(scale: Vec3, axis: Vec3, angle: f64, translation: Vec3) -> Self {
        let t_mat = Self::from_translation(translation);
        let r_mat = Self::from_axis_angle(axis, angle);
        let s_mat = Self::from_scale(scale);

        // T * R * S
        t_mat * r_mat * s_mat
    }

    // ====================== 投影与相机矩阵 (Graphics) ======================

    /// [Graphics] 构造 LookAt 矩阵 (右手坐标系 Right-Handed)
    /// eye: 摄像机位置
    /// center: 观察目标点
    /// up: 上方向量
    pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let f = (center - eye).unit(); // forward
        let s = f.cross(up).unit();    // side (right)
        let u = s.cross(f);            // up

        // 构造 View 矩阵 (世界 -> 相机)
        // 旋转部分 (基向量的转置)
        // [ s.x  s.y  s.z  0 ]
        // [ u.x  u.y  u.z  0 ]
        // [-f.x -f.y -f.z  0 ]
        // [ 0    0    0    1 ]
        // 平移部分: -eye

        // 组合结果 (Row-Major):
        Self::new(
            s.x,  s.y,  s.z, -s.dot(eye),
            u.x,  u.y,  u.z, -u.dot(eye),
            -f.x, -f.y, -f.z,  f.dot(eye),
            0.0,  0.0,  0.0,  1.0
        )
    }

    /// [Graphics] 透视投影 (右手系, OpenGL 风格 [-1, 1] 深度)
    pub fn perspective_rh_gl(fov_y_radians: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Self {
        let inv_len = 1.0 / (z_near - z_far);
        let f = 1.0 / (0.5 * fov_y_radians).tan();

        // Row-Major 布局，注意与参考代码(Col-Major)的行列对应关系
        Self::new(
            f / aspect_ratio, 0.0,  0.0, 0.0,
            0.0,              f,    0.0, 0.0,
            0.0,              0.0,  (z_far + z_near) * inv_len, (2.0 * z_far * z_near) * inv_len,
            0.0,              0.0, -1.0, 0.0
        )
    }

    /// [Graphics] 正交投影 (右手系, OpenGL 风格)
    pub fn orthographic_rh_gl(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Self {
        let w_inv = 1.0 / (right - left);
        let h_inv = 1.0 / (top - bottom);
        let d_inv = 1.0 / (far - near);

        Self::new(
            2.0 * w_inv, 0.0,          0.0,           -(right + left) * w_inv,
            0.0,         2.0 * h_inv,  0.0,           -(top + bottom) * h_inv,
            0.0,         0.0,         -2.0 * d_inv,   -(far + near) * d_inv,
            0.0,         0.0,          0.0,            1.0
        )
    }

    // ====================== Vec3 交互 (核心功能) ======================

    /// 变换点 (Transform Point): P' = M * P (隐式 w=1)
    /// 结果不进行透视除法
    #[inline]
    pub fn transform_point3(&self, v: Vec3) -> Vec3 {
        let x = self.m[0] * v.x + self.m[1] * v.y + self.m[2] * v.z + self.m[3];
        let y = self.m[4] * v.x + self.m[5] * v.y + self.m[6] * v.z + self.m[7];
        let z = self.m[8] * v.x + self.m[9] * v.y + self.m[10] * v.z + self.m[11];
        // 仿射变换下 w' 通常为 1，直接忽略
        Vec3::new(x, y, z)
    }

    /// 变换向量 (Transform Vector): V' = M * V (隐式 w=0)
    /// 忽略平移分量，只受旋转和缩放影响
    #[inline]
    pub fn transform_vector3(&self, v: Vec3) -> Vec3 {
        let x = self.m[0] * v.x + self.m[1] * v.y + self.m[2] * v.z;
        let y = self.m[4] * v.x + self.m[5] * v.y + self.m[6] * v.z;
        let z = self.m[8] * v.x + self.m[9] * v.y + self.m[10] * v.z;
        Vec3::new(x, y, z)
    }

    /// [New] 投影点 (Project Point): P' = M * P (隐式 w=1) 并执行透视除法
    /// 结果 = (x'/w', y'/w', z'/w')
    /// 用于将世界坐标转换为 NDC 坐标
    #[inline]
    pub fn project_point3(&self, v: Vec3) -> Vec3 {
        let x = self.m[0] * v.x + self.m[1] * v.y + self.m[2] * v.z + self.m[3];
        let y = self.m[4] * v.x + self.m[5] * v.y + self.m[6] * v.z + self.m[7];
        let z = self.m[8] * v.x + self.m[9] * v.y + self.m[10] * v.z + self.m[11];
        let w = self.m[12] * v.x + self.m[13] * v.y + self.m[14] * v.z + self.m[15];

        if w.abs() > 1e-9 {
            let inv_w = 1.0 / w;
            Vec3::new(x * inv_w, y * inv_w, z * inv_w)
        } else {
            Vec3::new(x, y, z) // 处理无穷远点
        }
    }

    // ====================== 核心运算 ======================

    #[inline]
    pub fn det(&self) -> f64 {
        let m = self.m;
        det4x4(
            m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7],
            m[8], m[9], m[10], m[11], m[12], m[13], m[14], m[15],
        )
    }

    #[inline]
    pub fn transpose(&self) -> Self {
        let m = self.m;
        Self::new(
            m[0], m[4], m[8], m[12],
            m[1], m[5], m[9], m[13],
            m[2], m[6], m[10], m[14],
            m[3], m[7], m[11], m[15]
        )
    }

    /// 求逆 (复用之前的实现，略去冗长的代码，保持功能)
    /// 求逆矩阵
    pub fn inverse(&self) -> Option<Self> {
        let m = self.m;

        let s0 = m[0] * m[5] - m[1] * m[4];
        let s1 = m[0] * m[6] - m[2] * m[4];
        let s2 = m[0] * m[7] - m[3] * m[4];
        let s3 = m[1] * m[6] - m[2] * m[5];
        let s4 = m[1] * m[7] - m[3] * m[5];
        let s5 = m[2] * m[7] - m[3] * m[6];

        let c5 = m[10] * m[15] - m[11] * m[14];
        let c4 = m[9] * m[15] - m[11] * m[13];
        let c3 = m[9] * m[14] - m[10] * m[13];
        let c2 = m[8] * m[15] - m[11] * m[12];
        let c1 = m[8] * m[14] - m[10] * m[12];
        let c0 = m[8] * m[13] - m[9] * m[12];

        let det = s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0;

        if det.abs() < 1e-12 {
            return None;
        }

        let inv_det = 1.0 / det;

        Some(Self::new(
            (m[5] * c5 - m[6] * c4 + m[7] * c3) * inv_det,
            (-m[1] * c5 + m[2] * c4 - m[3] * c3) * inv_det,
            (m[13] * s5 - m[14] * s4 + m[15] * s3) * inv_det,
            (-m[9] * s5 + m[10] * s4 - m[11] * s3) * inv_det,

            (-m[4] * c5 + m[6] * c2 - m[7] * c1) * inv_det,
            (m[0] * c5 - m[2] * c2 + m[3] * c1) * inv_det,
            (-m[12] * s5 + m[14] * s2 - m[15] * s1) * inv_det,
            (m[8] * s5 - m[10] * s2 + m[11] * s1) * inv_det,

            (m[4] * c4 - m[5] * c2 + m[7] * c0) * inv_det,
            (-m[0] * c4 + m[1] * c2 - m[3] * c0) * inv_det,
            (m[12] * s4 - m[13] * s2 + m[15] * s0) * inv_det,
            (-m[8] * s4 + m[9] * s2 - m[11] * s0) * inv_det,

            (-m[4] * c3 + m[5] * c1 - m[6] * c0) * inv_det,
            (m[0] * c3 - m[1] * c1 + m[2] * c0) * inv_det,
            (-m[12] * s3 + m[13] * s1 - m[14] * s0) * inv_det,
            (m[8] * s3 - m[9] * s1 + m[10] * s0) * inv_det
        ))
    }

    /// 求解 Ax = E
    pub fn solve(&self, e1: f64, e2: f64, e3: f64, e4: f64) -> (f64, f64, f64, f64) {
        let m = self.m;
        solve_linear_4x4(
            m[0], m[1], m[2], m[3], e1,
            m[4], m[5], m[6], m[7], e2,
            m[8], m[9], m[10], m[11], e3,
            m[12], m[13], m[14], m[15], e4,
        )
    }
}

// ====================== 运算符重载 ======================

impl Add for Matrix4x4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut m = [0.0; 16];
        for i in 0..16 { m[i] = self.m[i] + rhs.m[i]; }
        Self { m }
    }
}

impl Sub for Matrix4x4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut m = [0.0; 16];
        for i in 0..16 { m[i] = self.m[i] - rhs.m[i]; }
        Self { m }
    }
}

impl Mul<f64> for Matrix4x4 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        let mut m = [0.0; 16];
        for i in 0..16 { m[i] = self.m[i] * rhs; }
        Self { m }
    }
}

impl Mul<Matrix4x4> for f64 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 { rhs * self }
}

// Matrix * Matrix (Row-Major 乘法: Row * Col)
impl Mul for Matrix4x4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let l = self.m;
        let r = rhs.m;
        let mut res = [0.0; 16];
        for row in 0..4 {
            for col in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += l[row * 4 + k] * r[k * 4 + col];
                }
                res[row * 4 + col] = sum;
            }
        }
        Self { m: res }
    }
}

// Matrix * Vec3 (变换点)
impl Mul<Vec3> for Matrix4x4 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        self.transform_point3(rhs)
    }
}

impl Neg for Matrix4x4 {
    type Output = Self;
    fn neg(self) -> Self { self * -1.0 }
}

// Assign Ops
impl AddAssign for Matrix4x4 { fn add_assign(&mut self, rhs: Self) { for i in 0..16 { self.m[i] += rhs.m[i]; } } }
impl SubAssign for Matrix4x4 { fn sub_assign(&mut self, rhs: Self) { for i in 0..16 { self.m[i] -= rhs.m[i]; } } }
impl MulAssign<f64> for Matrix4x4 { fn mul_assign(&mut self, rhs: f64) { for i in 0..16 { self.m[i] *= rhs; } } }
impl MulAssign for Matrix4x4 { fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; } }

// Display
impl fmt::Display for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mat4x4[{},{},{},{}; {},{},{},{}; {},{},{},{}; {},{},{},{}]",
               self.m[0], self.m[1], self.m[2], self.m[3],
               self.m[4], self.m[5], self.m[6], self.m[7],
               self.m[8], self.m[9], self.m[10], self.m[11],
               self.m[12], self.m[13], self.m[14], self.m[15])
    }
}