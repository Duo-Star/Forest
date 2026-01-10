// src/math_forest/algebra/solver/polynomial.rs
#![allow(dead_code)]

// 依赖导入
use crate::math_forest::algebra::complex::complex::Complex;
use crate::math_forest::algebra::complex::d_complex::DComplex;
use crate::math_forest::algebra::complex::t_complex::TComplex;
use crate::math_forest::algebra::complex::q_complex::QComplex;
use crate::math_forest::algebra::fertile::d_num::DNum;

// ====================== 二次方程 (Quadratic) ======================

/// 实系数二次方程: a*x^2 + b*x + c = 0 -> 实数解 (DNum)
/// 如果判别式 < 0，返回 NaN
#[inline]
pub fn solve_real_quadratic_for_real(a: f64, b: f64, c: f64) -> DNum {
    // 鲁棒性：如果 a 极小，退化为线性方程 bx + c = 0
    if a.abs() < 1e-12 {
        if b.abs() < 1e-12 {
            return DNum::NAN; // 无解或无穷多解
        }
        let x = -c / b;
        return DNum::new(x, x);
    }

    let delta = b * b - 4.0 * a * c;
    if delta < 0.0 {
        return DNum::NAN;
    }

    let sqrt_delta = delta.sqrt();
    let inv_2a = 1.0 / (2.0 * a);

    // 经典公式
    let n1 = (-b + sqrt_delta) * inv_2a;
    let n2 = (-b - sqrt_delta) * inv_2a;
    DNum::new(n1, n2)
}

/// 实系数二次方程 -> 复数解 (DComplex)
#[inline]
pub fn solve_real_quadratic_for_complex(a: f64, b: f64, c: f64) -> DComplex {
    if a.abs() < 1e-12 {
        if b.abs() < 1e-12 { return DComplex { n1: Complex::NAN, n2: Complex::NAN }; }
        let x = Complex::from_real(-c / b);
        return DComplex::new(x, x);
    }

    let delta = b * b - 4.0 * a * c;
    let inv_2a = 1.0 / (2.0 * a);

    if delta >= 0.0 {
        let sqrt_delta = delta.sqrt();
        let n1 = (-b + sqrt_delta) * inv_2a;
        let n2 = (-b - sqrt_delta) * inv_2a;
        DComplex::from_real(n1, n2)
    } else {
        // delta < 0, 产生共轭复根
        let sqrt_delta_im = (-delta).sqrt();
        let re = -b * inv_2a;
        let im = sqrt_delta_im * inv_2a;
        DComplex::new(
            Complex::new(re, im),
            Complex::new(re, -im)
        )
    }
}

/// 复系数二次方程: a*x^2 + b*x + c = 0 -> 复数解 (DComplex)
/// [API优化] 改为传值
pub fn solve_complex_quadratic_for_complex(a: Complex, b: Complex, c: Complex) -> DComplex {
    if a.is_zero() {
        if b.is_zero() { return DComplex { n1: Complex::NAN, n2: Complex::NAN }; }
        let x = -c / b;
        return DComplex::new(x, x);
    }

    let delta = b * b - a * c * 4.0;
    let sqrt_delta = delta.sqrt();
    let inv_2a = Complex::ONE / (a * 2.0);

    let n1 = (-b + sqrt_delta) * inv_2a;
    let n2 = (-b - sqrt_delta) * inv_2a;
    DComplex::new(n1, n2)
}

// ====================== 三次方程 (Cubic) ======================

/// 求解一元三次方程: a*x^3 + b*x^2 + c*x + d = 0
/// [API优化] 改为传值
pub fn solve_cubic(a: Complex, b: Complex, c: Complex, d: Complex) -> TComplex {
    // 降阶处理
    if a.is_zero() {
        if b.is_zero() {
            // 线性方程 cx + d = 0
            if c.is_zero() { return TComplex::NAN; }
            return TComplex::all(-d / c);
        }
        // 二次方程
        let qr = solve_complex_quadratic_for_complex(b, c, d);
        return TComplex::new(qr.n1, qr.n2, qr.n2);
    }

    // 归一化: x^3 + px^2 + qx + r = 0
    let p = b / a;
    let q = c / a;
    let r = d / a;

    // 转化为压所三次方程 (Depressed Cubic): t^3 + mt + n = 0
    // 变换: x = t - p/3
    let p_div_3 = p / 3.0;
    let m = q - p * p_div_3;
    let n = p_div_3 * (q * 2.0 - p * p_div_3 * 2.0) - r; // 简化后的常数项计算
    // 原式: (2 p^3)/27 - (p q)/3 + r => 这里的 n 是负的原常数项吗？
    // 让我们用标准公式: y^3 + py + q = 0 (这里的 p,q 指 depressed 的参数)
    // depressed_p = (3ac - b^2) / 3a^2 = q - p^2/3. (你的 m) matches.
    // depressed_q = (2b^3 - 9abc + 27a^2d) / 27a^3.
    // 你的 n 计算: (2p^3 - 9pq + 27r) / 27. Matches.

    // 判别式 Delta = (n/2)^2 + (m/3)^3
    let delta = (n / 2.0).powf(2.0) + (m / 3.0).powf(3.0);
    let sqrt_delta = delta.sqrt();

    // Cardano 公式
    // u = (-n/2 + sqrt_delta)^(1/3)
    let u = (-n / 2.0 + sqrt_delta).powf(1.0 / 3.0);

    // [重要修正] v 不能直接由 (-n/2 - sqrt_delta) 开根号得到，
    // 因为必须满足 u * v = -m/3 的约束。
    // 如果直接开根号，可能会得到错误的相位组合。
    let v = if u.is_zero() {
        Complex::ZERO
    } else {
        -m / (u * 3.0)
    };

    let y1 = u + v;

    // 单位根
    let omega = Complex::new(-0.5, 3.0_f64.sqrt() / 2.0); // e^(i 2pi/3)
    let omega2 = omega.conj(); // e^(-i 2pi/3)

    let y2 = omega * u + omega2 * v;
    let y3 = omega2 * u + omega * v;

    let shift = p_div_3;
    TComplex::new(y1 - shift, y2 - shift, y3 - shift)
}

