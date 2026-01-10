// src/math_forest/geometry/d2/conic/conic.rs
#![allow(dead_code)]

use std::fmt;
use crate::math_forest::algebra::solver::linear::{det4x4, solve_linear_2x2};
use crate::math_forest::algebra::solver::polynomial::solve_real_quadratic_for_real;
use crate::math_forest::geometry::d2::linear::vec2::Vec2;
use crate::math_forest::geometry::d2::linear::line::Line;
use crate::math_forest::geometry::d2::conic::h_line::HLine;
use crate::math_forest::geometry::d2::conic::x_line::XLine;
use crate::math_forest::geometry::d2::conic::circle::Circle;
use crate::math_forest::geometry::d2::conic::ellipse::Ellipse;
use crate::math_forest::geometry::d2::conic::hyperbola::Hyperbola;
use crate::math_forest::geometry::d2::conic::parabola::Parabola;
use crate::math_forest::geometry::d2::fertile::d_point::DPoint;

/// 圆锥曲线类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConicType {
    Circle,             // 圆
    Ellipse,            // 椭圆
    Hyperbola,          // 双曲线
    Parabola,           // 抛物线
    RectangularHyperbola, // 等轴双曲线 (特殊双曲线)

    // 退化类型 (Degenerate)
    Point,              // 单点 (虚椭圆)
    Line,               // 单直线 (退化抛物线)
    ParallelLines,      // 平行双直线 (退化抛物线)
    IntersectingLines,  // 交叉双直线 (退化双曲线 - XLine)
    Imaginary,          // 虚空 (无实数解)
}

