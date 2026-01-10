// src/shader.wgsl

// 全局 Uniform
struct ViewUniforms {
    center: vec2<f32>,
    zoom: f32,
    aspect: f32,
    resolution: vec2<f32>,
    _pad: vec2<f32>, // 强制对齐到 32 字节
};

struct Style {
    color: vec4<f32>,
    width: f32,
};

@group(0) @binding(0) var<uniform> view: ViewUniforms;
@group(1) @binding(0) var<uniform> style: Style;

// --- 通用顶点输出 ---
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>, // 仅隐函数用到
};

// ==========================================
// 1. Grid Shader (背景)
// ==========================================
@vertex
fn vs_grid(@builtin(vertex_index) idx: u32) -> VertexOutput {
    var out: VertexOutput;
    // 全屏三角形
    let x = f32(i32(idx << 1u) & 2) * 2.0 - 1.0;
    let y = f32(i32(idx) & 2) * 2.0 - 1.0;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);

    // 计算世界坐标传给片元
    let range_y = 2.0 / view.zoom;
    let world_x = view.center.x + x * range_y * view.aspect;
    let world_y = view.center.y + y * range_y;
    out.uv = vec2<f32>(world_x, world_y); // 借用 uv 传参
    return out;
}

@fragment
fn fs_grid(in: VertexOutput) -> @location(0) vec4<f32> {
    let coord = in.uv;
    let px = fwidth(coord);
    let axis = smoothstep(2.0 * px, vec2<f32>(0.0), abs(coord));
    let grid = max(axis.x, axis.y);
    return mix(vec4<f32>(0.05, 0.05, 0.05, 1.0), vec4<f32>(0.5, 0.5, 0.5, 1.0), grid);
}

// ==========================================
// 2. Implicit Shader (Points) - 隐函数点
// ==========================================
@vertex
fn vs_point(
    @builtin(vertex_index) idx: u32,
    @location(0) center_pos: vec2<f32>
) -> VertexOutput {
    var out: VertexOutput;

    // 生成 Quad (-1..1)
    let u = f32(i32(idx) & 1) * 2.0 - 1.0;
    let v = f32(i32(idx >> 1u) & 1) * 2.0 - 1.0;
    out.uv = vec2<f32>(u, v);

    // 变换到 NDC
    let range_y = 2.0 / view.zoom;
    let range_x = range_y * view.aspect;

    let ndc_x = (center_pos.x - view.center.x) / range_x;
    let ndc_y = (center_pos.y - view.center.y) / range_y;

    // 加上点的大小偏移
    let pixel_scale = vec2<f32>(2.0/view.resolution.x, 2.0/view.resolution.y);
    let offset = vec2<f32>(u, v) * (style.width * 0.5) * pixel_scale;

    out.clip_position = vec4<f32>(ndc_x + offset.x, ndc_y + offset.y, 0.0, 1.0);
    return out;
}

@fragment
fn fs_point(in: VertexOutput) -> @location(0) vec4<f32> {
    let d = dot(in.uv, in.uv);
    let alpha = 1.0 - smoothstep(0.8, 1.0, d);
    if (alpha <= 0.0) { discard; }
    return vec4<f32>(style.color.rgb, style.color.a * alpha);
}

// ==========================================
// 3. Parametric Shader (Solid Mesh) - 参数方程线
//    CPU 已经把顶点算好了，这里只负责简单的世界转屏幕
// ==========================================
@vertex
fn vs_mesh(@location(0) pos: vec2<f32>) -> @builtin(position) vec4<f32> {
    let range_y = 2.0 / view.zoom;
    let range_x = range_y * view.aspect;

    let ndc_x = (pos.x - view.center.x) / range_x;
    let ndc_y = (pos.y - view.center.y) / range_y;

    return vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
}

@fragment
fn fs_mesh() -> @location(0) vec4<f32> {
    return style.color;
}