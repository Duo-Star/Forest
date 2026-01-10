// src/d3/parametric_curve.rs
#![allow(dead_code)]

// ★ 引入 MathForest Vec3
use crate::math_forest::geometry::d3::linear::vec3::Vec3;
// 引用同模块下的 mesh
use super::mesh::{MeshData, Vertex3D};

pub struct ParametricCurveSolver;

impl ParametricCurveSolver {
    /// 生成管状体网格 (Tube Geometry)
    /// t_range: 参数范围
    /// radius: 管子的半径
    /// tube_segments: 管子截面的分段数 (圆度)
    /// path_segments: 沿路径的采样段数
    pub fn solve<F>(
        func: F,
        t_range: (f64, f64),
        radius: f64,
        tube_segments: u32,
        path_segments: u32,
    ) -> MeshData
    where
        F: Fn(f64) -> Vec3, // 返回 MathForest Vec3
    {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let (t_min, t_max) = t_range;
        let t_step = (t_max - t_min) / path_segments as f64;

        // 1. 计算路径骨架点 (P) 和 标架 (Frenet Frame 或类似)
        struct Frame {
            pos: Vec3,
            // tangent: Vec3, // 暂时没用到，如果是平行输运需要用到
            normal: Vec3,   // 管子截面的局部 X 轴
            binormal: Vec3, // 管子截面的局部 Y 轴
        }

        let mut frames = Vec::with_capacity((path_segments + 1) as usize);

        for i in 0..=path_segments {
            let t = t_min + i as f64 * t_step;
            let pos = func(t);

            // 有限差分算切线
            let eps = 1e-9;
            let pos_next = func(t + eps);

            // [API 适配] normalize_or_zero -> unit()
            let tangent = (pos_next - pos).unit();

            // 计算该点的一个垂直于切线的法向量
            // 策略：取一个任意向量辅助，如果切线接近该向量，换一个
            // [API 适配] DVec3::Y -> Vec3::J (0,1,0), DVec3::Z -> Vec3::K (0,0,1)
            let mut helper = Vec3::J;
            if tangent.dot(helper).abs() > 0.99 {
                helper = Vec3::K;
            }

            // [API 适配] cross, unit
            let normal = tangent.cross(helper).unit();
            let binormal = tangent.cross(normal).unit();

            // 注意：这种简单的法线生成方式在曲线出现拐点或由直变弯时可能会发生翻转 (Flipping)。
            // 更好的方式是使用 Parallel Transport (平行输运) 算法，但为了保持原逻辑简洁，此处保留。

            frames.push(Frame { pos, normal, binormal });
        }

        // 2. 生成管壁顶点
        for i in 0..=path_segments {
            let frame = &frames[i as usize];

            for j in 0..=tube_segments {
                // j 和 tube_segments 重合时闭合圆环 (0 和 2PI)
                let theta = (j as f64 / tube_segments as f64) * std::f64::consts::TAU;
                let (sin_t, cos_t) = theta.sin_cos();

                // 在局部平面计算偏移
                // Local Circle: cos * Normal + sin * Binormal
                let offset_dir = frame.normal * cos_t + frame.binormal * sin_t;
                let position = frame.pos + offset_dir * radius;

                // 管壁法线就是偏移方向 (单位向量)
                let normal = offset_dir.unit();

                // [类型转换] f64 -> f32 存入 VertexBuffer
                vertices.push(Vertex3D {
                    position: [position.x as f32, position.y as f32, position.z as f32],
                    normal:   [normal.x as f32, normal.y as f32, normal.z as f32],
                });
            }
        }

        // 3. 生成索引 (连接相邻圆环)
        let verts_per_ring = tube_segments + 1;

        for i in 0..path_segments {
            for j in 0..tube_segments {
                let row1 = i * verts_per_ring;
                let row2 = (i + 1) * verts_per_ring;

                let a = row1 + j;
                let b = row1 + j + 1;
                let c = row2 + j + 1;
                let d = row2 + j;

                // Triangle 1
                indices.extend_from_slice(&[a, d, b]);
                // Triangle 2
                indices.extend_from_slice(&[b, d, c]);
            }
        }

        MeshData { vertices, indices }
    }
}