/// 通用圆锥曲线方程: Ax^2 + Bxy + Cy^2 + Dx + Ey + F = 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Conic {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Conic {
    pub const EPSILON: f64 = 1e-9;

    /// 基础构造
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        Self { a, b, c, d, e, f }
    }

    /// 从五个点创建圆锥曲线
    /// 原理：系数对应于 5x6 矩阵的 5x5 子行列式
    /// 矩阵行: [x^2, xy, y^2, x, y, 1]
    pub fn from_five_points(p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2, p5: Vec2) -> Self {
        // 构建 5x6 矩阵的数据 (每一行是一个点的数据)
        // r_i = [x^2, xy, y^2, x, y, 1]
        let points = [p1, p2, p3, p4, p5];

        let row = |i: usize| -> [f64; 6] {
            let p = points[i];
            [p.x * p.x, p.x * p.y, p.y * p.y, p.x, p.y, 1.0]
        };

        // 辅助函数：提取某一列被剔除后的 5x5 矩阵行列式
        // cols indices: 0:A(x^2), 1:B(xy), 2:C(y^2), 3:D(x), 4:E(y), 5:F(1)
        let get_cofactor = |skip_col: usize| -> f64 {
            let mut mat = [0.0; 25]; // 5x5
            for r in 0..5 {
                let data = row(r);
                let mut c_idx = 0;
                for c in 0..6 {
                    if c == skip_col { continue; }
                    mat[r * 5 + c_idx] = data[c];
                    c_idx += 1;
                }
            }
            det5x5(&mat)
        };

        // 根据 Cramer 法则推广，系数是代数余子式
        // 注意符号交替: + - + - + -
        let a =  get_cofactor(0);
        let b = -get_cofactor(1);
        let c =  get_cofactor(2);
        let d = -get_cofactor(3);
        let e =  get_cofactor(4);
        let f =  get_cofactor(5);

        Self::new(a, b, c, d, e, f)
    }

    // ====================== 属性计算 ======================

    /// 判别式 Delta = B^2 - 4AC
    pub fn discriminant(&self) -> f64 {
        self.b * self.b - 4.0 * self.a * self.c
    }

    /// 3x3 矩阵行列式 (用于判断退化)
    /// [ 2A  B  D ]
    /// [ B  2C  E ]
    /// [ D  E  2F ]
    /// 注意：通常教材用 [A B/2 D/2]... 为了避免除法，我们计算 8 * det
    pub fn det_3x3_scaled(&self) -> f64 {
        let (a, b, c) = (2.0*self.a, self.b, 2.0*self.c);
        let (d, e, f) = (self.d, self.e, 2.0*self.f);

        a * (c * f - e * e) - b * (b * f - e * d) + d * (b * e - c * d)
    }

    /// 获取圆锥曲线类型
    pub fn get_conic_type(&self) -> ConicType {
        let delta = self.discriminant();
        let det = self.det_3x3_scaled();
        let is_degenerate = det.abs() < 1.0; // 阈值需要根据系数大小调整，这里简化

        if is_degenerate {
            if delta < -Self::EPSILON { return ConicType::Point; }
            if delta > Self::EPSILON { return ConicType::IntersectingLines; }
            // delta == 0
            // 进一步区分平行线还是单线，需检查子行列式，这里简化
            return ConicType::ParallelLines;
        }

        if delta < -Self::EPSILON {
            if (self.a - self.c).abs() < Self::EPSILON && self.b.abs() < Self::EPSILON {
                return ConicType::Circle;
            }
            ConicType::Ellipse
        } else if delta > Self::EPSILON {
            if (self.a + self.c).abs() < Self::EPSILON {
                return ConicType::RectangularHyperbola;
            }
            return ConicType::Hyperbola;
        } else {
            return ConicType::Parabola;
        }
    }

    /// 计算中心点
    /// 椭圆/双曲线/圆有唯一中心。抛物线无中心（返回 INF）。
    pub fn center(&self) -> Vec2 {
        // 求解方程组:
        // 2Ax + By + D = 0
        // Bx + 2Cy + E = 0
        let (x, y) = solve_linear_2x2(
            2.0 * self.a, self.b, -self.d,
            self.b, 2.0 * self.c, -self.e
        );

        if x.is_nan() { Vec2::INF } else { Vec2::new(x, y) }
    }

    /// 获取旋转角 theta (使得旋转后 B' = 0)
    /// tan(2theta) = B / (A - C)
    pub fn rotation_angle(&self) -> f64 {
        if self.b.abs() < Self::EPSILON {
            0.0
        } else {
            // atan2(y, x) -> atan2(B, A-C) = 2theta
            0.5 * self.b.atan2(self.a - self.c)
        }
    }

    // ====================== 几何对象转换 ======================

    /// 转换为特定的几何对象（如果是该类型）

    pub fn to_circle(&self) -> Option<Circle> {
        if self.get_conic_type() != ConicType::Circle { return None; }
        let center = self.center();
        // r^2 = (D^2 + E^2 - 4AF) / (4A^2)  (对于 A=C, B=0)
        // 或者简单代入中心求 F'
        // F' = A*h^2 + C*k^2 + D*h + E*k + F
        // r^2 = -F' / A
        let val_at_center = self.eval(center);
        let r2 = -val_at_center / self.a;
        if r2 < 0.0 { return None; }
        Some(Circle::new(center, r2.sqrt()))
    }

    // 将通用方程转换为标准椭圆
    // 需要极其复杂的代数变换：旋转 -> 平移 -> 提取 a, b
    pub fn to_ellipse(&self) -> Option<Ellipse> {
        if let ConicType::Ellipse | ConicType::Circle = self.get_conic_type() {} else { return None; }

        let center = self.center();
        let theta = self.rotation_angle();
        let (sin, cos) = theta.sin_cos();

        // 旋转系数变换公式 (不变量 A+C, B^2-4AC)
        // A' = A cos^2 + B sin cos + C sin^2
        // C' = A sin^2 - B sin cos + C cos^2
        let a_prime = self.a * cos * cos + self.b * sin * cos + self.c * sin * sin;
        let c_prime = self.a * sin * sin - self.b * sin * cos + self.c * cos * cos;

        // 平移后的常数项 F' = val_at_center
        let f_prime = self.eval(center);

        // 标准方程: A' x^2 + C' y^2 + F' = 0 => x^2 / (-F'/A') + y^2 / (-F'/C') = 1
        let a2 = -f_prime / a_prime;
        let b2 = -f_prime / c_prime;

        if a2 <= 0.0 || b2 <= 0.0 { return None; } // 虚椭圆

        // 构造 Ellipse 对象
        // U 对应长轴方向（旋转 theta），模长 sqrt(a2) 或 sqrt(b2)
        // 需要判断哪个大
        let u_len = a2.sqrt();
        let v_len = b2.sqrt();

        // 旋转向量
        let u_vec = Vec2::new(cos, sin);
        let v_vec = Vec2::new(-sin, cos);

        Some(Ellipse::new(center, u_vec * u_len, v_vec * v_len))
    }

    pub fn to_hyperbola(&self) -> Option<Hyperbola> {
        // 逻辑类似于椭圆，只是 a2, b2 异号
        if self.get_conic_type() != ConicType::Hyperbola { return None; }

        let center = self.center();
        let theta = self.rotation_angle();
        let (sin, cos) = theta.sin_cos();

        let a_prime = self.a * cos * cos + self.b * sin * cos + self.c * sin * sin;
        let c_prime = self.a * sin * sin - self.b * sin * cos + self.c * cos * cos;
        let f_prime = self.eval(center);

        // A' x^2 + C' y^2 = -F'
        let term_x = -f_prime / a_prime;
        let term_y = -f_prime / c_prime;

        // 双曲线中 term_x, term_y 一正一负
        // 我们需要构造 Hyperbola(center, u, v)
        // u 对应实轴, v 对应虚轴(渐近线方向组合)

        // 这是一个复杂的转换，因为我们的 Hyperbola 定义是基于渐近线的 P + tU + V/t
        // 而这里得到的是标准方程 x^2/a^2 - y^2/b^2 = 1
        // 暂时留空或仅返回标准轴双曲线
        None // 待实现：标准型转渐近线型
    }

    // ====================== 通用几何计算 ======================

    /// 计算点 p 处的值
    pub fn eval(&self, p: Vec2) -> f64 {
        self.a * p.x * p.x + self.b * p.x * p.y + self.c * p.y * p.y
            + self.d * p.x + self.e * p.y + self.f
    }

    /// 极点-极线 (Pole-Polar) 关系
    /// 给定点 P(x0, y0)，返回极线 L: (2Ax0 + By0 + D)x + (Bx0 + 2Cy0 + E)y + (Dx0 + Ey0 + 2F) = 0
    /// 注意系数需要除以 2 还原
    pub fn polar_line(&self, p: Vec2) -> Line {
        // 直线方程 Ax + By + C = 0 -> 法向量 (A, B), 基点 ?
        // Line 定义是 (Point, Direction)
        let la = 2.0 * self.a * p.x + self.b * p.y + self.d; // x 系数 * 2
        let lb = self.b * p.x + 2.0 * self.c * p.y + self.e; // y 系数 * 2
        let lc = self.d * p.x + self.e * p.y + 2.0 * self.f; // 常数 * 2

        // 实际方程: (la/2) x + (lb/2) y + (lc/2) = 0
        let normal = Vec2::new(la, lb); // 法向量
        if normal.len() < Self::EPSILON {
            // 极点在中心，极线在无穷远
            return Line::new(Vec2::NAN, Vec2::NAN);
        }

        // 构造直线：找到直线上一点
        // 设 x=0, y = -lc/lb (如果 lb!=0)
        let p_on_line = if lb.abs() > Self::EPSILON {
            Vec2::new(0.0, -lc / lb)
        } else {
            Vec2::new(-lc / la, 0.0)
        };

        // 方向向量是法向量旋转 90 度
        let dir = normal.roll90();
        Line::new(p_on_line, dir)
    }
}

