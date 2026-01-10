// src/d3/camera.rs

use super::super::super::math_forest::geometry::d3::linear::vec3::Vec3;
use super::super::super::math_forest::algebra::linear::matrix4x4::Matrix4x4;

use winit::event::{MouseButton, MouseScrollDelta};
use glam::Mat4; // 仅保留 Mat4 用于最终输出给 GPU

pub struct Camera {
    pub target: Vec3, // [替换] DVec3 -> Vec3
    pub yaw: f64,
    pub pitch: f64,
    pub radius: f64,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            target: Vec3::ZERO, // [替换] DVec3::ZERO -> Vec3::ZERO
            yaw: 45.0f64.to_radians(),
            pitch: 30.0f64.to_radians(),
            radius: 10.0,
        }
    }

    // 生成视图投影矩阵
    // 输出给 Shader 的 Uniform 必须是 f32
    pub fn build_view_projection_matrix(&self, aspect: f32) -> Mat4 {
        let eye = self.get_eye_position();

        // [替换] 使用 MathForest::Matrix4x4 进行高精度矩阵计算
        // 注意：Vec3::K 代表 Z 轴 (0,0,1)
        let view = Matrix4x4::look_at_rh(eye, self.target, Vec3::K);

        // [替换] 使用 perspective_rh_gl (对应 OpenGL [-1, 1] 深度)
        let proj = Matrix4x4::perspective_rh_gl(45.0f64.to_radians(), aspect as f64, 0.1, 1000.0);

        let view_proj = proj * view;

        // ★ [核心适配] 将 MathForest(f64, Row-Major) 转换为 glam(f32, Col-Major)
        // 我们的 m 数组是 [Row0, Row1, Row2, Row3]
        // glam 期望的是 [Col0, Col1, Col2, Col3]
        // 因此需要先转置 (Transpose)，把列变成行存储的形式，再转 f32
        let t = view_proj.transpose();

        Mat4::from_cols_array(&[
            t.m[0] as f32, t.m[1] as f32, t.m[2] as f32, t.m[3] as f32,
            t.m[4] as f32, t.m[5] as f32, t.m[6] as f32, t.m[7] as f32,
            t.m[8] as f32, t.m[9] as f32, t.m[10] as f32, t.m[11] as f32,
            t.m[12] as f32, t.m[13] as f32, t.m[14] as f32, t.m[15] as f32,
        ])
    }

    pub fn get_eye_position(&self) -> Vec3 {
        let (sin_p, cos_p) = self.pitch.sin_cos();
        let (sin_y, cos_y) = self.yaw.sin_cos();

        let x = self.radius * cos_p * cos_y;
        let y = self.radius * cos_p * sin_y;
        let z = self.radius * sin_p;

        self.target + Vec3::new(x, y, z)
    }

    pub fn process_mouse_drag(&mut self, dx: f64, dy: f64, button: MouseButton) {
        match button {
            MouseButton::Left => {
                let sensitivity = 0.005;
                self.yaw -= dx * sensitivity;
                self.pitch += dy * sensitivity;
                self.pitch = self.pitch.clamp(-1.55, 1.55);
            }
            MouseButton::Middle => {
                let sensitivity = self.radius * 0.0015;

                let eye = self.get_eye_position();

                // [替换] normalize() -> unit()
                let forward = (self.target - eye).unit();

                // [替换] DVec3::Z -> Vec3::K, normalize() -> unit()
                let right = forward.cross(Vec3::K).unit();
                let up = right.cross(forward).unit();

                let delta = right * (-dx * sensitivity) + up * (dy * sensitivity);
                self.target += delta; // Vec3 实现了 AddAssign
            }
            _ => {}
        }
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        let zoom_amount = match delta {
            MouseScrollDelta::LineDelta(_, y) => *y as f64 * 1.0,
            MouseScrollDelta::PixelDelta(pos) => pos.y * 0.01,
        };
        self.radius -= zoom_amount;
        self.radius = self.radius.clamp(0.1, 1000.0);
    }
}