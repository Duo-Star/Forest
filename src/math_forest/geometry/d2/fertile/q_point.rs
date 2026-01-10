// src/math_forest/geometry/d2/fertile/q_point.rs
#![allow(dead_code)]

use crate::math_forest::algebra::fertile::d_num::DNum;
use crate::math_forest::algebra::fertile::q_num::QNum;
use crate::math_forest::geometry::d2::fertile::d_point::DPoint;
use crate::math_forest::geometry::d2::linear::line::Line;
use crate::math_forest::geometry::d2::linear::vec2::Vec2;

// 假设 XLine, DXLine, line520 都在对应的位置
use crate::math_forest::geometry::d2::conic::x_line::XLine;
use crate::math_forest::geometry::d2::fertile::d_x_line::DXLine;
use crate::math_forest::geometry::d2::intersection::line520;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QPoint {
    pub p1: Vec2,
    pub p2: Vec2,
    pub p3: Vec2,
    pub p4: Vec2,
}

impl QPoint {
    /// 构造函数
    #[inline(always)]
    pub fn new(p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2) -> Self {
        Self { p1, p2, p3, p4 }
    }

    /// 从两个骈点 (DPoint) 创建 (通常这两个 DPoint 代表对角线方向的点对)
    /// 映射逻辑: dp1 -> (p1, p3), dp2 -> (p2, p4)
    pub fn from_2dp(dp1: DPoint, dp2: DPoint) -> Self {
        Self {
            p1: dp1.p1,
            p2: dp2.p1, // 注意这里的穿插顺序，构成了顺时针/逆时针或交叉顺序
            p3: dp1.p2,
            p4: dp2.p2,
        }
    }

    /// 提取第一组对角点 (p1, p3)
    #[inline]
    pub fn dp1(self) -> DPoint {
        DPoint::new(self.p1, self.p3)
    }

    /// 提取第二组对角点 (p2, p4)
    #[inline]
    pub fn dp2(self) -> DPoint {
        DPoint::new(self.p2, self.p4)
    }

    /// 对角线 1 (连接 p1, p3)
    #[inline]
    pub fn l1(self) -> Line { self.dp1().line() }

    /// 对角线 2 (连接 p2, p4)
    #[inline]
    pub fn l2(self) -> Line { self.dp2().line() }

    /// "心" - 对角线交点
    /// 完全四边形的三个对角点之一 (Diagonal Point)
    pub fn heart(self) -> Vec2 {
        line520::x_line_line(&self.l1(), &self.l2())
    }

    // ====================== 四边连线 ======================

    /// 边 1-2
    #[inline] pub fn l12(self) -> Line { Line::from_two_points(self.p1, self.p2) }
    /// 边 1-4
    #[inline] pub fn l14(self) -> Line { Line::from_two_points(self.p1, self.p4) }
    /// 边 3-2
    #[inline] pub fn l32(self) -> Line { Line::from_two_points(self.p3, self.p2) }
    /// 边 3-4
    #[inline] pub fn l34(self) -> Line { Line::from_two_points(self.p3, self.p4) }

    /// 叉线 1: 由对边 (1-4) 和 (3-2) 组成
    pub fn xl1(self) -> XLine {
        XLine::from_two_lines(self.l14(), self.l32())
    }

    /// 叉线 2: 由对边 (1-2) 和 (3-4) 组成
    pub fn xl2(self) -> XLine {
        XLine::from_two_lines(self.l12(), self.l34())
    }

    /// 索引点 (0-3)
    #[inline]
    pub fn index_point(self, i: usize) -> Vec2 {
        match i {
            0 => self.p1,
            1 => self.p2,
            2 => self.p3,
            _ => self.p4,
        }
    }

    // ====================== 射影几何性质 ======================

    // 衍骈点 (Derived DPoint) - 另外两个对角点
    // p_a = (1-4) ∩ (3-2)
    // p_b = (1-2) ∩ (3-4)
    pub fn derive_dp(self) -> DPoint {
        let p_a = line520::x_line_line(&self.l14(), &self.l32());
        let p_b = line520::x_line_line(&self.l12(), &self.l34());
        DPoint::new(p_a, p_b)
    }

    // 衍线 (连接另外两个对角点的直线)
    pub fn derive_l(self) -> Line {
        self.derive_dp().line()
    }

    // 骈叉线 (DXLine)
    // 对应 Dart 逻辑:
    // Vector deriveDP1 = l520.xLineLine(l14, l32);
    // Vector deriveDP2 = l520.xLineLine(l12, l34);
    // return DXLine(...)
    pub fn net(self) -> DXLine {
        // 1. 计算两个对角衍点
        // (1-4) ∩ (3-2)
        let derive_dp1 = line520::x_line_line(&self.l14(), &self.l32());
        // (1-2) ∩ (3-4)
        let derive_dp2 = line520::x_line_line(&self.l12(), &self.l34());

        DXLine::new(
            // 第一条叉线: 中心 derive_dp1, 方向指向 p1, p2
            XLine::new(derive_dp1, self.p1 - derive_dp1, self.p2 - derive_dp1),

            // 第二条叉线: 中心 derive_dp2
            // 注意：这里严格遵循 Dart 代码，方向向量依然基于 derive_dp1 计算
            // (p1 - derive_dp1) 和 (p4 - derive_dp1)
            XLine::new(derive_dp2, self.p1 - derive_dp1, self.p4 - derive_dp1),
        )
    }

    // 调和四点组计算
    // 在直线 dp 上，根据调和比 t 生成四个点
    pub fn harmonic(dp: DPoint, t: f64) -> Self {
        let l = dp.line();
        // 0, 1 是基准，t 是分比
        let qn = QNum::harmonic(DNum::new(0.0, 1.0), t);
        l.index_q_point(qn)
    }
}

impl std::fmt::Display for QPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QPoint(p1: {}, p2: {}, p3: {}, p4: {})", self.p1, self.p2, self.p3, self.p4)
    }
}