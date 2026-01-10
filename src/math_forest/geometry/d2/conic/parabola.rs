// src/math_forest/geometry/d2/conic/parabola.rs
#![allow(dead_code)]

use std::fmt;

use crate::math_forest::algebra::fertile::d_num::DNum;
use crate::math_forest::algebra::fertile::q_num::QNum;
use crate::math_forest::geometry::d2::fertile::d_point::DPoint;
use crate::math_forest::geometry::d2::fertile::q_point::QPoint;
use crate::math_forest::geometry::d2::linear::line::Line;
use crate::math_forest::geometry::d2::linear::vec2::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Parabola {
    pub p: Vec2,  // 顶点 (Vertex)
    pub v: Vec2,  // 对称轴向量 (Axis Vector) - 其模长决定了开口大小/焦距
}

impl Parabola {
    /// 构造函数
    /// 不使用 Option，建议直接明确传入参数。如果需要默认值，可使用 Default trait 或辅助方法。
    #[inline(always)]
    pub fn new(p: Vec2, v: Vec2) -> Self {
        Self { p, v }
    }

    /// 默认抛物线
    pub fn std() -> Self {
        Self { p: Vec2::ZERO, v: Vec2::J } // 开口向上 y = x^2/4
    }

    pub fn get_type(&self) -> &str { "Parabola" }

    // ====================== 几何属性 ======================

    /// 获取切向基向量 U (垂直于 V)
    /// 对应参数 t 的线性方向
    #[inline]
    pub fn u(&self) -> Vec2 {
        self.v.roll90()
    }

    /// 焦距 f
    /// 基于公式 P(t) = P + tU + (t^2/4)V
    /// 在标准抛物线 y = x^2 / 4f 中，这里对应 f = |v|
    pub fn focal_length(&self) -> f64 {
        self.v.len()
    }

    /// 焦点 F = P + V
    /// (这是基于 t^2/4 系数推导的，如果系数不同，位置会变)
    pub fn focus(&self) -> Vec2 {
        self.p + self.v
    }

    /// 准线 L: 过 P - V，方向为 U
    pub fn directrix(&self) -> Line {
        Line::new(self.p - self.v, self.u())
    }

    // ====================== 核心索引 ======================

    /// 计算索引点 P(t) = P + t*U + (t^2 / 4)*V
    #[inline]
    pub fn index_point(&self, t: f64) -> Vec2 {
        // [BugFix] 增加了 self.p (原代码漏了)
        // 优化：乘以 0.25 比除以 4.0 略快
        self.p + self.u() * t + self.v * (t * t * 0.25)
    }

    pub fn index_d_point(&self, theta: DNum) -> DPoint {
        DPoint::new(self.index_point(theta.n1), self.index_point(theta.n2))
    }

    pub fn index_q_point(&self, theta: QNum) -> QPoint {
        QPoint::new(
            self.index_point(theta.n1),
            self.index_point(theta.n2),
            self.index_point(theta.n3),
            self.index_point(theta.n4),
        )
    }

    // ====================== 导数与切线 ======================

    /// 导数 P'(t) = U + (t/2)V
    pub fn der(&self, t: f64) -> Vec2 {
        self.u() + self.v * (t * 0.5)
    }

    /// 切线
    pub fn tangent_line(&self, t: f64) -> Line {
        Line::new(self.index_point(t), self.der(t))
    }

    // ====================== 距离优化求解 ======================
    // 抛物线到点的距离也是一个求解三次/四次方程的问题

    fn dist_sq(&self, p: Vec2, t: f64) -> f64 {
        self.index_point(t).dis_pow2(p)
    }

    fn dist_sq_der(&self, p: Vec2, t: f64) -> f64 {
        let pt = self.index_point(t);
        let der = self.der(t);
        2.0 * (pt - p).dot(der)
    }

    /// 寻找最近点的参数 t (牛顿法)
    pub fn theta_closest_p(&self, p: Vec2, tolerance: f64, max_iter: usize) -> f64 {
        // 初始猜测：将点投影到轴上估算 t
        // P ~ P0 + tU => t ~ (P - P0) . U / |U|^2
        let diff = p - self.p;
        let u = self.u();
        let mut t = diff.dot(u) / u.pow2();

        for _ in 0..max_iter {
            let f = self.dist_sq_der(p, t);
            if f.abs() < tolerance { break; }

            // 二阶导 f''(t)
            // P''(t) = V / 2
            let der = self.der(t);
            let der2 = self.v * 0.5;
            let pt = self.index_point(t);
            let f_prime = 2.0 * (der.pow2() + (pt - p).dot(der2)); // (f')'

            if f_prime.abs() < 1e-9 { break; }
            t -= f / f_prime;
        }
        t
    }

    pub fn closest_p(&self, p: Vec2) -> Vec2 {
        let t = self.theta_closest_p(p, 1e-8, 20);
        self.index_point(t)
    }
}

// ====================== 格式化显示 ======================

impl fmt::Display for Parabola {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 Dart 的 toString
        write!(f, "Parabola(Vertex: {}, Axis: {})", self.p, self.v)
    }
}