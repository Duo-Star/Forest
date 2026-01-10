// src/giac
#[allow(dead_code)]

use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

use super::common::{Vertex, GeoObj, GeoType};
use super::implicit::ImplicitSolver;
use super::parametric::ParametricSolver;
use super::explicit::ExplicitSolver;

// 4x MSAA
const SAMPLE_COUNT: u32 = 4; // 4倍采样，效果通常足够好

// 全局 Uniform (注意对齐)
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct ViewUniforms {
    center: [f32; 2],     // 8
    zoom: f32,            // 4
    aspect: f32,          // 4
    resolution: [f32; 2], // 8
    _pad: [f32; 2],       // 8 -> Total 32 bytes (16-byte aligned)
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct StyleUniform {
    color: [f32; 4],
    width: f32,
    _padding: [f32; 3],
}

struct ViewState {
    center_x: f64,
    center_y: f64,
    zoom: f64,
    is_dragging: bool,
    last_mouse_pos: Option<(f64, f64)>,
    dirty: bool,
}

struct RenderLayer {
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    style_buffer: wgpu::Buffer,
    style_bind_group: wgpu::BindGroup,
}

struct WindowState {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    // ★ 新增：MSAA 纹理
    msaa_texture: wgpu::Texture,

    grid_pipeline: wgpu::RenderPipeline,
    point_pipeline: wgpu::RenderPipeline, // 隐函数
    mesh_pipeline: wgpu::RenderPipeline,  // 参数方程 (实心网格)

    globals_buffer: wgpu::Buffer,
    globals_bind_group: wgpu::BindGroup,
    style_bind_group_layout: wgpu::BindGroupLayout,
    layers: Vec<RenderLayer>,
}

pub struct D2Plotter {
    instance: wgpu::Instance,
    state: Option<WindowState>,
    view: ViewState,
    objects: Vec<GeoObj>,

    implicit_solver: ImplicitSolver,
    parametric_solver: ParametricSolver,
    explicit_solver: ExplicitSolver,
    last_frame_time: Option<Instant>,
}


fn create_msaa_texture(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    sample_count: u32
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Multisampled Framebuffer"),
        size: wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format: config.format, // 必须匹配 Surface 格式
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    })
}


impl D2Plotter {
    pub(crate) fn new() -> Self {
        Self {
            instance: wgpu::Instance::default(),
            state: None,
            view: ViewState {
                center_x: 0.0, center_y: 0.0, zoom: 1.0,
                is_dragging: false, last_mouse_pos: None, dirty: true,
            },
            objects: Vec::new(),
            implicit_solver: ImplicitSolver::new(),
            parametric_solver: ParametricSolver::new(),
            explicit_solver: ExplicitSolver::new(),
            last_frame_time: None,
        }
    }

    pub fn add_object(&mut self, obj: GeoObj) {
        self.objects.push(obj);
    }

