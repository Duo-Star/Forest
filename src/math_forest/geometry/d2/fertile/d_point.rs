// src/math_forest/geometry/d2/fertile/d_point.rs
use crate::math_forest::algebra::fertile::d_num::DNum;
use crate::math_forest::algebra::fertile::q_num::QNum;
use crate::math_forest::geometry::d2::linear::line::Line;
use crate::math_forest::geometry::d2::linear::vec2::Vec2;
use std::fmt;
use std::ops::{Add, Sub, Neg};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DPoint {
    pub p1: Vec2,
    pub p2: Vec2,
}

impl DPoint {
    pub const ZERO: DPoint = DPoint { p1: Vec2::ZERO, p2: Vec2::ZERO };
    pub const INF: DPoint = DPoint { p1: Vec2::INF, p2: Vec2::INF };
    pub const NAN: DPoint = DPoint { p1: Vec2::NAN, p2: Vec2::NAN };

    /// 基础构造
    #[inline(always)]
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        Self { p1, p2 }
    }

    /// 中心辐射构造：Center ± Vector
    #[inline]
    pub fn new_pv(center: Vec2, v: Vec2) -> Self {
        Self { p1: center + v, p2: center - v }
    }

    /// 重合点构造
    #[inline]
    pub fn overlap(p: Vec2) -> Self {
        Self { p1: p, p2: p }
    }

    /// 获取中点
    #[inline]
    pub fn mid(self) -> Vec2 {
        (self.p1 + self.p2) * 0.5
    }

    /// 骈点间距（焦距/直径）
    #[inline]
    pub fn len(self) -> f64 {
        self.p1.dis(self.p2)
    }

    /// 确定的直线
    #[inline]
    pub fn line(self) -> Line {
        Line::from_two_points(self.p1, self.p2)
    }

    /// 索引获取点 (0 -> p1, 其他 -> p2)
    /// 建议：如果仅仅是获取两点之一，直接访问字段 .p1/.p2 更快。
    /// 这个方法适合用于循环或统一接口。
    #[inline]
    pub fn at_index(self, i: usize) -> Vec2 {
        if i == 0 { self.p1 } else { self.p2 }
    }

    /// 计算点 p_target 到骈点中各点的距离，返回 DNum
    #[inline]
    pub fn dis_p(self, p_target: Vec2) -> DNum {
        DNum::new(p_target.dis(self.p1), p_target.dis(self.p2))
    }

    /// 调和分割 (Harmonic Division)
    /// 给定比例 t，求两点的调和分割点对
    /// 依赖：QNum, Line::at_q_num (上一轮重命名), QPoint::dp2 (假设存在)
    pub fn harmonic(self, t: f64) -> DPoint {
        // [0, 1] 是线段 p1->p2 的自然参数化
        let qn = QNum::harmonic(DNum::new(0.0, 1.0), t);

        // 注意：这里调用了 Line 的 at_q_num (即原 index_q_point)
        // 假设 QPoint 有 dp2() 方法提取后两个点(调和共轭点)
        self.line().index_q_point(qn).dp2()
    }

    // 交换两点顺序
    pub fn swap(self) -> Self {
        Self { p1: self.p2, p2: self.p1 }
    }
}

// ====================== 格式化显示 ======================

impl fmt::Display for DPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.p1, self.p2) // 使用花括号表示集合感
    }
}

// ====================== 运算符支持 ======================

// DPoint + Vec2 (整体平移)
impl Add<Vec2> for DPoint {
    type Output = DPoint;
    #[inline]
    fn add(self, rhs: Vec2) -> Self::Output {
        DPoint { p1: self.p1 + rhs, p2: self.p2 + rhs }
    }
}

// DPoint - Vec2 (整体平移)
impl Sub<Vec2> for DPoint {
    type Output = DPoint;
    #[inline]
    fn sub(self, rhs: Vec2) -> Self::Output {
        DPoint { p1: self.p1 - rhs, p2: self.p2 - rhs }
    }
}

// -DPoint (关于原点对称)
impl Neg for DPoint {
    type Output = DPoint;
    #[inline]
    fn neg(self) -> Self::Output {
        DPoint { p1: -self.p1, p2: -self.p2 }
    }
}