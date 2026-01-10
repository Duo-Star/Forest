// src/math_forest/geometry/d2/conic/circle.rs
#![allow(dead_code)]

use std::f64::consts::PI;

use crate::math_forest::algebra::fertile::d_num::DNum;
use crate::math_forest::algebra::fertile::q_num::QNum;
use crate::math_forest::geometry::d2::fertile::d_point::DPoint;
use crate::math_forest::geometry::d2::fertile::q_point::QPoint;
use crate::math_forest::geometry::d2::linear::vec2::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    pub p: Vec2,
    pub r: f64,
}

impl Circle {
    /// 默认构造函数
    #[inline(always)]
    pub fn new(p: Vec2, r: f64) -> Self {
        Self { p, r }
    }

    /// 从圆心和圆上一点创建
    /// [API修正] p_center, p_edge 改为传值，移除 &
    pub fn from_center_and_point(p_center: Vec2, p_edge: Vec2) -> Self {
        Self {
            p: p_center,
            r: p_center.dis(p_edge),
        }
    }

    /// 从直径（骈点）创建
    pub fn from_diameter(dp: DPoint) -> Self {
        Self {
            p: dp.mid(),
            r: dp.len() * 0.5, // 乘法比除法略快
        }
    }

    pub fn area(&self) -> f64 {
        PI * self.r * self.r
    }

    pub fn circumference(&self) -> f64 {
        2.0 * PI * self.r
    }

    // 圆的焦点（退化为圆心，重合双点）
    pub fn f_points(&self) -> DPoint {
        DPoint::overlap(self.p)
    }

    // ====================== 核心几何索引 ======================

    // 根据参数 θ 获取圆上的点
    // 优化：直接调用 Vec2 的极坐标构造
    pub fn index_point(&self, theta: f64) -> Vec2 {
        self.p + Vec2::from_angle_length(theta, self.r)
    }

    /// 索引双点
    pub fn index_d_point(&self, theta: DNum) -> DPoint {
        DPoint::new(
            self.index_point(theta.n1),
            self.index_point(theta.n2),
        )
    }

    /// 索引四点
    pub fn index_q_point(&self, theta: QNum) -> QPoint {
        QPoint::new(
            self.index_point(theta.n1),
            self.index_point(theta.n2),
            self.index_point(theta.n3),
            self.index_point(theta.n4),
        )
    }

    // ====================== 距离与投影 ======================

    /// 找到点 P 对应的圆上最近点的参数 θ
    /// [API修正] p_target 传值
    pub fn theta_closest_p(&self, p_target: Vec2) -> f64 {
        let dp = p_target - self.p;
        dp.y.atan2(dp.x)
    }

    /// 圆上距离点 P 最近的点
    /// [API修正] p_target 传值
    pub fn closest_p(&self, p_target: Vec2) -> Vec2 {
        // unit() 内部已处理零向量情况（返回 ZERO），
        // 如果 p_target == self.p，这里会返回 self.p (圆心)，符合逻辑
        self.p + (p_target - self.p).unit() * self.r
    }

    /// 点 P 到圆边界的距离
    /// [API修正] p0 传值，移除 &
    pub fn dis_p(&self, p0: Vec2) -> f64 {
        (p0.dis(self.p) - self.r).abs()
    }

    pub fn get_type(&self) -> &str { "Circle" }
}

// ====================== 格式化显示 ======================

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cir2({}, r: {:.4})", self.p, self.r)
    }
}