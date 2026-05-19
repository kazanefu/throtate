#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct BulletParams {
    time: f32,
}

@group(2) @binding(0)
var<uniform> params: BulletParams;

fn hash(p: vec2<f32>) -> vec2<f32> {
    return fract(sin(vec2<f32>(
        dot(p, vec2<f32>(127.1, 311.7)),
        dot(p, vec2<f32>(269.5, 183.3))
    )) * 43758.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(hash(i + vec2<f32>(0.0, 0.0)).x, hash(i + vec2<f32>(1.0, 0.0)).x, u.x),
        mix(hash(i + vec2<f32>(0.0, 1.0)).x, hash(i + vec2<f32>(1.0, 1.0)).x, u.x),
        u.y
    );
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let pos = vec2<f32>(uv.x - 0.5, 0.5 - uv.y);
    
    let dist = length(pos);
    let angle = atan2(pos.y, pos.x);
    
    // -----------------------------------------------------
    // 1. Spinning Shuriken shape definition
    // -----------------------------------------------------
    let rot_angle = angle - params.time * 15.0;
    let r_shuriken = 0.12 + 0.10 * cos(4.0 * rot_angle);
    
    // Add procedural flames flowing outwards
    let noise_val = noise(vec2<f32>(rot_angle * 3.5, dist * 10.0 - params.time * 18.0));
    let r_fire = r_shuriken + noise_val * 0.07;
    
    let d_fire = dist - r_fire;
    
    // -----------------------------------------------------
    // 2. Fire coloring and Glow
    // -----------------------------------------------------
    let fill = smoothstep(0.015, -0.015, d_fire);
    let glow = exp(-max(d_fire, 0.0) * 32.0) * 0.9 + exp(-max(d_fire, 0.0) * 10.0) * 0.45;
    
    let core_color = vec3<f32>(1.0, 1.0, 0.85); // Yellow-white core
    let mid_color = vec3<f32>(1.0, 0.35, 0.02); // Flame orange
    let outer_color = vec3<f32>(0.9, 0.04, 0.0); // Smoky red
    
    let t_fill = smoothstep(-0.15, 0.0, d_fire);
    let base_color = mix(core_color, mid_color, t_fill);
    
    let color = mix(outer_color * glow, base_color, fill);
    let alpha = max(fill, glow);
    
    return vec4<f32>(color * alpha, alpha);
}
