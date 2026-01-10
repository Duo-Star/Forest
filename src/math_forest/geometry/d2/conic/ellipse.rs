// src/math_forest/geometry/d2/conic/ellipse.rs
#![allow(dead_code)]

use std::f64::consts::PI;

use crate::math_forest::algebra::fertile::d_num::DNum;
use crate::math_forest::algebra::fertile::q_num::QNum;
use crate::math_forest::geometry::d2::fertile::d_point::DPoint;
use crate::math_forest::geometry::d2::fertile::q_point::QPoint;
use crate::math_forest::geometry::d2::linear::line::Line;
use crate::math_forest::geometry::d2::linear::vec2::Vec2;
// use super::x_line::XLine; // 假设 XLine (叉线) 稍后提供

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ellipse {
    pub p: Vec2, // 中心 Center
    pub u: Vec2, // 共轭轴 U (Conjugate Radius U)
    pub v: Vec2, // 共轭轴 V (Conjugate Radius V)
}

impl Ellipse {
    /// 构造函数
    #[inline(always)]
    pub fn new(p: Vec2, u: Vec2, v: Vec2) -> Self {
        Self { p, u, v }
    }

    pub fn get_type(&self) -> &str { "Ellipse" }

    // ====================== 核心索引 ======================

    /// 根据参数 θ 获取曲线上的点 P = C + U cosθ + V sinθ
    #[inline]
    pub fn index_point(&self, theta: f64) -> Vec2 {
        // 利用 Vec2 的值传递和运算符重载
        let (sin, cos) = theta.sin_cos(); // 同时计算 sin, cos 更快
        self.p + self.u * cos + self.v * sin
    }

    /// 索引骈点 (DPoint)
    pub fn index_d_point(&self, theta: DNum) -> DPoint {
        DPoint::new(
            self.index_point(theta.n1),
            self.index_point(theta.n2)
        )
    }

    /// 索引合点 (QPoint)
    pub fn index_q_point(&self, theta: QNum) -> QPoint {
        QPoint::new(
            self.index_point(theta.n1),
            self.index_point(theta.n2),
            self.index_point(theta.n3),
            self.index_point(theta.n4),
        )
    }

    // ====================== 几何属性 ======================

    /// 计算椭圆的长短半轴长度 (a, b)
    /// 这是一个比较昂贵的操作，涉及特征值求解
    pub fn ab(&self) -> (f64, f64) {
        let u2 = self.u.pow2();
        let v2 = self.v.pow2();
        let dot_uv = self.u.dot(self.v);

        let term1 = 0.5 * (u2 + v2);
        let term2_sq = ((u2 - v2) * 0.5).powi(2) + dot_uv.powi(2);
        let term2 = term2_sq.sqrt();

        let a = (term1 + term2).sqrt();
        let b = (term1 - term2).sqrt();
        (a, b)
    }

    // 半长轴
    pub fn a(&self) -> f64 { self.ab().0 }

    // 半短轴
    pub fn b(&self) -> f64 { self.ab().1 }

    // 半焦距 c = sqrt(a^2 - b^2)
    pub fn c(&self) -> f64 {
        let (a, b) = self.ab();
        (a * a - b * b).sqrt()
    }

    // 判断退化 (面积接近0)
    pub fn is_degenerate(&self) -> bool {
        // 叉积代表由 u, v 组成的平行四边形面积，这也正比于椭圆面积
        self.u.cross_len(self.v) < Vec2::EPSILON
    }

    // 辅助参数 h (用于周长计算)
    fn h_param(&self) -> f64 {
        let (a, b) = self.ab();
        let diff = a - b;
        let sum = a + b;
        (diff * diff) / (sum * sum)
    }

    // 离心率 e
    pub fn e(&self) -> f64 {
        let (a, b) = self.ab();
        if a < Vec2::EPSILON { return 0.0; } // 防止除零
        (1.0 - (b * b) / (a * a)).sqrt()
    }

    // 面积: PI * det(u, v) 其实也可以，但用 ab 更加直观
    pub fn area(&self) -> f64 {
        // |u x v| * PI == a * b * PI
        PI * self.u.cross_len(self.v)
    }

    // 周长 (Ramanujan 近似公式 2)
    pub fn circumference(&self) -> f64 {
        let (a, b) = self.ab();
        let h = self.h_param();
        PI * (a + b) * (1.0 + (3.0 * h) / (10.0 + (4.0 - 3.0 * h).sqrt()))
    }

    // ====================== 特殊点获取 ======================

    // 计算长轴和短轴对应的参数 θ 基础值
    fn get_ab_theta_base(&self) -> f64 {
        let u2 = self.u.pow2();
        let v2 = self.v.pow2();
        let uv_dot2 = self.u.dot(self.v) * 2.0;
        (u2 - v2).atan2(uv_dot2) // 注意：这里使用了 atan2(y, x)，原代码顺序似乎是 atan2(x, y)?
        // 修正：原代码 atan2(a1-a3, a2) -> atan2(cos_coeff, sin_coeff)
        // 你的公式对应 tan(2theta) = 2uv / (u^2 - v^2)。
        // 确认逻辑无误即可。
    }

