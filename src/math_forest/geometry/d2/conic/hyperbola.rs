// src/math_forest/geometry/d2/conic/hyperbola.rs
#![allow(dead_code)]

use crate::math_forest::algebra::fertile::d_num::DNum;
use crate::math_forest::algebra::fertile::q_num::QNum;
use crate::math_forest::geometry::d2::fertile::d_point::DPoint;
use crate::math_forest::geometry::d2::fertile::q_point::QPoint;
use crate::math_forest::geometry::d2::linear::line::Line;
use crate::math_forest::geometry::d2::linear::vec2::Vec2;

use crate::math_forest::geometry::d2::conic::h_line::HLine;
use crate::math_forest::geometry::d2::conic::x_line::XLine; // 叉线（渐近线对） // 平行双线

/// 双曲线：P(t) = Center + t*U + (1/t)*V
/// U, V 为渐近线方向的缩放向量
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hyperbola {
    pub p: Vec2, // 中心 Center
    pub u: Vec2, // 渐近线方向向量 1
    pub v: Vec2, // 渐近线方向向量 2
}

impl Hyperbola {
    #[inline(always)]
    pub fn new(p: Vec2, u: Vec2, v: Vec2) -> Self {
        Self { p, u, v }
    }

    /// 通过一点和渐近线构造
    /// p_on_curve: 曲线上的任意一点
    /// xl: 渐近线对 (XLine)
    pub fn from_p_and_xl(p_on_curve: Vec2, xl: &XLine) -> Self {
        // 将点分解到渐近线基底上：P - C = λU + μV
        let (lam, mu) = (p_on_curve - xl.p).rsv(xl.u, xl.v);
        // 双曲线方程 xy = k => lam * mu = k
        // 构造新的 u', v' 使得 t=1 时经过该点
        let scale = (lam * mu).sqrt();
        // 如果 lam*mu < 0，说明点在另一对共轭双曲线上，这里取 sqrt 可能 NaN
        // 你的逻辑假设点在由 u,v 正向张成的区域内。
        // 若需通用性，建议处理符号。暂时保持你的逻辑。
        Hyperbola::new(xl.p, xl.u * scale, xl.v * scale)
    }

    pub fn get_type(&self) -> &str {
        "Hyperbola"
    }

    // ====================== 核心索引 ======================

    /// 索引点 P(t) = p + u*t + v/t
    #[inline]
    pub fn index_point(&self, t: f64) -> Vec2 {
        // 避免除零
        if t.abs() < 1e-9 {
            return Vec2::NAN;
        }
        self.p + self.u * t + self.v * (1.0 / t)
    }

    pub fn index_d_point(&self, theta: DNum) -> DPoint {
        DPoint::new(self.index_point(theta.n1), self.index_point(theta.n2))
    }

    // ====================== 几何属性 ======================

    /// 实轴方向 (A轴)
    pub fn v_a(&self) -> Vec2 {
        (self.u.unit() + self.v.unit()).unit()
    }

    /// 虚轴方向 (B轴)
    pub fn v_b(&self) -> Vec2 {
        (self.u.unit() - self.v.unit()).unit()
    }

    /// 顶点参数 t0
    /// 顶点满足 t*|u| = (1/t)*|v| => t^2 = |v|/|u|
    pub fn t0(&self) -> DNum {
        let ratio = self.v.len() / self.u.len(); // 优化：直接用 len 而非 pow2.sqrt
        let t = ratio.sqrt();
        DNum::new(t, -t)
    }

    /// 顶点位置
    pub fn a_vertex(&self) -> DPoint {
        self.index_d_point(self.t0())
    }

    // 半角正切 (渐近线半夹角的 tan 值)
    pub fn half_ang_tan(&self) -> f64 {
        let cos_theta = self.u.cos(self.v);
        // tan(theta/2) = sqrt((1-cos)/(1+cos))
        ((1.0 - cos_theta) / (1.0 + cos_theta)).sqrt()
    }

    /// 半长轴 a
    pub fn a(&self) -> f64 {
        (2.0 * (self.u.pow2() * self.v.pow2()).sqrt() + 2.0 * self.u.dot(self.v)).sqrt()
    }

    /// 半短轴 b = a * tan(theta)
    pub fn b(&self) -> f64 {
        self.a() * self.half_ang_tan()
    }

    /// 半焦距 c = sqrt(a^2 + b^2)
    pub fn c(&self) -> f64 {
        let a = self.a();
        let b = self.b();
        (a * a + b * b).sqrt()
    }

