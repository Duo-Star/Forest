// src/math_forest/geometry/d2/intersection/line520.rs
#![allow(dead_code)]

// ====================== 导入依赖 ======================

// 几何基元
use crate::math_forest::geometry::d2::linear::vec2::Vec2;
use crate::math_forest::geometry::d2::linear::line::Line;

// 几何形状
use crate::math_forest::geometry::d2::conic::circle::Circle;
use crate::math_forest::geometry::d2::conic::ellipse::Ellipse;
use crate::math_forest::geometry::d2::conic::parabola::Parabola;
use crate::math_forest::geometry::d2::conic::hyperbola::Hyperbola;
use crate::math_forest::geometry::d2::conic::x_line::XLine;
use crate::math_forest::geometry::d2::conic::h_line::HLine;
use crate::math_forest::geometry::d2::conic::wipkyy::Wipkyy;

// 结果容器
use crate::math_forest::algebra::fertile::d_num::DNum;
use crate::math_forest::geometry::d2::fertile::d_point::DPoint;

// 代数求解器 (Solver)
// 假设这些求解器 API 是稳定的，且能处理数值误差
use crate::math_forest::algebra::solver::linear;
use crate::math_forest::algebra::solver::trigonometric;
use crate::math_forest::algebra::solver::polynomial;

// ====================== 核心求交逻辑 ======================

/// 两个直线求交点
/// 原理：克拉默法则求解 2x2 线性方程组
pub fn x_line_line(la: &Line, lb: &Line) -> Vec2 {
    // 方程: la.p + t1 * la.v = lb.p + t2 * lb.v
    // 移项: t1 * la.v - t2 * lb.v = lb.p - la.p
    // [ la.v.x  -lb.v.x ] [ t1 ] = [ dx ]
    // [ la.v.y  -lb.v.y ] [ t2 ] = [ dy ]

    let diff = lb.p - la.p;

    // 注意：linear::solve_linear_2x2 需要返回 (t1, t2)
    let (_t1, t2) = linear::solve_linear_2x2(
        la.v.x, -lb.v.x, diff.x,
        la.v.y, -lb.v.y, diff.y,
    );

    // 代回 lb 计算坐标
    lb.index_point(t2)
}

/// 计算直线与圆交点的参数 theta (优化版：叉积法)
/// 方程: (C + R(cos, sin) - P_l) x V_l = 0
/// 展开: R(cos * V_l.y - sin * V_l.x) + (C - P_l) x V_l = 0
/// 整理: (-R * V_l.y) cos + (R * V_l.x) sin = (P_l - C) x V_l
pub fn x_circle_line_theta(c: &Circle, l: &Line) -> DNum {
    // A cos + B sin = K
    let a = -c.r * l.v.y;
    let b = c.r * l.v.x;

    // 常数项移到等号右边: K = -((C - P_l) x V_l) = (P_l - C) x V_l
    let k = (l.p - c.p).cross(l.v);

    trigonometric::solve_cos_sin_for_main_root(a, b, k)
}

/// 直线与圆求交
pub fn x_circle_line(c: &Circle, l: &Line) -> DPoint {
    let theta = x_circle_line_theta(c, l);
    c.index_d_point(theta)
}

/// 直线与椭圆求交 (优化版：叉积法)
/// 方程: (U x V_l) cos + (V x V_l) sin + (C - P_l) x V_l = 0
/// 整理: (U x V_l) cos + (V x V_l) sin = (P_l - C) x V_l
pub fn x_ellipse_line(c: &Ellipse, l: &Line) -> DPoint {
    // 这种写法无需判断直线是否垂直，数值极度稳定
    let a = c.u.cross(l.v);         // cos 系数
    let b = c.v.cross(l.v);         // sin 系数
    let k = (l.p - c.p).cross(l.v); // 常数项 (移项后)

    let thetas = trigonometric::solve_cos_sin_for_main_root(a, b, k);
    c.index_d_point(thetas) // 注意：这里检查一下是否需要 &thetas，如果 DNum 是 Copy 直接传值
}

// ====================== 抛物线与双曲线 (保持你优秀的实现) ======================

/// 直线与抛物线求交
pub fn x_parabola_line(c: &Parabola, l: &Line) -> DPoint {
    // At^2 + Bt + K = 0
    let a = c.v.cross(l.v);         // t^2 系数
    let b = c.u().cross(l.v);       // t 系数 (注意 Parabola::u() 是方法)
    let k = (c.p - l.p).cross(l.v); // 常数项

    let t_dnum = polynomial::solve_real_quadratic_for_real(a, b, k);
    c.index_d_point(t_dnum)
}

/// 直线与双曲线求交
pub fn x_hyperbola_line(c: &Hyperbola, l: &Line) -> DPoint {
    let diff = c.p - l.p;

    // At^2 + Bt + C = 0 (原理: t^2(U x V_l) + t((P_c - P_l) x V_l) + (V x V_l) = 0)
    let a_coeff = c.u.cross(l.v);
    let b_coeff = diff.cross(l.v); // 注意这里的 diff 是 (C - P_l)
    let c_coeff = c.v.cross(l.v);

    let t_dnum = polynomial::solve_real_quadratic_for_real(
        a_coeff,
        b_coeff,
        c_coeff,
    );
    c.index_d_point(t_dnum)
}


/// 直线与叉线 (XLine) 求交
/// 结果为两个点（分别与两条渐近线/直线的交点）
pub fn x_x_line_line(c: &XLine, l: &Line) -> DPoint {
    DPoint::new(
        x_line_line(&c.l1(), l),
        x_line_line(&c.l2(), l),
    )
}

/// 直线与平行线 (HLine) 求交
pub fn x_h_line_line(c: &HLine, l: &Line) -> DPoint {
    DPoint::new(
        x_line_line(&c.l1(), l),
        x_line_line(&c.l2(), l),
    )
}

/// 直线与虚空 (Wipkyy) 求交
/// 也许这是"WIP" (Work In Progress) 曲线？暂时保留你的逻辑
pub fn x_wipkyy_line(_c: &Wipkyy, _l: &Line) -> DPoint {
    DPoint::INF
}
