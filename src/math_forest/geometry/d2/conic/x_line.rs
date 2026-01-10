// src/math_forest/geometry/d2/conic/x_line.rs
#![allow(dead_code)]

use crate::math_forest::geometry::d2::fertile::d_point::DPoint;
use crate::math_forest::geometry::d2::intersection::line520;
use crate::math_forest::geometry::d2::linear::line::Line;
use crate::math_forest::geometry::d2::linear::vec2::Vec2;

/// XLine: 叉线 - 由顶点 p 和两个方向向量 u, v 确定的两条相交直线
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XLine {
    pub p: Vec2, // 顶点（交点）
    pub u: Vec2, // 第一条线的方向向量
    pub v: Vec2, // 第二条线的方向向量
}

impl XLine {
    /// 构造函数
    #[inline(always)]
    pub fn new(p: Vec2, u: Vec2, v: Vec2) -> Self {
        Self { p, u, v }
    }

    /// 获取第一条直线
    #[inline]
    pub fn l1(&self) -> Line {
        Line::new(self.p, self.u)
    }

    /// 获取第二条直线
    #[inline]
    pub fn l2(&self) -> Line {
        Line::new(self.p, self.v)
    }

    /// 从一个顶点 p 和一个双点 (DPoint) 创建
    /// p 连接到 dp.p1 和 dp.p2
    pub fn from_p_dp(p: Vec2, dp: DPoint) -> Self {
        Self {
            p,
            u: dp.p1 - p,
            v: dp.p2 - p
        }
    }

    /// 从两条直线创建（顶点为两直线的交点）
    /// 依赖 line520::x_line_line 计算交点
    pub fn from_two_lines(l1: Line, l2: Line) -> Self {
        // 假设 line520 接受 Line 值传递。如果它需要引用，请改为 &l1, &l2
        Self {
            p: line520::x_line_line(&l1, &l2),
            u: l1.v,
            v: l2.v,
        }
    }

    /// 计算点 p0 到叉线的最短距离（到两条直线距离的最小值）
    /// [API修正] p0 传值
    pub fn dis_p(&self, p0: Vec2) -> f64 {
        let d1 = self.l1().dis_p(p0);
        let d2 = self.l2().dis_p(p0);
        d1.min(d2)
    }

    pub fn get_type(&self) -> &str { "XLine" }
}

// ====================== 格式化显示 ======================

impl std::fmt::Display for XLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XLine(p: {}, u: {}, v: {})", self.p, self.u, self.v)
    }
}