// src/math_forest/algebra/solver/linear.rs
#![allow(dead_code)]

const EPSILON: f64 = 1e-12;

#[inline]
pub fn det2x2(a1: f64, b1: f64, a2: f64, b2: f64) -> f64 {
    a1 * b2 - a2 * b1
}

#[inline]
pub fn det3x3(
    a1: f64, b1: f64, c1: f64,
    a2: f64, b2: f64, c2: f64,
    a3: f64, b3: f64, c3: f64,
) -> f64 {
    a1 * (b2 * c3 - b3 * c2) - b1 * (a2 * c3 - a3 * c2) + c1 * (a2 * b3 - a3 * b2)
}

#[inline]
#[allow(clippy::too_many_arguments)]
pub fn det4x4(
    m00: f64, m01: f64, m02: f64, m03: f64,
    m10: f64, m11: f64, m12: f64, m13: f64,
    m20: f64, m21: f64, m22: f64, m23: f64,
    m30: f64, m31: f64, m32: f64, m33: f64,
) -> f64 {
    // 2x2 minor determinants for the first two columns
    let s0 = m00 * m11 - m01 * m10;
    let s1 = m00 * m12 - m02 * m10;
    let s2 = m00 * m13 - m03 * m10;
    let s3 = m01 * m12 - m02 * m11;
    let s4 = m01 * m13 - m03 * m11;
    let s5 = m02 * m13 - m03 * m12;

    // 2x2 minor determinants for the last two columns
    let c5 = m22 * m33 - m23 * m32;
    let c4 = m21 * m33 - m23 * m31;
    let c3 = m21 * m32 - m22 * m31;
    let c2 = m20 * m33 - m23 * m30;
    let c1 = m20 * m32 - m22 * m30;
    let c0 = m20 * m31 - m21 * m30;

    // Expansion
    s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0
}

// 求解二元一次线性方程组：
// a1*x + b1*y = c1
// a2*x + b2*y = c2
// 返回 (x, y)
#[inline]
pub fn solve_linear_2x2(a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> (f64, f64) {
    let d = a1 * b2 - a2 * b1;
    // 使用 EPSILON 避免浮点误差
    if d.abs() < EPSILON {
        (f64::NAN, f64::NAN)
    } else {
        let inv_d = 1.0 / d; // 乘法比除法快
        let x = (c1 * b2 - c2 * b1) * inv_d;
        let y = (a1 * c2 - a2 * c1) * inv_d;
        (x, y)
    }
}

// 求解三元一次线性方程组：Ax = D
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn solve_linear_3x3(
    a1: f64, b1: f64, c1: f64, d1: f64,
    a2: f64, b2: f64, c2: f64, d2: f64,
    a3: f64, b3: f64, c3: f64, d3: f64,
) -> (f64, f64, f64) {
    let det = det3x3(a1, b1, c1, a2, b2, c2, a3, b3, c3);

    if det.abs() < EPSILON {
        (f64::NAN, f64::NAN, f64::NAN)
    } else {
        let inv_det = 1.0 / det;

        let det_x = det3x3(d1, b1, c1, d2, b2, c2, d3, b3, c3);
        let det_y = det3x3(a1, d1, c1, a2, d2, c2, a3, d3, c3);
        let det_z = det3x3(a1, b1, d1, a2, b2, d2, a3, b3, d3);

        (det_x * inv_det, det_y * inv_det, det_z * inv_det)
    }
}

// 求解四元一次线性方程组
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn solve_linear_4x4(
    a1: f64, b1: f64, c1: f64, d1: f64, e1: f64, // Row 1: ... = e1
    a2: f64, b2: f64, c2: f64, d2: f64, e2: f64,
    a3: f64, b3: f64, c3: f64, d3: f64, e3: f64,
    a4: f64, b4: f64, c4: f64, d4: f64, e4: f64,
) -> (f64, f64, f64, f64) {
    let det = det4x4(
        a1, b1, c1, d1,
        a2, b2, c2, d2,
        a3, b3, c3, d3,
        a4, b4, c4, d4,
    );

    if det.abs() < EPSILON {
        return (f64::NAN, f64::NAN, f64::NAN, f64::NAN);
    }

    let inv_det = 1.0 / det;

    let dx = det4x4(e1, b1, c1, d1, e2, b2, c2, d2, e3, b3, c3, d3, e4, b4, c4, d4);
    let dy = det4x4(a1, e1, c1, d1, a2, e2, c2, d2, a3, e3, c3, d3, a4, e4, c4, d4);
    let dz = det4x4(a1, b1, e1, d1, a2, b2, e2, d2, a3, b3, e3, d3, a4, b4, e4, d4);
    let dw = det4x4(a1, b1, c1, e1, a2, b2, c2, e2, a3, b3, c3, e3, a4, b4, c4, e4);

    (dx * inv_det, dy * inv_det, dz * inv_det, dw * inv_det)
}