    fn update_sim(&mut self) {
        let s = match self.state.as_mut() { Some(s) => s, None => return };

        let width = s.config.width;
        let height = s.config.height;
        let aspect = width as f32 / height as f32;

        let range_y = 2.0 / self.view.zoom;
        let range_x = range_y * aspect as f64;
        let x_range = (self.view.center_x - range_x, self.view.center_x + range_x);
        let y_range = (self.view.center_y - range_y, self.view.center_y + range_y);

        // 同步 Layer
        if s.layers.len() != self.objects.len() {
            s.layers.clear();
            for obj in &self.objects {
                let style_data = StyleUniform { color: obj.color, width: obj.width, _padding: [0.0;3] };
                let buffer = s.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Style Buffer"),
                    contents: bytemuck::cast_slice(&[style_data]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
                let bg = s.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Style BindGroup"),
                    layout: &s.style_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() }],
                });
                let vb = s.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Empty VB"), size: 1024, usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false
                });

                s.layers.push(RenderLayer {
                    vertex_buffer: vb,
                    vertex_count: 0,
                    style_buffer: buffer,
                    style_bind_group: bg,
                });
            }
        }

        // 计算
        for (i, obj) in self.objects.iter().enumerate() {
            let layer = &mut s.layers[i];

            let vertices = match &obj.geo_type {
                GeoType::Implicit(func) => {
                    self.implicit_solver.solve(func, x_range, y_range, width, height)
                },
                GeoType::Parametric(func, t_range) => {
                    // ★ 核心修改：传入屏幕信息以计算线宽，返回三角形网格顶点
                    self.parametric_solver.solve(
                        func,
                        *t_range,
                        obj.width,
                        self.view.zoom as f32,
                        aspect,
                        height as f32
                    )
                },

                // ★ 新增：显函数处理
                GeoType::Explicit(func) => {
                    // 显函数只需要 x_range，以及屏幕信息
                    self.explicit_solver.solve(
                        func, x_range, obj.width,
                        self.view.zoom as f32, width, height as f32
                    )
                },

                GeoType::Geometry => Vec::new()
            };

            if vertices.len() > 0 {
                let required_size = (vertices.len() * size_of::<Vertex>()) as u64;
                if layer.vertex_buffer.size() < required_size {
                    layer.vertex_buffer = s.device.create_buffer(&wgpu::BufferDescriptor {
                        label: Some("Resize VB"),
                        size: required_size * 2,
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                        mapped_at_creation: false,
                    });
                }
                s.queue.write_buffer(&layer.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
                layer.vertex_count = vertices.len() as u32;
            } else {
                layer.vertex_count = 0;
            }
        }

        self.view.dirty = false;
    }

    fn redraw(&mut self) {
        if self.view.dirty { self.update_sim(); }
        let s = match self.state.as_mut() { Some(s) => s, None => return };

        let width = s.config.width as f32;
        let height = s.config.height as f32;

        let globals = ViewUniforms {
            center: [self.view.center_x as f32, self.view.center_y as f32],
            zoom: self.view.zoom as f32,
            aspect: width / height,
            resolution: [width, height],
            _pad: [0.0; 2],
        };
        s.queue.write_buffer(&s.globals_buffer, 0, bytemuck::cast_slice(&[globals]));

        let frame = s.surface.get_current_texture().expect("Failed to acquire frame");
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = s.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // 获取 MSAA 的 View
        let msaa_view = s.msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut rp =  encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    // ★ view 指向 MSAA 纹理
                    view: &msaa_view,

                    // ★ resolve_target 指向真正的屏幕 Frame
                    resolve_target: Some(&view),

                    ops: wgpu::Operations {
                        // 清除 MSAA 缓冲区
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.05, g: 0.05, b: 0.05, a: 1.0 }),
                        // store 必须为 Discard，因为我们只关心 resolve_target 的结果
                        store: wgpu::StoreOp::Discard,
                    },
                    depth_slice: None,
                })],
                ..Default::default()
            });

            // Pass 1: Grid
            rp.set_pipeline(&s.grid_pipeline);
            rp.set_bind_group(0, &s.globals_bind_group, &[]);
            rp.draw(0..3, 0..1);

            // Pass 2: Graph Objects
            for (i, obj) in self.objects.iter().enumerate() {
                let layer = &s.layers[i];
                if layer.vertex_count > 0 {
                    rp.set_bind_group(1, &layer.style_bind_group, &[]);

                    match obj.geo_type {
                        GeoType::Implicit(_) => {
                            // 隐函数：使用 Point Pipeline (Instancing)
                            rp.set_pipeline(&s.point_pipeline);
                            // Slot 0 is Instance Data
                            rp.set_vertex_buffer(0, layer.vertex_buffer.slice(0..(layer.vertex_count as u64 * 8)));
                            rp.draw(0..4, 0..layer.vertex_count);
                        },
                        // ★ 参数方程和显函数都使用 Mesh Pipeline (实心三角形)
                        GeoType::Parametric(_, _) | GeoType::Explicit(_) => {
                            rp.set_pipeline(&s.mesh_pipeline);
                            rp.set_vertex_buffer(0, layer.vertex_buffer.slice(0..(layer.vertex_count as u64 * 8)));
                            rp.draw(0..layer.vertex_count, 0..1);
                        },
                        _ => {}
                    }
                }
            }
        }
        s.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}