// ====================== 四次方程 (Quartic) ======================

/// 求解一元四次方程: a*x^4 + b*x^3 + c*x^2 + d*x + e = 0
/// [API优化] 改为传值
pub fn solve_quartic(a: Complex, b: Complex, c: Complex, d: Complex, e: Complex) -> QComplex {
    // 降阶
    if a.is_zero() {
        let cr = solve_cubic(b, c, d, e);
        return QComplex::new(cr.n1, cr.n2, cr.n3, cr.n3); // 补齐4个
    }

    let p = b / a;
    let q = c / a;
    let r = d / a;
    let s = e / a;

    // 转化为压缩四次方程 y^4 + Ay^2 + By + C = 0
    // x = y - p/4
    let p_div_4 = p / 4.0;
    let p2 = p_div_4 * p_div_4;

    let aa = q - p2 * 6.0; // A
    let bb = p2 * p_div_4 * 8.0 - p_div_4 * q * 2.0 + r; // B
    let cc = p2 * p2 * -3.0 + p2 * q - p_div_4 * r + s; // C

    // 如果 B 是 0，这就是一个双二次方程 (Biquadratic)
    if bb.is_zero() {
        // y^4 + Ay^2 + C = 0 -> Let z = y^2
        let z_roots = solve_complex_quadratic_for_complex(Complex::ONE, aa, cc);
        let y1 = z_roots.n1.sqrt();
        let y2 = -y1;
        let y3 = z_roots.n2.sqrt();
        let y4 = -y3;
        return QComplex::new(y1 - p_div_4, y2 - p_div_4, y3 - p_div_4, y4 - p_div_4);
    }

    // 费拉里方法 (Ferrari's method)
    // 构造预解三次方程 (Resolvent Cubic): m^3 + A m^2 + (A^2 - 4C)/4 m - B^2/8 = 0
    // 或者使用: u^3 - (q)u^2 + (pr - 4s)u - (4qs - r^2 - p^2s) = 0 (对于原方程)
    // 这里使用基于压缩方程的形式: 8m^3 + 8Am^2 + (2A^2 - 8C)m - B^2 = 0

    let cubic_a = Complex::from_real(8.0);
    let cubic_b = aa * 8.0;
    let cubic_c = aa * aa * 2.0 - cc * 8.0;
    let cubic_d = -(bb * bb);

    let cubic_roots = solve_cubic(cubic_a, cubic_b, cubic_c, cubic_d);
    let m = cubic_roots.n1; // 取任意一个非零实根通常更好，这里取 n1

    // 求解两个二次方程
    // y^2 + sqrt(2m) y + (m + A/2 + B/sqrt(8m)) = 0
    // y^2 - sqrt(2m) y + (m + A/2 - B/sqrt(8m)) = 0

    let sqrt_2m = (m * 2.0).sqrt();

    // [稳定性修正] 避免除以零
    let term_b_div = if sqrt_2m.is_zero() {
        Complex::ZERO
    } else {
        bb / (sqrt_2m * 2.0) // 对应公式里的 B/sqrt(8m)
    };

    let term_common = m + aa / 2.0;

    let q1_c = term_common - term_b_div;
    let q2_c = term_common + term_b_div;

    // 解两个二次方程
    // 1. y^2 + sqrt(2m) y + q1_c = 0
    let roots1 = solve_complex_quadratic_for_complex(Complex::ONE, sqrt_2m, q1_c);
    // 2. y^2 - sqrt(2m) y + q2_c = 0
    let roots2 = solve_complex_quadratic_for_complex(Complex::ONE, -sqrt_2m, q2_c);

    QComplex::new(
        roots1.n1 - p_div_4,
        roots1.n2 - p_div_4,
        roots2.n1 - p_div_4,
        roots2.n2 - p_div_4,
    )
}