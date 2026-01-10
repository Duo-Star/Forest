// src/math_forest/geometry/d2/conic/h_line.rs
#![allow(dead_code)]

use std::fmt;
use crate::math_forest::geometry::d2::linear::line::Line;
use crate::math_forest::geometry::d2::linear::vec2::Vec2;

/// HLine: 平行双直线 (Parallel Double Lines)
/// 通常用于表示圆锥曲线的准线对
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HLine {
    pub p1: Vec2,  // 第一条线的基点
    pub p2: Vec2,  // 第二条线的基点
    pub v: Vec2,   // 统一的方向向量
}

impl HLine {
    /// 构造函数
    #[inline(always)]
    pub fn new(p1: Vec2, p2: Vec2, v: Vec2) -> Self {
        Self { p1, p2, v }
    }

    /// 获取第一条直线
    #[inline]
    pub fn l1(&self) -> Line {
        // Vec2 是 Copy 的，直接传值，无需 clone()
        Line::new(self.p1, self.v)
    }

    /// 获取第二条直线
    #[inline]
    pub fn l2(&self) -> Line {
        Line::new(self.p2, self.v)
    }

    pub fn get_type(&self) -> &str { "HLine" }
}

// ====================== 格式化显示 ======================

impl fmt::Display for HLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HLine(p1: {}, p2: {}, v: {})", self.p1, self.p2, self.v)
    }
}