impl ApplicationHandler for D2Plotter {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(Window::default_attributes().with_title("GraphMF - 12.27 - Duo")).unwrap());
        let s = pollster::block_on(async {
            let surface = self.instance.create_surface(window.clone()).unwrap();
            let adapter = self.instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
            let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();

            let size = window.inner_size();
            let config = surface.get_default_config(&adapter, size.width, size.height).unwrap();
            surface.configure(&device, &config);

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            // Layouts
            let globals_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Globals Layout"),
                entries: &[wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::VERTEX_FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None }],
            });
            let style_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Style Layout"),
                entries: &[wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None }],
            });

            // Globals
            let globals_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Globals Buffer"), size: size_of::<ViewUniforms>() as u64, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
            });
            let globals_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Globals BG"), layout: &globals_layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() }],
            });

            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None, bind_group_layouts: &[&globals_layout, &style_layout], immediate_size: 0,
            });

            // 1. Grid Pipeline
            let grid_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Grid Pipeline"),
                layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { bind_group_layouts: &[&globals_layout], ..Default::default() })),
                vertex: wgpu::VertexState { module: &shader, entry_point: Some("vs_grid"), buffers: &[], compilation_options: Default::default() },
                fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fs_grid"), targets: &[Some(config.format.into())], compilation_options: Default::default() }),
                primitive: wgpu::PrimitiveState::default(), depth_stencil: None,multisample: wgpu::MultisampleState {
                    count: SAMPLE_COUNT, // 改为 4
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                }, cache: None, multiview_mask: None,
            });

            // 2. Point Pipeline (Implicit: Instancing)
            let point_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Point Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader, entry_point: Some("vs_point"),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: 8,
                        step_mode: wgpu::VertexStepMode::Instance, // 按实例
                        attributes: &wgpu::vertex_attr_array![0 => Float32x2]
                    }],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader, entry_point: Some("fs_point"),
                    targets: &[Some(wgpu::ColorTargetState { format: config.format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleStrip, ..Default::default() },
                depth_stencil: None, multisample: wgpu::MultisampleState {
                    count: SAMPLE_COUNT, // 改为 4
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                }, cache: None, multiview_mask: None,
            });

            // 3. Mesh Pipeline (Parametric: Solid Triangles)
            let mesh_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Mesh Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader, entry_point: Some("vs_mesh"),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: 8,
                        step_mode: wgpu::VertexStepMode::Vertex, // 按顶点
                        attributes: &wgpu::vertex_attr_array![0 => Float32x2]
                    }],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader, entry_point: Some("fs_mesh"),
                    targets: &[Some(wgpu::ColorTargetState { format: config.format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
                    compilation_options: Default::default(),
                }),
                // ★ 使用 TriangleList 绘制实心三角形
                primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleList, ..Default::default() },
                depth_stencil: None, multisample: wgpu::MultisampleState {
                    count: SAMPLE_COUNT, // 改为 4
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                }, cache: None, multiview_mask: None,
            });

            let msaa_texture = create_msaa_texture(&device, &config, SAMPLE_COUNT);

            WindowState {
                window, surface, device, queue, config,
                msaa_texture,
                grid_pipeline, point_pipeline, mesh_pipeline,
                globals_buffer, globals_bind_group,
                style_bind_group_layout: style_layout, layers: Vec::new(),
            }
        });
        self.state = Some(s);
    }

    // WindowEvent 保持不变 (缩放/拖拽逻辑)
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::MouseWheel { delta, .. } => {
                let s = self.state.as_ref().unwrap();
                let size = s.window.inner_size();
                let mouse_pos = self.view.last_mouse_pos.unwrap_or((size.width as f64 / 2.0, size.height as f64 / 2.0));
                let aspect = size.width as f64 / size.height as f64;
                let full_world_h = 2.0 * 2.0 / self.view.zoom;
                let full_world_w = full_world_h * aspect;
                let mouse_rel_x = (mouse_pos.0 / size.width as f64) - 0.5;
                let mouse_rel_y = 0.5 - (mouse_pos.1 / size.height as f64);
                let mouse_world_x = self.view.center_x + mouse_rel_x * full_world_w;
                let mouse_world_y = self.view.center_y + mouse_rel_y * full_world_h;

                let y = match delta { MouseScrollDelta::LineDelta(_, y) => y as f64, MouseScrollDelta::PixelDelta(pos) => pos.y / 60.0 };
                self.view.zoom *= 1.1f64.powf(y);

                let new_full_world_h = 4.0 / self.view.zoom;
                let new_full_world_w = new_full_world_h * aspect;
                self.view.center_x = mouse_world_x - mouse_rel_x * new_full_world_w;
                self.view.center_y = mouse_world_y - mouse_rel_y * new_full_world_h;

                self.view.dirty = true;
                s.window.request_redraw();
            }
            WindowEvent::MouseInput { state, button: MouseButton::Left, .. } => {
                self.view.is_dragging = state == ElementState::Pressed;
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.view.is_dragging {
                    if let (Some(s), Some(last)) = (&self.state, self.view.last_mouse_pos) {
                        let size = s.window.inner_size();
                        let aspect = size.width as f64 / size.height as f64;
                        let world_h = 4.0 / self.view.zoom;
                        let world_w = world_h * aspect;
                        let dx = (position.x - last.0) / size.width as f64 * world_w;
                        let dy = (position.y - last.1) / size.height as f64 * world_h;
                        self.view.center_x -= dx;
                        self.view.center_y += dy;

                        self.view.dirty = true;
                        s.window.request_redraw();
                    }
                }
                self.view.last_mouse_pos = Some((position.x, position.y));
            }
            WindowEvent::Resized(new_size) => {
                if let Some(s) = self.state.as_mut() {
                    s.config.width = new_size.width.max(1);
                    s.config.height = new_size.height.max(1);
                    s.surface.configure(&s.device, &s.config);
                    // ★ 新增：窗口大小变了，MSAA 纹理也要变
                    s.msaa_texture = create_msaa_texture(&s.device, &s.config, SAMPLE_COUNT);

                    self.view.dirty = true;
                    s.window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => { self.redraw(); }
            _ => (),
        }
    }
}

