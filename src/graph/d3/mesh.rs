// src/d3/mesh.rs
use bytemuck::{Pod, Zeroable};

// ★ 引入 MathForest Vec3 (f64)
use crate::math_forest::geometry::d3::linear::vec3::Vec3;

// GPU 顶点结构体 (保持 f32，WGPU 标准管线)
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex3D {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

pub struct MeshData {
    pub vertices: Vec<Vertex3D>,
    pub indices: Vec<u32>,
}

impl MeshData {
    // ★ 泛型 F 现在返回 MathForest::Vec3 (f64)
    pub fn new_parametric_surface<F>(
        func: F,
        u_range: (f64, f64),
        v_range: (f64, f64),
        u_segments: u32,
        v_segments: u32
    ) -> Self
    where F: Fn(f64, f64) -> Vec3
    {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let (u_min, u_max) = u_range;
        let (v_min, v_max) = v_range;
        let u_step = (u_max - u_min) / u_segments as f64;
        let v_step = (v_max - v_min) / v_segments as f64;

        for i in 0..=u_segments {
            for j in 0..=v_segments {
                let u = u_min + i as f64 * u_step;
                let v = v_min + j as f64 * v_step;

                let pos = func(u, v);

                // --- ★ 修复开始 ★ ---
                let eps = 1e-9;

                // 计算偏导数 (Approximate Derivatives)
                // du ≈ (P(u+e) - P(u)) / e
                // dv ≈ (P(v+e) - P(u)) / e
                // 这样 du 和 dv 的模长就是正常的几何尺度 (约等于1或其他常数)，而不是 1e-9
                let pos_u = func(u + eps, v);
                let pos_v = func(u, v + eps);

                let du = (pos_u - pos) * (1.0 / eps);
                let dv = (pos_v - pos) * (1.0 / eps);

                // 现在 du 和 dv 是正常的切向量，叉积后模长也是正常的
                // unit() 可以正常工作
                let normal = du.cross(dv).unit();
                // --- ★ 修复结束 ★ ---

                vertices.push(Vertex3D {
                    position: [pos.x as f32, pos.y as f32, pos.z as f32],
                    normal:   [normal.x as f32, normal.y as f32, normal.z as f32]
                });
            }
        }

        // 索引生成逻辑不变
        for i in 0..u_segments {
            for j in 0..v_segments {
                let row1 = i * (v_segments + 1);
                let row2 = (i + 1) * (v_segments + 1);
                let a = row1 + j; let b = row1 + j + 1;
                let c = row2 + j + 1; let d = row2 + j;
                indices.extend_from_slice(&[a, d, b, b, d, c]);
            }
        }
        Self { vertices, indices }
    }

    // 坐标轴 (简单物体直接构造 f32 数据)
    pub fn new_axes(length: f32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let o = [0.0, 0.0, 0.0];
        let x = [length, 0.0, 0.0];
        let y = [0.0, length, 0.0];
        let z = [0.0, 0.0, length];
        let n = [0.0, 0.0, 0.0]; // 法线对于线框渲染通常不重要
        vertices.push(Vertex3D { position: o, normal: n });
        vertices.push(Vertex3D { position: x, normal: n });
        vertices.push(Vertex3D { position: y, normal: n });
        vertices.push(Vertex3D { position: z, normal: n });
        indices.extend_from_slice(&[0, 1, 0, 2, 0, 3]);
        Self { vertices, indices }
    }

    // 平面 (简单物体)
    pub fn new_plane(size: f32) -> Self {
        let h = size / 2.0;
        let n = [0.0, 0.0, 1.0];
        let vertices = vec![
            Vertex3D { position: [-h, -h, 0.0], normal: n },
            Vertex3D { position: [ h, -h, 0.0], normal: n },
            Vertex3D { position: [ h,  h, 0.0], normal: n },
            Vertex3D { position: [-h,  h, 0.0], normal: n },
        ];
        // 两个三角形组成一个矩形
        let indices = vec![0, 1, 2, 0, 2, 3];
        Self { vertices, indices }
    }
}