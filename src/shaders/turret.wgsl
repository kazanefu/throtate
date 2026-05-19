#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct TurretParams {
    time: f32,
}

@group(2) @binding(0)
var<uniform> params: TurretParams;

fn sd_box(p: vec2<f32>, b: vec2<f32>) -> f32 {
    let d = abs(p) - b;
    return length(max(d, vec2<f32>(0.0))) + min(max(d.x, d.y), 0.0);
}

fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h = max(k - abs(a - b), 0.0) / k;
    return min(a, b) - h * h * h * k * (1.0 / 6.0);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let pos = vec2<f32>(uv.x - 0.5, 0.5 - uv.y);
    
    let fw = fwidth(pos.x);
    let transition = max(fw, 0.005);
    
    // -----------------------------------------------------
    // 1. Geometry definition (SDF)
    // -----------------------------------------------------
    // Base circle centered at (0, 0)
    let r_circle = 0.38;
    let d_circle = length(pos) - r_circle;
    
    // Barrel (rectangular barrel sticking out to the right)
    let barrel_center = vec2<f32>(0.24, 0.0);
    let barrel_half_size = vec2<f32>(0.24, 0.11);
    let d_barrel = sd_box(pos - barrel_center, barrel_half_size);
    
    // Smooth union of the circle and barrel
    let d_shape = smin(d_circle, d_barrel, 0.05);
    
    // Inner glowing core
    let d_core = length(pos) - 0.12;
    
    // -----------------------------------------------------
    // 2. Glow and Fill coloring
    // -----------------------------------------------------
    // Solid fill masks
    let fill_shape = smoothstep(transition, -transition, d_shape);
    let fill_core = smoothstep(transition, -transition, d_core);
    
    // Outer edge glowing orange border
    let glow_outer = exp(-max(d_shape, 0.0) * 35.0) * 0.9 + exp(-max(d_shape, 0.0) * 12.0) * 0.45;
    let glow_inner = exp(min(d_shape, 0.0) * 28.0) * 0.65;
    
    // Core glow pulse
    let pulse = 1.0 + sin(params.time * 4.2) * 0.18;
    let core_glow = exp(-abs(d_core) * 45.0) * 0.95 * pulse;
    
    // Color definitions
    let fill_color = vec3<f32>(0.13, 0.15, 0.17); // Dark charcoal metal
    let orange_glow = vec3<f32>(1.0, 0.42, 0.06); // Intense neon orange
    
    // Base shape fill with orange inner edge glow
    var color = mix(vec3<f32>(0.0), mix(fill_color, orange_glow * 1.6, glow_inner), fill_shape);
    
    // Add central core glow
    color = mix(color, orange_glow * 2.2, fill_core);
    color = color + orange_glow * core_glow;
    
    // Add outer glow for the silhouette
    color = color + orange_glow * glow_outer * (1.0 - fill_shape);
    
    // -----------------------------------------------------
    // 3. Scanlines and Alpha Composition
    // -----------------------------------------------------
    let scanline = 0.95 + sin(uv.y * 140.0 + params.time * 6.0) * 0.05;
    color = color * scanline;
    
    let alpha = max(fill_shape, max(glow_outer, core_glow));
    
    return vec4<f32>(color * alpha, alpha);
}
