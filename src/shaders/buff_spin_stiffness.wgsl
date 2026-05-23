#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct BuffSpinStiffnessUniform {
    time: f32,
    color: vec4<f32>,
}

@group(2) @binding(0)
var<uniform> params: BuffSpinStiffnessUniform;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let pos = vec2<f32>(uv.x - 0.5, 0.5 - uv.y);
    let p = abs(pos) * 2.0;

    let fw = fwidth(p.x);
    let transition = max(fw, 0.004);

    // Corner brackets
    let in_outer =
        smoothstep(1.0 + transition, 1.0 - transition, p.x) *
        smoothstep(1.0 + transition, 1.0 - transition, p.y);

    let L_norm = 0.32;
    let T_norm = 0.05;

    let horiz =
        smoothstep(1.0 - T_norm - transition, 1.0 - T_norm + transition, p.y) *
        smoothstep(1.0 - L_norm - transition, 1.0 - L_norm + transition, p.x);

    let vert =
        smoothstep(1.0 - T_norm - transition, 1.0 - T_norm + transition, p.x) *
        smoothstep(1.0 - L_norm - transition, 1.0 - L_norm + transition, p.y);

    let bracket = max(horiz, vert) * in_outer;

    let T_glow = T_norm * 2.8;
    let L_glow = L_norm + T_norm;

    let horiz_glow =
        smoothstep(1.0 - T_glow - transition * 4.0, 1.0 - T_glow + transition * 4.0, p.y) *
        smoothstep(1.0 - L_glow - transition * 4.0, 1.0 - L_glow + transition * 4.0, p.x);

    let vert_glow =
        smoothstep(1.0 - T_glow - transition * 4.0, 1.0 - T_glow + transition * 4.0, p.x) *
        smoothstep(1.0 - L_glow - transition * 4.0, 1.0 - L_glow + transition * 4.0, p.y);

    let glow_mask = max(horiz_glow, vert_glow) * in_outer;

    // Hexagonal grid pattern (Stiffness/Strength theme)
    let hex_scale = 8.0;
    let hex_pos = pos * hex_scale;
    let q = vec2<f32>(hex_pos.x * 1.732, hex_pos.y);
    let r = vec2<f32>(hex_pos.x * 0.866 + hex_pos.y * 1.5, -hex_pos.x * 0.866 + hex_pos.y * 1.5);

    let hex_dist = max(abs(q.x), max(abs(r.x), abs(r.y)));
    let hex_edge = fract(hex_dist) - 0.5;
    let hex_pattern = smoothstep(0.08, 0.05, abs(hex_edge)) * 0.4;

    // Pulse in phase with time
    let hex_pulse = hex_pattern * (0.7 + 0.3 * sin(params.time * 3.0));

    // Background
    let base_color = params.color.rgb;
    let bg = base_color * 0.06 + vec3<f32>(0.01, 0.01, 0.02);

    // Scanlines
    let scanline = 0.95 + sin(uv.y * 180.0 + params.time * 6.0) * 0.05;

    // Pulse animation
    let pulse = 1.0 + sin(params.time * 4.0) * 0.15;

    // Compose
    var color = bg + base_color * hex_pulse;
    color = mix(color, base_color * 1.4, glow_mask * 0.35 * pulse);
    color = mix(color, base_color * 2.0, bracket);
    color *= scanline;

    return vec4<f32>(color, 1.0);
}
