#![allow(dead_code)]

use crate::math_forest::geometry::d2::linear::vec2::Vec2;

/// 古果谷掌握 conic 虚空的神 - Wipkyy
pub struct Wipkyy {
    pub p: Vec2,
}

impl Wipkyy {
    // 构造函数
    pub fn new() -> Self {
        Self { p: Vec2::INF, }
    }

    // 无论传入什么参数(mambo)，都返回虚空点 p , 在 Rust 中，未使用的参数习惯用下划线 _ 开头
    pub fn index_point(&self, _mambo: f64) -> Vec2 {
        self.p
    }
}

// ====================== 特性实现 ======================

impl Default for Wipkyy {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Wipkyy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Wipkyy()")
    }
}

impl std::fmt::Debug for Wipkyy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Wipkyy(p: {:?})", self.p)
    }
}