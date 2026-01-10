// src/d3/implicit_surface.rs
#![allow(dead_code)]

use rayon::prelude::*;
use super::mesh::{MeshData, Vertex3D}; // 使用相对路径导入 mesh
use super::implicit_data::{EDGE_TABLE, TRI_TABLE}; // 导入查找表

// ★ 引入 MathForest
use crate::math_forest::geometry::d3::linear::vec3::Vec3;

pub struct ImplicitSurfaceSolver;

impl ImplicitSurfaceSolver {
    /// Marching Cubes 算法实现
    /// x/y/z_range: 采样范围
    /// resolution: 分辨率 (例如 50 -> 50x50x50 个格子)
    pub fn solve<F>(
        func: &F, // 引用以支持多线程共享
        x_range: (f64, f64),
        y_range: (f64, f64),
        z_range: (f64, f64),
        resolution: u32,
    ) -> MeshData
    where
        F: Fn(f64, f64, f64) -> f64 + Sync + Send,
    {
        let res_p1 = (resolution + 1) as usize;
        let total_points = res_p1 * res_p1 * res_p1;

        let step_x = (x_range.1 - x_range.0) / resolution as f64;
        let step_y = (y_range.1 - y_range.0) / resolution as f64;
        let step_z = (z_range.1 - z_range.0) / resolution as f64;

        // 1. 并行计算标量场 (Scalar Field)
        // 使用 Vec 存储所有网格点的值，避免在 Marching 阶段重复计算函数
        let mut values = vec![0.0; total_points];

        // Rayon 并行填充
        values.par_chunks_mut(res_p1 * res_p1).enumerate().for_each(|(k, plane)| {
            let z = z_range.0 + k as f64 * step_z;
            for j in 0..res_p1 {
                let y = y_range.0 + j as f64 * step_y;
                for i in 0..res_p1 {
                    let x = x_range.0 + i as f64 * step_x;
                    let idx = j * res_p1 + i;
                    plane[idx] = func(x, y, z);
                }
            }
        });

        // 2. 并行 Marching Cubes
        // 我们将 Z 轴切片进行并行处理，每个线程计算一层的三角形
        let geometry_parts: Vec<(Vec<Vertex3D>, Vec<u32>)> = (0..resolution).into_par_iter().map(|k| {
            let mut local_vertices = Vec::new();
            let mut local_indices = Vec::new();
            let mut index_counter = 0;

            for j in 0..resolution {
                for i in 0..resolution {
                    // 计算 8 个角点的索引
                    // 坐标顺序参考 Paul Bourke (标准右手系顺序匹配 TRI_TABLE)
                    // 0:(x,y,z), 1:(x+1,y,z), 2:(x+1,y,z+1), 3:(x,y,z+1) ...

                    let idx0 = k as usize * res_p1 * res_p1 + j as usize * res_p1 + i as usize;

                    // 8个顶点的偏移 (x, y, z)
                    let corner_offsets = [
                        (0, 0, 0), (1, 0, 0), (1, 0, 1), (0, 0, 1),
                        (0, 1, 0), (1, 1, 0), (1, 1, 1), (0, 1, 1)
                    ];

                    let mut cube_index = 0;
                    let mut corner_vals = [0.0; 8];
                    let mut corner_pos = [Vec3::ZERO; 8];

                    for n in 0..8 {
                        let (di, dj, dk) = corner_offsets[n];
                        // 计算全局索引 (注意步长)
                        let global_idx = idx0 + dk * res_p1 * res_p1 + dj * res_p1 + di;

                        let val = values[global_idx];
                        corner_vals[n] = val;

                        // 世界坐标
                        let wx = x_range.0 + (i as usize + di) as f64 * step_x;
                        let wy = y_range.0 + (j as usize + dj) as f64 * step_y;
                        let wz = z_range.0 + (k as usize + dk) as f64 * step_z;
                        corner_pos[n] = Vec3::new(wx, wy, wz);

                        if val < 0.0 { // 假设 Isovalue = 0.0
                            cube_index |= 1 << n;
                        }
                    }

                    // 查表：如果完全在内部或外部，跳过
                    let edges = EDGE_TABLE[cube_index];
                    if edges == 0 { continue; }

                    // 插值计算 12 条边上的点
                    let mut vert_list = [Vec3::ZERO; 12];

                    if (edges & 1) != 0 { vert_list[0] = vertex_interp(corner_pos[0], corner_vals[0], corner_pos[1], corner_vals[1]); }
                    if (edges & 2) != 0 { vert_list[1] = vertex_interp(corner_pos[1], corner_vals[1], corner_pos[2], corner_vals[2]); }
                    if (edges & 4) != 0 { vert_list[2] = vertex_interp(corner_pos[2], corner_vals[2], corner_pos[3], corner_vals[3]); }
                    if (edges & 8) != 0 { vert_list[3] = vertex_interp(corner_pos[3], corner_vals[3], corner_pos[0], corner_vals[0]); }
                    if (edges & 16) != 0 { vert_list[4] = vertex_interp(corner_pos[4], corner_vals[4], corner_pos[5], corner_vals[5]); }
                    if (edges & 32) != 0 { vert_list[5] = vertex_interp(corner_pos[5], corner_vals[5], corner_pos[6], corner_vals[6]); }
                    if (edges & 64) != 0 { vert_list[6] = vertex_interp(corner_pos[6], corner_vals[6], corner_pos[7], corner_vals[7]); }
                    if (edges & 128) != 0 { vert_list[7] = vertex_interp(corner_pos[7], corner_vals[7], corner_pos[4], corner_vals[4]); }
                    if (edges & 256) != 0 { vert_list[8] = vertex_interp(corner_pos[0], corner_vals[0], corner_pos[4], corner_vals[4]); }
                    if (edges & 512) != 0 { vert_list[9] = vertex_interp(corner_pos[1], corner_vals[1], corner_pos[5], corner_vals[5]); }
                    if (edges & 1024) != 0 { vert_list[10] = vertex_interp(corner_pos[2], corner_vals[2], corner_pos[6], corner_vals[6]); }
                    if (edges & 2048) != 0 { vert_list[11] = vertex_interp(corner_pos[3], corner_vals[3], corner_pos[7], corner_vals[7]); }

                    // 生成三角形
                    for t in (0..16).step_by(3) {
                        let v_idx1 = TRI_TABLE[cube_index][t];
                        if v_idx1 == -1 { break; }
                        let v_idx2 = TRI_TABLE[cube_index][t+1];
                        let v_idx3 = TRI_TABLE[cube_index][t+2];

                        let p1 = vert_list[v_idx1 as usize];
                        let p2 = vert_list[v_idx2 as usize];
                        let p3 = vert_list[v_idx3 as usize];

                        // 计算法线：对该点位置再次求导 (Gradient)
                        let n1 = calc_gradient_normal(func, p1);
                        let n2 = calc_gradient_normal(func, p2);
                        let n3 = calc_gradient_normal(func, p3);

                        // Push 顶点 (MathForest f64 -> GPU f32)
                        local_vertices.push(Vertex3D {
                            position: [p1.x as f32, p1.y as f32, p1.z as f32],
                            normal:   [n1.x as f32, n1.y as f32, n1.z as f32]
                        });
                        local_vertices.push(Vertex3D {
                            position: [p2.x as f32, p2.y as f32, p2.z as f32],
                            normal:   [n2.x as f32, n2.y as f32, n2.z as f32]
                        });
                        local_vertices.push(Vertex3D {
                            position: [p3.x as f32, p3.y as f32, p3.z as f32],
                            normal:   [n3.x as f32, n3.y as f32, n3.z as f32]
                        });

                        local_indices.push(index_counter);
                        local_indices.push(index_counter + 1);
                        local_indices.push(index_counter + 2);
                        index_counter += 3;
                    }
                }
            }
            (local_vertices, local_indices)
        }).collect();

        // 3. 合并所有线程的网格
        let mut final_vertices = Vec::new();
        let mut final_indices = Vec::new();
        let mut base_index = 0;

        for (mut verts, mut idxs) in geometry_parts {
            // 修正 indices 的偏移量
            for i in &mut idxs {
                *i += base_index;
            }
            base_index += verts.len() as u32;

            final_vertices.append(&mut verts);
            final_indices.append(&mut idxs);
        }

        MeshData { vertices: final_vertices, indices: final_indices }
    }
}

// 辅助：线性插值找零点
#[inline]
fn vertex_interp(p1: Vec3, v1: f64, p2: Vec3, v2: f64) -> Vec3 {
    if (v2 - v1).abs() < 1e-9 { return p1; }
    let mu = (0.0 - v1) / (v2 - v1);
    // MathForest Vec3 支持 + - * 运算
    p1 + (p2 - p1) * mu
}

// 辅助：计算法线 (通过梯度)
// 隐曲面 f(x,y,z)=0 的法线就是 Gradient f
#[inline]
fn calc_gradient_normal<F>(func: &F, p: Vec3) -> Vec3
where F: Fn(f64, f64, f64) -> f64
{
    let eps = 1e-6;
    let dx = func(p.x + eps, p.y, p.z) - func(p.x - eps, p.y, p.z);
    let dy = func(p.x, p.y + eps, p.z) - func(p.x, p.y - eps, p.z);
    let dz = func(p.x, p.y, p.z + eps) - func(p.x, p.y, p.z - eps);

    // unit() 已经处理了零向量情况
    Vec3::new(dx, dy, dz).unit()
}