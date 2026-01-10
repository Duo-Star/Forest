// src/math_forest/geometry/d2/linear/line.rs
use crate::math_forest::algebra::fertile::d_num::DNum;
use crate::math_forest::algebra::fertile::q_num::QNum;
use crate::math_forest::geometry::d2::fertile::d_point::DPoint;
use crate::math_forest::geometry::d2::fertile::q_point::QPoint;
use crate::math_forest::geometry::d2::linear::vec2::Vec2;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    pub p: Vec2, // 基点 Base Point
    pub v: Vec2, // 方向向量 Direction Vector
}

impl Line {
    /// 默认构造：P + λV
    #[inline(always)]
    pub fn new(p: Vec2, v: Vec2) -> Self {
        Self { p, v }
    }

    /// 两点式构造
    #[inline]
    pub fn from_two_points(p1: Vec2, p2: Vec2) -> Self {
        Self {
            p: p1,
            v: p2 - p1,
        }
    }

    // ================= 核心几何方法 =================

    /// 索引点：获取 P(t) = p + v * t
    /// 建议使用简短的命名 at(t) 替代 index_point
    #[inline]
    pub fn index_point(&self, t: f64) -> Vec2 {
        self.p + self.v * t
    }

    /// 多值索引 (DPoint)
    pub fn index_d_point(&self, t: DNum) -> DPoint {
        DPoint {
            p1: self.index_point(t.n1),
            p2: self.index_point(t.n2),
        }
    }

    /// 四值索引 (QPoint)
    pub fn index_q_point(&self, t: QNum) -> QPoint {
        QPoint {
            p1: self.index_point(t.n1),
            p2: self.index_point(t.n2),
            p3: self.index_point(t.n3),
            p4: self.index_point(t.n4),
        }
    }

    /// 判定平行
    /// 这里的 &Line 是合适的，因为 Line 结构体稍大(32字节)，且是"对象"而非"数值"
    #[inline]
    pub fn is_parallel(&self, other: &Line) -> bool {
        self.v.is_parallel(other.v)
    }

    /// 判定垂直
    #[inline]
    pub fn is_vertical(&self, other: &Line) -> bool {
        self.v.is_vertical(other.v)
    }

    /// 点到直线的投影点
    #[inline]
    pub fn project_p(&self, p_target: Vec2) -> Vec2 {
        // 公式：P_base + projection_of(vector_to_target)
        self.p + (p_target - self.p).project_vec(self.v)
    }

    /// 最近点（对于直线等同于投影点，保留此别名为了接口统一）
    #[inline]
    pub fn closest_p(&self, p_target: Vec2) -> Vec2 {
        self.project_p(p_target)
    }

    /// 点到直线的距离
    #[inline]
    pub fn dis_p(&self, p_target: Vec2) -> f64 {
        // 利用叉积计算距离：Area / Base
        // d = |v x (p_target - p)| / |v|
        self.v.cross_len(p_target - self.p) / self.v.len()
    }

    /// 点到直线的距离的平方 (避免开方，性能更高，用于比较远近)
    #[inline]
    pub fn dis_p_pow2(&self, p_target: Vec2) -> f64 {
        let cross = self.v.cross(p_target - self.p);
        (cross * cross) / self.v.pow2()
    }

    /// 反向求解参数 t：已知点 P，求 t 使得 p + v*t = P
    /// 优化：使用向量投影代替分量除法。
    /// 1. 更加数值稳定（避免除以接近0的分量）
    /// 2. 即使点不在直线上，也能返回该点在直线投影处的 t 值 (鲁棒性)
    #[inline]
    pub fn get_t(&self, p_on_line: Vec2) -> f64 {
        let diff = p_on_line - self.p;
        // t = (diff · v) / (v · v)
        // 推导： diff = t * v  => diff · v = t * (v · v)
        let v_sq = self.v.pow2();
        if v_sq < Vec2::EPSILON { 0.0 } else { diff.dot(self.v) / v_sq }
    }

    /// 获取投影点对应的参数 t (语义上的别名)
    #[inline]
    pub fn t_of_project(&self, p_target: Vec2) -> f64 {
        self.get_t(p_target)
    }

    pub fn get_type(&self) -> &str { "Line" }
}

// ====================== Display ======================

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line(p: {}, v: {})", self.p, self.v)
    }
}

// 移除了 Hash 实现。
// 原因：f64 实现 Hash 极其危险（NaN != NaN，-0.0 != 0.0），
// 在几何库中如果将直线作为 HashMap 的 Key，极易导致难以排查的 bug。
// 如果确实需要，建议使用 id 或封装 ordered_float。