    // 长轴端点参数
    pub fn theta_a(&self) -> DNum {
        let base = self.get_ab_theta_base();
        // 这里的系数调整是根据几何推导得来的，保持原样
        DNum::new(PI * 1.25 - base * 0.5, PI * 2.25 - base * 0.5)
    }

    // 短轴端点参数
    pub fn theta_b(&self) -> DNum {
        let base = self.get_ab_theta_base();
        DNum::new(PI * 0.75 - base * 0.5, PI * 1.75 - base * 0.5)
    }

    pub fn get_a_points(&self) -> DPoint { self.index_d_point(self.theta_a()) }
    pub fn get_b_points(&self) -> DPoint { self.index_d_point(self.theta_b()) }

    // 长轴方向单位向量
    pub fn v_a(&self) -> Vec2 {
        // 取第一个长轴端点减去中心
        (self.index_point(self.theta_a().n1) - self.p).unit()
    }

    // 焦点组 (F1, F2)
    pub fn f_points(&self) -> DPoint {
        let focus_vec = self.v_a() * self.c();
        DPoint::new_pv(self.p, focus_vec)
    }

    // ====================== 导数与切线 ======================

    // 切向量 P'(t) = -U sin t + V cos t
    #[inline]
    pub fn der(&self, theta: f64) -> Vec2 {
        let (sin, cos) = theta.sin_cos();
        self.v * cos - self.u * sin
    }

    // 切线
    pub fn tangent_line_at(&self, theta: f64) -> Line {
        Line::new(self.index_point(theta), self.der(theta))
    }

    // ====================== 距离优化求解 (关键部分) ======================

    // 目标函数：f(t) = |P(t) - Target|^2
    fn dist_sq_at(&self, p_target: Vec2, theta: f64) -> f64 {
        self.index_point(theta).dis_pow2(p_target)
    }

    // 目标函数的导数：f'(t)
    fn dist_sq_der_at(&self, p_target: Vec2, theta: f64) -> f64 {
        let (sin, cos) = theta.sin_cos();
        // P(t) - Target
        let diff = self.p + self.u * cos + self.v * sin - p_target;
        // P'(t)
        let tangent = self.v * cos - self.u * sin;
        // f'(t) = 2 * diff · tangent
        2.0 * diff.dot(tangent)
    }

    // 找到最近点的参数 θ (梯度下降法)
    pub fn theta_closest_p(&self, p_target: Vec2, tolerance: f64, max_iter: usize) -> f64 {
        let mut best_theta = 0.0;
        let mut min_dist_sq = f64::INFINITY;

        // 1. 粗略搜索 (12分度)
        for i in 0..12 {
            let t = i as f64 * PI / 6.0;
            let d = self.dist_sq_at(p_target, t);
            if d < min_dist_sq {
                min_dist_sq = d;
                best_theta = t;
            }
        }

        // 2. 精细搜索 (梯度下降)
        let mut t = best_theta;
        let mut learning_rate = -0.5 / self.u.pow2().max(self.v.pow2()); // 自适应学习率粗估
        // 修正学习率：步长太大会震荡。几何距离的二阶导大概和半径平方成正比。
        // 这里沿用你的经验参数 -0.5 动态调整可能更稳。
        let mut k = -0.05; // 稍微保守一点的初始步长

        let mut prev_val = min_dist_sq;

        for _ in 0..max_iter {
            let grad = self.dist_sq_der_at(p_target, t);
            if grad.abs() < tolerance { break; }

            t += k * grad; // 更新 t

            // 归一化到 [0, 2PI]
            t = t.rem_euclid(2.0 * PI);

            let curr_val = self.dist_sq_at(p_target, t);

            // 简单线搜索/阻尼策略
            if curr_val > prev_val {
                k *= 0.5; // 步子迈大了，缩小步长
            } else {
                if (prev_val - curr_val).abs() < tolerance { break; }
                prev_val = curr_val;
                // k 保持或微增? 通常减小比较稳
            }
        }
        t
    }

    /// 最近点
    pub fn closest_p(&self, p_target: Vec2) -> Vec2 {
        let t = self.theta_closest_p(p_target, 1e-8, 50);
        self.index_point(t)
    }

    /// 最小距离
    pub fn dis_p(&self, p_target: Vec2) -> f64 {
        self.closest_p(p_target).dis(p_target)
    }

    /// 最近点处的切线
    pub fn tangent_line_closest(&self, p_target: Vec2) -> Line {
        let t = self.theta_closest_p(p_target, 1e-8, 50);
        self.tangent_line_at(t)
    }
}

// 格式化
impl std::fmt::Display for Ellipse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ellipse(Center:{}, U:{}, V:{})", self.p, self.u, self.v)
    }
}