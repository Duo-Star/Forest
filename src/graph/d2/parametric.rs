use rayon::prelude::*;
use crate::graph::d2::common::Vertex;

const SAMPLES_PER_UNIT_T: f64 = 20.0;

// ★ 新增：断裂阈值系数
// 如果两点之间的屏幕距离超过了屏幕高度的 2 倍，就认为是断点/渐近线，不连线。
const JUMP_THRESHOLD_FACTOR: f32 = 2.0;

pub struct ParametricSolver {}

impl ParametricSolver {
    pub fn new() -> Self { Self {} }

    pub fn solve<F>(
        &self,
        f: &F,
        t_range: (f64, f64),
        width_px: f32,
        zoom: f32,
        aspect: f32,
        screen_h: f32
    ) -> Vec<Vertex>
    where
        F: Fn(f64) -> (f64, f64) + Sync + Send,
    {
        let (t_min, t_max) = t_range;
        let t_len = t_max - t_min;
        if t_len <= 0.0 { return Vec::new(); }

        let total_samples = (t_len * SAMPLES_PER_UNIT_T).floor() as usize;
        let total_samples = total_samples.max(200);
        let step_t = t_len / total_samples as f64;

        // 1. 计算所有点 (包含屏幕外的)
        let path: Vec<(f64, f64)> = (0..=total_samples).into_par_iter().map(|i| {
            let t = t_min + i as f64 * step_t;
            f(t)
        }).collect();

        // 2. 准备网格参数
        let pixel_size_world = (2.0 / zoom) / screen_h;
        let half_width_world = (width_px * 0.5) * pixel_size_world;

        // ★ 计算世界坐标系下的最大允许跳跃距离
        // 视口高度 = 2.0 / zoom
        let max_jump_dist_sq = ((2.0 / zoom) * JUMP_THRESHOLD_FACTOR).powi(2);

        let mut vertices = Vec::with_capacity(total_samples * 6);

        // 3. 生成网格 (含熔断检测)
        for i in 0..path.len().saturating_sub(1) {
            let p0 = path[i];
            let p1 = path[i+1];

            // A. 数学有效性检测 (NaN / Inf)
            if !p0.0.is_finite() || !p0.1.is_finite() ||
                !p1.0.is_finite() || !p1.1.is_finite() {
                continue; // 跳过，从而断开连接
            }

            // B. 计算两点距离平方 (World Space)
            let dx = p1.0 - p0.0;
            let dy = p1.1 - p0.1;
            let dist_sq = (dx*dx + dy*dy) as f32;

            // 如果两点重合，跳过
            if dist_sq < 1e-12 { continue; }

            // ★ C. 渐近线熔断检测 (Asymptote Culling)
            // 如果一步跨越了半个银河系，那肯定是渐近线，切断它！
            if dist_sq > max_jump_dist_sq {
                continue;
            }

            // --- 正常的网格挤出逻辑 ---
            let len = dist_sq.sqrt() as f64;

            // 法线
            let nx = -dy / len;
            let ny = dx / len;

            // 挤出顶点
            let offset_x = nx * half_width_world as f64;
            let offset_y = ny * half_width_world as f64;

            let p0_l = Vertex { position: [(p0.0 + offset_x) as f32, (p0.1 + offset_y) as f32] };
            let p0_r = Vertex { position: [(p0.0 - offset_x) as f32, (p0.1 - offset_y) as f32] };
            let p1_l = Vertex { position: [(p1.0 + offset_x) as f32, (p1.1 + offset_y) as f32] };
            let p1_r = Vertex { position: [(p1.0 - offset_x) as f32, (p1.1 - offset_y) as f32] };

            // 生成两个三角形 (Quad)
            vertices.push(p0_l); vertices.push(p1_l); vertices.push(p0_r);
            vertices.push(p0_r); vertices.push(p1_l); vertices.push(p1_r);
        }

        vertices
    }
}