impl fmt::Display for Conic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Conic({:.2}x² + {:.2}xy + {:.2}y² + {:.2}x + {:.2}y + {:.2} = 0)",
               self.a, self.b, self.c, self.d, self.e, self.f)
    }
}

// ====================== 辅助函数 ======================

/// 计算 5x5 行列式 (用于五点共锥)
/// 这是一个递归实现，性能一般但够用
fn det5x5(m: &[f64; 25]) -> f64 {
    // 展开第一行
    let mut det = 0.0;
    let mut sign = 1.0;

    for c in 0..5 {
        if m[c].abs() > 1e-12 {
            let sub = get_sub_matrix_4x4(m, 0, c);
            det += sign * m[c] * det4x4_array(&sub);
        }
        sign = -sign;
    }
    det
}

// 辅助：从 5x5 提取 4x4
fn get_sub_matrix_4x4(m: &[f64; 25], skip_row: usize, skip_col: usize) -> [f64; 16] {
    let mut res = [0.0; 16];
    let mut idx = 0;
    for r in 0..5 {
        if r == skip_row { continue; }
        for c in 0..5 {
            if c == skip_col { continue; }
            res[idx] = m[r * 5 + c];
            idx += 1;
        }
    }
    res
}

// 辅助：数组版 det4x4 (复用 solver 的逻辑)
fn det4x4_array(m: &[f64; 16]) -> f64 {
    det4x4(
        m[0], m[1], m[2], m[3],
        m[4], m[5], m[6], m[7],
        m[8], m[9], m[10], m[11],
        m[12], m[13], m[14], m[15]
    )
}