#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct BuffRestitutionUniform {
    time: f32,
    color: vec4<f32>,
}

@group(2) @binding(0)
var<uniform> params: BuffRestitutionUniform;

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

    // Expanding circles pattern (Restitution/Bounce theme)
    var circles = 0.0;
    for (var i = 0; i < 4; i = i + 1) {
        let fi = f32(i);
        let phase = params.time * 2.0 + fi * 1.57;
        let expand = fract(phase * 0.5);

        let radius = expand * 0.5;
        let dist = length(pos);
        let circle_edge = abs(dist - radius);

        let circle = smoothstep(0.06, 0.02, circle_edge) *
                    (1.0 - expand) * 0.5;
        circles = max(circles, circle);
    }

    // Ripple effect from center
    let center_dist = length(pos);
    let ripple = sin(center_dist * 15.0 - params.time * 8.0);
    let ripple_mask = smoothstep(0.4, 0.1, center_dist);
    let ripple_pattern = (ripple * 0.5 + 0.5) * ripple_mask * 0.2;

    // Background
    let base_color = params.color.rgb;
    let bg = base_color * 0.06 + vec3<f32>(0.01, 0.01, 0.02);

    // Scanlines
    let scanline = 0.95 + sin(uv.y * 180.0 + params.time * 6.0) * 0.05;

    // Pulse animation
    let pulse = 1.0 + sin(params.time * 4.0) * 0.15;

    // Compose
    var color = bg + base_color * (circles + ripple_pattern);
    color = mix(color, base_color * 1.4, glow_mask * 0.35 * pulse);
    color = mix(color, base_color * 2.0, bracket);
    color *= scanline;

    return vec4<f32>(color, 1.0);
}
