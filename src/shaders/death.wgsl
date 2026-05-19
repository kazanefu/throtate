#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct DeathParams {
    time: f32,
}

@group(2) @binding(0)
var<uniform> params: DeathParams;

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

fn hex_dist(p: vec2<f32>) -> f32 {
    let r = vec2<f32>(1.0, 1.7320508);
    let h = r * 0.5;
    let a = abs(fract(p / r) - h);
    let b = abs(fract((p - h) / r) - h);
    
    let da = max(a.x * 0.8660254 + a.y * 0.5, a.y);
    let db = max(b.x * 0.8660254 + b.y * 0.5, b.y);
    
    return min(da, db);
}

fn get_lightning(pos: vec2<f32>, time: f32) -> vec4<f32> {
    let period = 1.6;
    let time_block = floor(time / period);
    let block_time = time % period;
    
    // Hash for this block
    let h = hash(vec2<f32>(time_block, 42.89));
    let active_prob = h.x;
    
    // 55% chance of a strike in each period
    if (active_prob > 0.55) {
        return vec4<f32>(0.0);
    }
    
    // Strike duration is 0.38 seconds
    let strike_duration = 0.38;
    if (block_time > strike_duration) {
        return vec4<f32>(0.0);
    }
    
    let progress = block_time / strike_duration;
    
    // Random direction / angle
    let angle = h.y * 6.283185;
    let cos_a = cos(angle);
    let sin_a = sin(angle);
    let rot = mat2x2<f32>(cos_a, -sin_a, sin_a, cos_a);
    let rot_pos = rot * pos;
    
    // 2 octaves of fractal noise for jagged lightning shape
    let n1 = noise(vec2<f32>(rot_pos.x * 7.0, time_block * 5.12)) * 0.08;
    let n2 = noise(vec2<f32>(rot_pos.x * 16.0 + 2.0, time_block * 9.34)) * 0.025;
    let jagged = n1 + n2;
    
    let dist = abs(rot_pos.y - jagged);
    
    // Propagation mapping from one side of the rotated coordinates to the other
    let x_norm = rot_pos.x + 0.5;
    let propagation = smoothstep(x_norm - 0.2, x_norm, progress);
    
    // Decay: intense bright strike that fades/dissolves out
    let decay = 1.0 - progress;
    
    let core = smoothstep(0.007, 0.0, dist);
    let glow = exp(-dist * 38.0) * 0.85 + exp(-dist * 13.0) * 0.45;
    
    let intensity = (core + glow) * propagation * decay;
    let lightning_color = vec3<f32>(1.0, 0.08, 0.08) * 3.5;
    
    return vec4<f32>(lightning_color * intensity, intensity);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let pos = vec2<f32>(uv.x - 0.5, 0.5 - uv.y);
    
    // -----------------------------------------------------
    // 1. Diagonal Scrolling Warning Stripes (Top-Left to Bottom-Right)
    // -----------------------------------------------------
    let stripe_scale = 16.0;
    // Map coordinate diagonal along top-left -> bottom-right
    let coord = (uv.x + uv.y) * 0.7071 - params.time * 0.28;
    
    let stripe_val = sin(coord * stripe_scale);
    let fw = fwidth(coord);
    let transition = max(fw * stripe_scale, 0.05);
    
    let stripe_mask = smoothstep(-transition, transition, stripe_val);
    
    // -----------------------------------------------------
    // 2. Honeycomb grid pattern on the red stripes
    // -----------------------------------------------------
    // Scroll honeycomb grid in sync with the stripes
    let scroll_offset = vec2<f32>(1.0, 1.0) * params.time * 0.198;
    let hex_uv = (uv - scroll_offset) * 26.0;
    
    let d = hex_dist(hex_uv);
    let hex_border = smoothstep(0.41, 0.46, d);
    
    let cell_color = vec3<f32>(0.68, 0.0, 0.02);
    let border_color = vec3<f32>(1.0, 0.15, 0.05) * 1.6;
    
    let pulse = 1.0 + sin(params.time * 3.6) * 0.15;
    let honeycomb_pattern = mix(cell_color, border_color, hex_border) * pulse;
    
    // Base stripe pre-multiplied color
    let base_alpha = stripe_mask * 0.82;
    let base_color = honeycomb_pattern * base_alpha;
    
    // -----------------------------------------------------
    // 3. Random Red Lightning strikes
    // -----------------------------------------------------
    let lightning = get_lightning(pos, params.time);
    
    // Composite base warning stripes and the lightning discharges
    let final_color = base_color + lightning.rgb;
    let final_alpha = max(base_alpha, lightning.a);
    
    return vec4<f32>(final_color, final_alpha);
}
