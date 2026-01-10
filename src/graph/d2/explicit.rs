// src/d2/explicit.rs
use rayon::prelude::*;
use crate::graph::d2::common::Vertex;

// 采样密度
const SAMPLING_DENSITY: f64 = 1.0;
// 渐近线检测阈值：如果相邻两点 Y 差值超过“屏幕高度”的多少倍，则断开
// 10.0 是一个经验值，既能过滤掉 tan(x)，又不会误伤只是比较陡峭的函数
const ASYMPTOTE_THRESHOLD_FACTOR: f64 = 10.0;

pub struct ExplicitSolver {}

impl ExplicitSolver {
    pub fn new() -> Self { Self {} }

    pub fn solve<F>(
        &self,
        f: &F,
        x_range: (f64, f64),
        width_px: f32,
        zoom: f32,
        screen_w: u32,
        screen_h: f32
    ) -> Vec<Vertex>
    where
        F: Fn(f64) -> f64 + Sync + Send,
    {
        let (x_min, x_max) = x_range;
        let x_len = x_max - x_min;
        // 增加对 screen_w 的检查，防止除以0 panic
        if x_len <= 0.0 || screen_w == 0 { return Vec::new(); }

        let total_samples = (screen_w as f64 * SAMPLING_DENSITY).ceil() as usize;
        let total_samples = total_samples.max(100);

        let step_x = x_len / total_samples as f64;

        // 1. 并行计算路径点
        let path: Vec<(f64, f64)> = (0..=total_samples).into_par_iter().map(|i| {
            let x = x_min + i as f64 * step_x;
            let y = f(x);
            (x, y)
        }).collect();

        // 2. 准备网格生成参数
        // 屏幕上的 1 像素对应多少世界单位
        let pixel_size_world = (2.0 / zoom) / screen_h;
        let half_width_world = (width_px * 0.5) * pixel_size_world;

        // 计算“视口在世界坐标系下的高度”
        // 我们的视口 Y 范围通常是 center.y +/- (1.0 / zoom)
        // 所以总高度是 2.0 / zoom
        let view_height_world = 2.0 / zoom as f64;

        // 计算断点阈值：如果 dy > 10 * 屏幕高度，就认为是渐近线
        let jump_threshold = view_height_world * ASYMPTOTE_THRESHOLD_FACTOR;

        let mut vertices = Vec::with_capacity(total_samples * 6);

        // 3. 生成网格 (含断点检测)
        for i in 0..path.len().saturating_sub(1) {
            let p0 = path[i];
            let p1 = path[i+1];

            // A. 基础有效性检测 (NaN / Inf)
            if !p0.1.is_finite() || !p1.1.is_finite() { continue; }

            // B. ★★★ 渐近线/断点检测 ★★★
            let dy = (p1.1 - p0.1).abs();
            // 如果仅一步之遥(1像素宽)，Y值却跨越了数倍于屏幕的高度，这绝对是断点
            if dy > jump_threshold {
                continue;
            }

            // C. 正常的网格挤出逻辑
            let dx = p1.0 - p0.0;
            // 注意：这里 dy 已经算过了，如果不取绝对值的话: let dy_signed = p1.1 - p0.1;
            let dy_signed = p1.1 - p0.1;

            let len = (dx*dx + dy_signed*dy_signed).sqrt();
            if len < 1e-9 { continue; }

            let nx = -dy_signed / len;
            let ny = dx / len;

            // 挤出顶点 (World Space)
            let p0_l = Vertex { position: [(p0.0 + nx * half_width_world as f64) as f32, (p0.1 + ny * half_width_world as f64) as f32] };
            let p0_r = Vertex { position: [(p0.0 - nx * half_width_world as f64) as f32, (p0.1 - ny * half_width_world as f64) as f32] };
            let p1_l = Vertex { position: [(p1.0 + nx * half_width_world as f64) as f32, (p1.1 + ny * half_width_world as f64) as f32] };
            let p1_r = Vertex { position: [(p1.0 - nx * half_width_world as f64) as f32, (p1.1 - ny * half_width_world as f64) as f32] };

            vertices.push(p0_l); vertices.push(p1_l); vertices.push(p0_r);
            vertices.push(p0_r); vertices.push(p1_l); vertices.push(p1_r);
        }

        vertices
    }
}