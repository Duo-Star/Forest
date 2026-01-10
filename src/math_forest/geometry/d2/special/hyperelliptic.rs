use std::f64::consts::PI;
use super::super::super::super::algebra::solver::nt::NewtonSolver;
use super::super::linear::vec2::Vec2;

pub struct Hyperelliptic {
    pub a: f64,
    pub b: f64,
    pub m: f64,
}

impl Hyperelliptic {
    // t 索引 (x, y)
    // signum * |cos|^m 处理符号
    pub fn point_at(&self, t: f64) -> Vec2 {
        let cos_t = t.cos();
        let sin_t = t.sin();

        // x = a * sgn(cos t) * |cos t|^m
        let x = self.a * cos_t.signum() * cos_t.abs().powf(self.m);
        // y = b * sgn(sin t) * |sin t|^m
        let y = self.b * sin_t.signum() * sin_t.abs().powf(self.m);

        Vec2{x, y}
    }

    pub fn implicit(&self, x:f64, y:f64) -> f64{
        (x/self.a).abs().powf(2.0/self.m) + (y/self.b).abs().powf(2.0/self.m) - 1.0
    }

    /// 计算曲线的一阶导数 (dx/dt, dy/dt)
    pub fn derivative_at(&self, t: f64) -> Vec2 {
        let cos_t = t.cos();
        let sin_t = t.sin();
        let m = self.m;

        // dx/dt = -a * m * |cos t|^(m-1) * sin t
        // 注意：符号分析后简化为 -a * m * |cos t|^(m-1) * sin t
        let dx = -self.a * m * cos_t.abs().powf(m - 1.0) * sin_t;

        // dy/dt = b * m * |sin t|^(m-1) * cos t
        let dy = self.b * m * sin_t.abs().powf(m - 1.0) * cos_t;

        Vec2{ x: dx, y: dy }
    }

    /// 寻找曲线上距离目标点 (px, py) 最近的点
    /// 返回 (最短距离, 最优参数 t, 曲线上的点 x, 曲线上的点 y)
    pub fn find_closest_point(&self, p:Vec2) -> (f64, f64, Vec2) {
        let solver = NewtonSolver::new();

        // 目标方程：D^2(t) 的导数 = 0
        // g(t) = (x(t) - px) * x'(t) + (y(t) - py) * y'(t)
        let objective_deriv = |t: f64| -> f64 {
            let tp = self.point_at(t);
            let dp = self.derivative_at(t);
            (tp-p).dot(dp)
        };

        // 多点启动策略：从 0, PI/2, PI, 3PI/2 四个位置开始搜索
        // 这是因为距离函数可能有多个极值（例如长轴和短轴的端点）
        let start_points = [0.0, PI / 2.0, PI, 3.0 * PI / 2.0];

        let mut best_dist_sq = f64::INFINITY;
        let mut best_t = 0.0;
        let mut best_pt = Vec2::new(0.0, 0.0);

        for &start_t in &start_points {
            if let Some(t_sol) = solver.solve(start_t, objective_deriv, (0.0, 2.0 * PI)) {
                let b = self.point_at(t_sol);
                let dist_sq= (b-p).pow2();
                // 检查是否是更优解
                if dist_sq < best_dist_sq {
                    best_dist_sq = dist_sq;
                    best_t = t_sol;
                    best_pt = b;
                }
            }
        }

        // 规范化最终的 t 到 [0, 2PI)
        let final_t = best_t.rem_euclid(2.0 * PI);
        (best_dist_sq.sqrt(), final_t, best_pt)
    }
}


