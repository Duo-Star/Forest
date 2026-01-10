struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    camera_pos: vec3<f32>,
    _pad: f32,
    // 新增：基础颜色
    base_color: vec4<f32>,
    // 新增：是否使用光照 (1.0 = enable, 0.0 = disable)
    use_lighting: f32,
    _pad2: vec3<f32>, // 对齐
};

@group(0) @binding(0) var<uniform> u: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = u.model * vec4<f32>(in.position, 1.0);
    out.world_pos = world_pos.xyz;
    out.clip_position = u.view_proj * world_pos;
    out.world_normal = in.normal;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 1. 如果不使用光照 (如坐标轴)，直接返回颜色
    if (u.use_lighting < 0.5) {
        return u.base_color;
    }

    // 2. 光照计算 (曲面)
    let object_color = u.base_color.rgb;
    let light_pos = vec3<f32>(10.0, 10.0, 20.0);
    let light_color = vec3<f32>(1.0, 1.0, 1.0);

    // 双面渲染法线修正：如果法线背对相机，翻转它
    var N = normalize(in.world_normal);
    let V = normalize(u.camera_pos - in.world_pos);
    if (dot(N, V) < 0.0) { N = -N; }

    let L = normalize(light_pos - in.world_pos);

    // Diffuse
    let diff = max(dot(N, L), 0.0);
    let diffuse = diff * light_color;

    // Specular
    let R = reflect(-L, N);
    let spec = pow(max(dot(V, R), 0.0), 32.0);
    let specular = 0.5 * spec * light_color;

    let ambient = 0.2 * light_color;

    let final_color = (ambient + diffuse + specular) * object_color;

    return vec4<f32>(final_color, u.base_color.a);
}