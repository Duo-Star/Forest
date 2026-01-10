// src/d3/mod.rs
mod camera;
mod mesh;
pub mod parametric_curve;
pub mod implicit_surface;
mod implicit_data; // 假设查找表在这里

// 导出求解器
pub use parametric_curve::ParametricCurveSolver;
pub use implicit_surface::ImplicitSurfaceSolver;

use std::sync::Arc;
use std::mem::size_of;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, WindowEvent, DeviceEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};
use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

// --- ★ 引入 MathForest ---
use crate::math_forest::algebra::linear::matrix4x4::Matrix4x4;
use crate::math_forest::geometry::d3::linear::vec3::Vec3;

// 保留 glam::Mat4 仅用于与 Camera 的返回值对接 (Camera 内部已处理好 ViewProj 的 f32 转换)
use glam::Mat4;

use self::camera::Camera;
// 导出 MeshData 和 Vertex3D 以便外部使用
pub use self::mesh::{MeshData, Vertex3D};

// --- GPU 数据结构 ---

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Uniforms {
    view_proj: [f32; 16],   // 64 bytes
    model: [f32; 16],       // 64 bytes
    camera_pos: [f32; 3],   // 12 bytes
    _pad: f32,              // 4 bytes (align to 16)
    base_color: [f32; 4],   // 16 bytes
    use_lighting: f32,      // 4 bytes
    _pad2: [f32; 3],        // 12 bytes (align)
    _pad3: [f32; 4],        // 16 bytes -> Total 192 bytes (padding for alignment)
}

// 渲染对象 (GPU端 + 逻辑状态)
struct RenderObject {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    // 属性
    color: [f32; 4],
    use_lighting: bool,
    // ★ 使用 MathForest 的矩阵 (f64, Row-Major)
    model_matrix: Matrix4x4,
    topology: wgpu::PrimitiveTopology,
}

// ==========================================
// ★ 1. 3D 几何对象描述 (CPU 端)
// ==========================================
pub struct GeoObjD3 {
    pub mesh: MeshData,
    pub color: [f32; 4],
    pub topology: wgpu::PrimitiveTopology,
    pub use_lighting: bool,
    pub is_transparent: bool,
}

impl GeoObjD3 {
    // 辅助构造函数：创建一个标准的实体曲面
    pub fn new_surface(mesh: MeshData, color: [f32; 4]) -> Self {
        Self {
            mesh,
            color,
            topology: wgpu::PrimitiveTopology::TriangleList,
            use_lighting: true,
            is_transparent: false,
        }
    }

    // 辅助构造函数：创建一个线框对象
    pub fn new_wireframe(mesh: MeshData, color: [f32; 4]) -> Self {
        Self {
            mesh,
            color,
            topology: wgpu::PrimitiveTopology::LineList,
            use_lighting: false, // 线条通常不需要光照
            is_transparent: false,
        }
    }
}

// ==========================================
// ★ 2. State 结构体
// ==========================================
pub struct State {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    // 管线区分：普通网格、线条、半透明
    mesh_pipeline: wgpu::RenderPipeline,
    line_pipeline: wgpu::RenderPipeline,
    transparent_pipeline: wgpu::RenderPipeline,

    bind_group_layout: wgpu::BindGroupLayout,
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,

    camera: Camera,
    mouse_pressed: Option<MouseButton>,

    objects: Vec<RenderObject>, // 不透明对象
    transparent_objects: Vec<RenderObject>, // 半透明对象 (最后绘制)
}

impl State {
    async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();

        let config = surface.get_default_config(&adapter, size.width, size.height).unwrap();
        surface.configure(&device, &config);

