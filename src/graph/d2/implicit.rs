// src/implicit.rs
use rayon::prelude::*;
use crate::graph::d2::common::Vertex; // 导入公共顶点结构

pub struct ImplicitSolver {}

impl ImplicitSolver {
    pub fn new() -> Self { Self {} }

    pub fn solve<F>(&self, f: &F, x_range: (f64, f64), y_range: (f64, f64), screen_w: u32, screen_h: u32) -> Vec<Vertex>
    where
        F: Fn(f64, f64) -> f64 + Sync,
    {
        // 性能限制：限制网格最大分辨率为 700x700
        let limit = 700;
        let grid_w = (screen_w as usize / 2).clamp(100, limit);
        let grid_h = (screen_h as usize / 2).clamp(100, limit);

        let x_step = (x_range.1 - x_range.0) / grid_w as f64;
        let y_step = (y_range.1 - y_range.0) / grid_h as f64;

        (0..grid_w).into_par_iter().flat_map(|i| {
            let mut local_pts = Vec::with_capacity(16);
            let x = x_range.0 + i as f64 * x_step;
            for j in 0..grid_h {
                let y = y_range.0 + j as f64 * y_step;
                let v00 = f(x, y);
                let v10 = f(x + x_step, y);
                let v01 = f(x, y + y_step);

                if v00 * v10 <= 0.0 {
                    let t = self.linear_interp(v00, v10);
                    local_pts.push(Vertex { position: [(x + t * x_step) as f32, y as f32] });
                }
                if v00 * v01 <= 0.0 {
                    let t = self.linear_interp(v00, v01);
                    local_pts.push(Vertex { position: [x as f32, (y + t * y_step) as f32] });
                }
            }
            local_pts
        }).collect()
    }

    fn linear_interp(&self, v0: f64, v1: f64) -> f64 {
        let diff = v1 - v0;
        if diff.abs() < 1e-15 { return 0.5; }
        (-v0 / diff).clamp(0.0, 1.0)
    }
}