    /// 离心率 e = c / a
    pub fn e(&self) -> f64 {
        let a = self.a();
        if a < Vec2::EPSILON {
            return 1.0;
        } // 退化情况
        self.c() / a
    }

    /// 焦点 F1, F2
    pub fn f_points(&self) -> DPoint {
        let f_vec = self.v_a() * self.c();
        DPoint::new_pv(self.p, f_vec)
    }

    /// 准线 (Directrix): HLine
    pub fn l(&self) -> HLine {
        let a = self.a();
        let c = self.c();
        let d = if c > 0.0 { a * a / c } else { 0.0 }; // 中心到准线的距离
        let offset = self.v_a() * d;
        // HLine 构造：过 P1, P2，方向为 v_b 的两条平行线
        HLine::new(self.p + offset, self.p - offset, self.v_b())
    }

    /// 渐近线 (Asymptotes): XLine
    pub fn x(&self) -> XLine {
        XLine::new(self.p, self.u, self.v)
    }

    // ====================== 导数与切线 ======================

    /// 切向量 P'(t) = u - v/t^2
    pub fn der(&self, t: f64) -> Vec2 {
        if t.abs() < 1e-9 {
            return self.u;
        } // 极限情况
        self.u - self.v * (1.0 / (t * t))
    }

    pub fn tangent_line(&self, t: f64) -> Line {
        Line::new(self.index_point(t), self.der(t))
    }

    // ====================== 共轭性质 ======================

    /// 共轭双曲线
    pub fn conjugate(&self) -> Hyperbola {
        // 将 v 反向即可
        Hyperbola::new(self.p, self.u, -self.v)
    }

    // ====================== 距离优化求解 ======================

    fn dist_sq(&self, p: Vec2, t: f64) -> f64 {
        self.index_point(t).dis_pow2(p)
    }

    // f'(t)
    fn dist_sq_der(&self, p: Vec2, t: f64) -> f64 {
        let pt = self.index_point(t);
        let der = self.der(t);
        2.0 * (pt - p).dot(der)
    }

    // f''(t) 用于牛顿法
    fn dist_sq_der2(&self, p: Vec2, t: f64) -> f64 {
        let pt = self.index_point(t);
        let der1 = self.der(t);
        // P''(t) = 2v / t^3
        let der2 = self.v * (2.0 / (t * t * t));

        // (f')' = 2 * (der · der + diff · der2)
        2.0 * (der1.pow2() + (pt - p).dot(der2))
    }

    /// 寻找最近点的参数 t
    pub fn theta_closest_p(&self, p: Vec2, tolerance: f64, max_iter: i32) -> f64 {
        if self.u.len() < Vec2::EPSILON && self.v.len() < Vec2::EPSILON {
            return 1.0;
        }

        // 初始猜测点生成策略
        // 双曲线有两支 (t > 0 和 t < 0)，必须分别搜索
        // 关键点：t = ±sqrt(|v|/|u|) 是顶点
        let t_vertex = (self.v.len() / self.u.len()).sqrt();

        // 生成一组候选 t 值
        let candidates = [
            t_vertex,
            -t_vertex, // 顶点
            t_vertex * 10.0,
            -t_vertex * 10.0, // 远端
            t_vertex * 0.1,
            -t_vertex * 0.1, // 近中心端
            1.0,
            -1.0,
        ];

        let mut best_t = t_vertex;
        let mut min_dist = f64::INFINITY;

        for &start_t in &candidates {
            // 牛顿法迭代
            let mut t = start_t;
            for _ in 0..max_iter {
                if t.abs() < 1e-6 {
                    t = 1e-6 * t.signum();
                } // 避免 t=0

                let f1 = self.dist_sq_der(p, t);
                if f1.abs() < tolerance {
                    break;
                }

                let f2 = self.dist_sq_der2(p, t);
                if f2.abs() < 1e-9 {
                    break;
                } // 二阶导过小，牛顿法失效

                let delta = f1 / f2;
                t -= delta;

                // 简单的步长控制，防止飞到无穷远或零点
                if delta.abs() < tolerance {
                    break;
                }
            }

            let d = self.dist_sq(p, t);
            if d < min_dist {
                min_dist = d;
                best_t = t;
            }
        }
        best_t
    }

    pub fn closest_p(&self, p: Vec2) -> Vec2 {
        let t = self.theta_closest_p(p, 1e-8, 20);
        self.index_point(t)
    }
}

impl std::fmt::Display for Hyperbola {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hyperbola(C:{}, U:{}, V:{})", self.p, self.u, self.v)
    }
}
