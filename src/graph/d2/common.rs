// src/common.rs
use bytemuck::{Pod, Zeroable};

// 统一使用这个顶点结构
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
}

// 几何类型
pub enum GeoType {
    // 隐函数 f(x, y) = 0
    Implicit(Box<dyn Fn(f64, f64) -> f64 + Sync + Send>),
    // 参数方程：存储函数、t范围
    Parametric(Box<dyn Fn(f64) -> (f64, f64) + Sync + Send>, (f64, f64)),
    // 显函数 y = f(x)
    Explicit(Box<dyn Fn(f64) -> f64 + Sync + Send>),
    // 几何对象
    Geometry,
}

pub struct GeoObj {
    pub geo_type: GeoType,
    pub color: [f32; 4],
    pub width: f32,
}

impl GeoObj {
    //
    pub fn new_implicit<F>(f: F, color: [f32; 4], width: f32) -> Self
    where F: Fn(f64, f64) -> f64 + Sync + Send + 'static{
        Self {
            geo_type: GeoType::Implicit(Box::new(f)),
            color,
            width
        }
    }

    //
    pub fn new_parametric<F>(f: F, t_range: (f64, f64), color: [f32; 4], width: f32) -> Self
    where F: Fn(f64) -> (f64, f64) + Sync + Send + 'static{
        Self {
            geo_type: GeoType::Parametric(Box::new(f), t_range),
            color,
            width
        }
    }

    // 显函数构造器
    pub fn new_explicit<F>(f: F, color: [f32; 4], width: f32) -> Self
    where F: Fn(f64) -> f64 + Sync + Send + 'static
    {
        Self {
            geo_type: GeoType::Explicit(Box::new(f)),
            color,
            width
        }
    }
}