
/// 求解器配置参数
#[derive(Debug, Clone)]
pub struct NewtonSolver {
    pub max_iter: usize,
    pub tolerance: f64,
    pub step_size_deriv: f64, // 用于数值求导的步长
}

impl NewtonSolver {
    pub fn new() -> Self {
        Self {
            max_iter: 100,
            tolerance: 1e-9,
            step_size_deriv: 1e-5,
        }
    }

    /// 通用牛顿迭代求解器
    ///
    /// * `start_guess`: 初始猜测值
    /// * `func`: 目标方程 f(t) = 0 (这里是距离函数的导数)
    /// * `range`: (min, max) 搜索范围，用于处理周期性
    pub fn solve<F>(&self, start_guess: f64, func: F, range: (f64, f64)) -> Option<f64>
    where
        F: Fn(f64) -> f64,
    {
        let mut t = start_guess;
        let (min_t, max_t) = range;
        let period = max_t - min_t;

        for _ in 0..self.max_iter {
            // 确保 t 在范围内 (处理周期性)
            while t < min_t { t += period; }
            while t >= max_t { t -= period; }

            let y = func(t);

            // 检查收敛
            if y.abs() < self.tolerance {
                return Some(t);
            }

            // 数值求导 f'(t) ≈ (f(t+h) - f(t-h)) / 2h
            // 注意：这里的 f 是距离的一阶导，所以我们算的是距离的二阶导
            let h = self.step_size_deriv;
            let y_plus = func(t + h);
            let y_minus = func(t - h);
            let derivative = (y_plus - y_minus) / (2.0 * h);

            // 防止导数为0导致除以0
            if derivative.abs() < 1e-14 {
                break;
            }

            let step = y / derivative;

            // 阻尼牛顿法（可选）：限制步长防止飞出太远
            let clamped_step = step.max(-1.0).min(1.0);

            t -= clamped_step;
        }

        None // 未收敛
    }
}