        // BindGroup Layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniform_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                count: None,
            }],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            immediate_size: 0,
        });

        // 1. Mesh Pipeline (实体，开启深度写入)
        let mesh_pipeline = create_pipeline(&device, &pipeline_layout, &shader, config.format, wgpu::PrimitiveTopology::TriangleList, true, false);
        // 2. Line Pipeline (线框)
        let line_pipeline = create_pipeline(&device, &pipeline_layout, &shader, config.format, wgpu::PrimitiveTopology::LineList, true, false);
        // 3. Transparent Pipeline (开启混合，不写入深度但进行测试)
        let transparent_pipeline = create_pipeline(&device, &pipeline_layout, &shader, config.format, wgpu::PrimitiveTopology::TriangleList, false, true);

        // 深度纹理
        let (depth_texture, depth_view) = create_depth_texture(&device, &config);

        let mut state = Self {
            window, surface, device, queue, config,
            mesh_pipeline, line_pipeline, transparent_pipeline,
            bind_group_layout,
            depth_texture, depth_view,
            camera: Camera::new(),
            mouse_pressed: None,
            objects: Vec::new(),
            transparent_objects: Vec::new(),
        };

        // --- ★ 初始化默认场景 (坐标轴和网格) ---

        // X轴 (红)
        let mut x_mesh = MeshData::new_axes(100.0);
        x_mesh.indices.truncate(2); // 只取第一段
        state.add_mesh(x_mesh, [1.0, 0.0, 0.0, 1.0], false, wgpu::PrimitiveTopology::LineList, false);

        // Y轴 (绿)
        let mut y_mesh = MeshData::new_axes(100.0);
        y_mesh.indices = vec![0, 2]; // 假设 new_axes 0是原点, 2是y端点
        state.add_mesh(y_mesh, [0.0, 0.7, 0.0, 1.0], false, wgpu::PrimitiveTopology::LineList, false);

        // Z轴 (蓝)
        let mut z_mesh = MeshData::new_axes(100.0);
        z_mesh.indices = vec![0, 3]; // 假设 new_axes 0是原点, 3是z端点
        state.add_mesh(z_mesh, [0.0, 0.0, 1.0, 1.0], false, wgpu::PrimitiveTopology::LineList, false);

        // 地面网格 (灰透明)
        let grid_mesh = MeshData::new_plane(20.0);
        state.add_mesh(grid_mesh, [0.8, 0.8, 0.8, 0.3], false, wgpu::PrimitiveTopology::TriangleList, true);

        state
    }

    // 添加对象的方法 (内部使用)
    fn add_mesh(&mut self, mesh: MeshData, color: [f32; 4], use_lighting: bool, topology: wgpu::PrimitiveTopology, is_transparent: bool) {
        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("VB"), contents: bytemuck::cast_slice(&mesh.vertices), usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("IB"), contents: bytemuck::cast_slice(&mesh.indices), usage: wgpu::BufferUsages::INDEX,
        });

        // ★ MathForest 矩阵初始化 (默认单位阵)
        let model_matrix = Matrix4x4::IDENTITY;

        let uniforms = Uniforms {
            view_proj: Mat4::IDENTITY.to_cols_array(), // 占位，update时更新
            model: mat4_to_raw_f32(model_matrix),      // ★ 转换
            camera_pos: [0.0; 3],
            _pad: 0.0,
            base_color: color,
            use_lighting: if use_lighting { 1.0 } else { 0.0 },
            _pad2: [0.0; 3],
            _pad3: [0.0; 4],
        };

        let uniform_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("UB"), contents: bytemuck::cast_slice(&[uniforms]), usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("BG"), layout: &self.bind_group_layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: uniform_buffer.as_entire_binding() }],
        });

        let obj = RenderObject {
            vertex_buffer, index_buffer, num_indices: mesh.indices.len() as u32,
            uniform_buffer, bind_group, color, use_lighting, model_matrix, topology
        };

        if is_transparent {
            self.transparent_objects.push(obj);
        } else {
            self.objects.push(obj);
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            let (dt, dv) = create_depth_texture(&self.device, &self.config);
            self.depth_texture = dt; self.depth_view = dv;
        }
    }

    fn update(&mut self) {
        let aspect = self.config.width as f32 / self.config.height as f32;

        // Camera 返回的是 glam::Mat4 (已经针对 GPU 做过转置处理)，直接转数组
        let vp = self.camera.build_view_projection_matrix(aspect).to_cols_array();

        // MathForest::Vec3 -> [f32; 3]
        let cam_pos_f64 = self.camera.get_eye_position();
        let cam_pos = [cam_pos_f64.x as f32, cam_pos_f64.y as f32, cam_pos_f64.z as f32];

        // 更新所有对象 Uniform
        let update_obj = |obj: &RenderObject| {
            let u = Uniforms {
                view_proj: vp,
                // ★ MathForest Matrix4x4 (Row-Major) -> GPU (Col-Major f32)
                model: mat4_to_raw_f32(obj.model_matrix),
                camera_pos: cam_pos,
                _pad: 0.0,
                base_color: obj.color,
                use_lighting: if obj.use_lighting { 1.0 } else { 0.0 },
                _pad2: [0.0; 3],
                _pad3: [0.0; 4],
            };
            self.queue.write_buffer(&obj.uniform_buffer, 0, bytemuck::cast_slice(&[u]));
        };

        for obj in &self.objects { update_obj(obj); }
        for obj in &self.transparent_objects { update_obj(obj); }
    }

    fn render(&mut self) {
        let output = match self.surface.get_current_texture() { Ok(tex) => tex, Err(_) => return };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("3D Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.95, g: 0.95, b: 0.95, a: 1.0 }), store: wgpu::StoreOp::Store },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });

            // 1. 绘制不透明物体
            for obj in &self.objects {
                self.draw_obj(&mut rp, obj, &self.mesh_pipeline, &self.line_pipeline);
            }

            // 2. 绘制半透明物体
            rp.set_pipeline(&self.transparent_pipeline);
            for obj in &self.transparent_objects {
                self.draw_obj(&mut rp, obj, &self.transparent_pipeline, &self.line_pipeline); // 半透明通常是 Mesh
            }
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    fn draw_obj<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>, obj: &'a RenderObject, mesh_p: &'a wgpu::RenderPipeline, line_p: &'a wgpu::RenderPipeline) {
        match obj.topology {
            wgpu::PrimitiveTopology::TriangleList => rp.set_pipeline(mesh_p),
            wgpu::PrimitiveTopology::LineList => rp.set_pipeline(line_p),
            _ => {}
        }
        rp.set_bind_group(0, &obj.bind_group, &[]);
        rp.set_vertex_buffer(0, obj.vertex_buffer.slice(..));
        rp.set_index_buffer(obj.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        rp.draw_indexed(0..obj.num_indices, 0, 0..1);
    }
}

// ==========================================
// ★ 辅助函数
// ==========================================

// 将 MathForest::Matrix4x4 (f64, Row-Major) 转换为 WGPU 所需的 (f32, Col-Major)
fn mat4_to_raw_f32(m: Matrix4x4) -> [f32; 16] {
    // 关键：WGPU/OpenGL 期望列优先矩阵。
    // MathForest 是行优先的。
    // 行优先数据的 [Row0, Row1...] 如果直接当作列优先数据读取，相当于转置。
    // 但是 GPU 上的矩阵乘法通常是 Mat * Vec (标准数学写法)。
    // 如果我们把 CPU 的 Row0 传给 GPU 的 Col0，那么 GPU 上的矩阵就是 CPU 矩阵的转置。
    // 为了让 GPU 矩阵 == CPU 矩阵，我们需要在 CPU 端先转置一次（变成列优先存储），
    // 然后传给 GPU。

    let t = m.transpose(); // 转置以获得列优先的数据布局
    [
        t.m[0] as f32, t.m[1] as f32, t.m[2] as f32, t.m[3] as f32,
        t.m[4] as f32, t.m[5] as f32, t.m[6] as f32, t.m[7] as f32,
        t.m[8] as f32, t.m[9] as f32, t.m[10] as f32, t.m[11] as f32,
        t.m[12] as f32, t.m[13] as f32, t.m[14] as f32, t.m[15] as f32,
    ]
}

fn create_pipeline(
    device: &wgpu::Device, layout: &wgpu::PipelineLayout, shader: &wgpu::ShaderModule,
    fmt: wgpu::TextureFormat, topology: wgpu::PrimitiveTopology, depth_write: bool, blend: bool
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None, layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader, entry_point: Some("vs_main"),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: size_of::<Vertex3D>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
            }],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader, entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: fmt,
                blend: if blend { Some(wgpu::BlendState::ALPHA_BLENDING) } else { Some(wgpu::BlendState::REPLACE) },
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState { topology, cull_mode: None, ..Default::default() },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: depth_write,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(), multiview_mask: None, cache: None,
    })
}

fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> (wgpu::Texture, wgpu::TextureView) {
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        size: wgpu::Extent3d { width: config.width, height: config.height, depth_or_array_layers: 1 },
        mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        label: Some("Depth"), view_formats: &[],
    });
    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
    (tex, view)
}

// ==========================================
// ★ 3. AppD3 主入口
// ==========================================
pub struct D3Plotter {
    pub state: Option<State>,
    pub pending_objects: Vec<GeoObjD3>,
}

impl D3Plotter {
    pub fn new() -> Self {
        Self {
            state: None,
            pending_objects: Vec::new(),
        }
    }

    // ★ 对外接口：添加 3D 对象
    pub fn add_object(&mut self, obj: GeoObjD3) {
        self.pending_objects.push(obj);
    }
}

impl ApplicationHandler for D3Plotter {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(Window::default_attributes().with_title("MathForest - 3D")).unwrap());
        let mut state = pollster::block_on(State::new(window));

        // --- ★ 将暂存的对象上传到 GPU ---
        for obj in self.pending_objects.drain(..) {
            state.add_mesh(
                obj.mesh,
                obj.color,
                obj.use_lighting,
                obj.topology,
                obj.is_transparent
            );
        }

        self.state = Some(state);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let Some(state) = self.state.as_mut() {
            match event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::Resized(size) => state.resize(size),
                WindowEvent::RedrawRequested => { state.update(); state.render(); }
                WindowEvent::MouseInput { state: mstate, button, .. } => {
                    state.mouse_pressed = if mstate == ElementState::Pressed { Some(button) } else { None };
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    state.camera.process_scroll(&delta);
                    state.window.request_redraw();
                }
                _ => {}
            }
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: winit::event::DeviceId, event: DeviceEvent) {
        if let Some(state) = self.state.as_mut() {
            if let DeviceEvent::MouseMotion { delta } = event {
                if let Some(btn) = state.mouse_pressed {
                    state.camera.process_mouse_drag(delta.0, delta.1, btn);
                    state.window.request_redraw();
                }
            }
        }
    }
}