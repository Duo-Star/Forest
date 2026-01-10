// src/math_forest/algebra/solver/trigonometric.rs
#![allow(dead_code)]

use std::f64::consts::PI;
use crate::math_forest::algebra::fertile::d_num::DNum;

const EPSILON: f64 = 1e-12;

/// 计算三角方程的主解: a * sin(w * t + p) + c = 0
/// => sin(w * t + p) = -c / a
#[inline]
pub fn solve_sin_for_main_root(a: f64, w: f64, p: f64, c: f64) -> DNum {
    // 避免除以零
    if a.abs() < EPSILON {
        return if c.abs() < EPSILON {
            DNum::new(0.0, PI) // 无穷解，返回两个周期内的典型值
        } else {
            DNum::NAN
        };
    }

    let ratio = -c / a;

    // 无实数解 (-1 <= sin <= 1)
    if ratio.abs() > 1.0 {
        return DNum::NAN;
    }

    let u = ratio.asin();

    // sin(X) = u 的两个主解: X1 = u, X2 = PI - u
    // w*t + p = u       => t = (u - p) / w
    // w*t + p = PI - u  => t = (PI - u - p) / w

    // 考虑到 w 可能为负，通常几何中 w=1，这里直接除即可
    let inv_w = 1.0 / w;
    DNum::new((u - p) * inv_w, (PI - u - p) * inv_w)
}

/// 计算三角方程的主解: u * cos(t) + v * sin(t) + c = 0
/// 优化：使用 atan2 合并辅助角公式，无需手动处理象限和分母为0的情况
pub fn solve_cos_sin_for_main_root(u: f64, v: f64, c: f64) -> DNum {
    // 合成公式: u*cos(t) + v*sin(t) = R * sin(t + phi)
    // 其中 R = sqrt(u^2 + v^2), phi = atan2(u, v)
    // 推导: R(sin t cos phi + cos t sin phi) = (R cos phi) sin t + (R sin phi) cos t
    // 对比系数: v = R cos phi, u = R sin phi => tan phi = u/v

    let r = u.hypot(v); // 稳健的 sqrt(u*u + v*v)

    if r < EPSILON {
        // 退化为 c = 0
        return if c.abs() < EPSILON {
            DNum::new(0.0, PI)
        } else {
            DNum::NAN
        };
    }

    let phi = u.atan2(v); // 注意参数顺序: atan2(y, x) -> atan2(coeff_of_cos, coeff_of_sin)

    solve_sin_for_main_root(r, 1.0, phi, c)
}