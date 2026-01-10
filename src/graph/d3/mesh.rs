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
        v_segments: u32,
    ) -> Self
    where
        F: Fn(f64, f64) -> Vec3,
    {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let (u_min, u_max) = u_range;
        let (v_min, v_max) = v_range;
        let u_step = (u_max - u_min) / u_segments as f64;
        let v_step = (v_max - v_min) / v_segments as f64;

        // --- 1. 生成顶点 (含有效性检查) ---
        for i in 0..=u_segments {
            for j in 0..=v_segments {
                let u = u_min + i as f64 * u_step;
                let v = v_min + j as f64 * v_step;

                let pos = func(u, v);

                // ★ [新增]：断点/无效值检测
                // 如果坐标是 NaN 或无穷大，写入一个 NaN 标记，后续生成三角形时会跳过
                if !pos.x.is_finite() || !pos.y.is_finite() || !pos.z.is_finite() {
                    vertices.push(Vertex3D {
                        position: [f32::NAN; 3],
                        normal: [0.0; 3],
                    });
                    continue;
                }

                // --- ★ 之前修复的法线逻辑 (带安全检查) ★ ---
                let eps = 1e-9;
                let pos_u = func(u + eps, v);
                let pos_v = func(u, v + eps);

                // 安全检查：如果邻居也是无效值，无法计算法线，给一个默认向上的法线
                let normal = if pos_u.x.is_finite() && pos_v.x.is_finite() {
                    let du = (pos_u - pos) * (1.0 / eps);
                    let dv = (pos_v - pos) * (1.0 / eps);
                    du.cross(dv).unit()
                } else {
                    Vec3::new(0.0, 0.0, 1.0)
                };
                // ----------------------------------------

                vertices.push(Vertex3D {
                    position: [pos.x as f32, pos.y as f32, pos.z as f32],
                    normal: [normal.x as f32, normal.y as f32, normal.z as f32],
                });
            }
        }

        // --- 2. 生成索引 (含突跃屏蔽) ---

        // 定义最大允许边长的平方 (阈值)。
        // 如果两个网格点距离超过这个值（例如渐近线），则不连接三角形。
        // 这里设为 10.0 的平方，你可以根据场景缩放调整，或者作为参数传入。
        const JUMP_THRESHOLD_SQ: f32 = 10.0 * 10.0;

        for i in 0..u_segments {
            for j in 0..v_segments {
                let row1 = i * (v_segments + 1);
                let row2 = (i + 1) * (v_segments + 1);

                // 四个角点索引
                let idx_a = row1 + j;
                let idx_b = row1 + j + 1;
                let idx_c = row2 + j + 1;
                let idx_d = row2 + j;

                // 获取位置
                let pa = vertices[idx_a as usize].position;
                let pb = vertices[idx_b as usize].position;
                let pc = vertices[idx_c as usize].position;
                let pd = vertices[idx_d as usize].position;

                // 辅助闭包：检查三角形是否有效
                let is_valid_tri = |p1: [f32; 3], p2: [f32; 3], p3: [f32; 3]| -> bool {
                    // 1. 检查是否有 NaN (之前的断点)
                    if p1[0].is_nan() || p2[0].is_nan() || p3[0].is_nan() {
                        return false;
                    }

                    // 2. 检查边长是否过大 (突跃屏蔽)
                    let d12 =
                        (p1[0] - p2[0]).powi(2) + (p1[1] - p2[1]).powi(2) + (p1[2] - p2[2]).powi(2);
                    let d23 =
                        (p2[0] - p3[0]).powi(2) + (p2[1] - p3[1]).powi(2) + (p2[2] - p3[2]).powi(2);
                    let d31 =
                        (p3[0] - p1[0]).powi(2) + (p3[1] - p1[1]).powi(2) + (p3[2] - p1[2]).powi(2);

                    if d12 > JUMP_THRESHOLD_SQ || d23 > JUMP_THRESHOLD_SQ || d31 > JUMP_THRESHOLD_SQ
                    {
                        return false;
                    }
                    true
                };

                // Triangle 1: a-d-b
                if is_valid_tri(pa, pd, pb) {
                    indices.extend_from_slice(&[idx_a, idx_d, idx_b]);
                }

                // Triangle 2: b-d-c
                if is_valid_tri(pb, pd, pc) {
                    indices.extend_from_slice(&[idx_b, idx_d, idx_c]);
                }
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
        vertices.push(Vertex3D {
            position: o,
            normal: n,
        });
        vertices.push(Vertex3D {
            position: x,
            normal: n,
        });
        vertices.push(Vertex3D {
            position: y,
            normal: n,
        });
        vertices.push(Vertex3D {
            position: z,
            normal: n,
        });
        indices.extend_from_slice(&[0, 1, 0, 2, 0, 3]);
        Self { vertices, indices }
    }

    // 平面 (简单物体)
    pub fn new_plane(size: f32) -> Self {
        let h = size / 2.0;
        let n = [0.0, 0.0, 1.0];
        let vertices = vec![
            Vertex3D {
                position: [-h, -h, 0.0],
                normal: n,
            },
            Vertex3D {
                position: [h, -h, 0.0],
                normal: n,
            },
            Vertex3D {
                position: [h, h, 0.0],
                normal: n,
            },
            Vertex3D {
                position: [-h, h, 0.0],
                normal: n,
            },
        ];
        // 两个三角形组成一个矩形
        let indices = vec![0, 1, 2, 0, 2, 3];
        Self { vertices, indices }
    }
}
