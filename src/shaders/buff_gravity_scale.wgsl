#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct BuffGravityScaleUniform {
    time: f32,
    color: vec4<f32>,
}

@group(2) @binding(0)
var<uniform> params: BuffGravityScaleUniform;

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

    // Falling particles pattern (Gravity theme)
    var particles = 0.0;
    for (var i = 0; i < 6; i = i + 1) {
        let fi = f32(i);
        let offset_x = sin(fi * 2.37) * 0.4;
        let fall_speed = 0.3 + sin(fi * 1.83) * 0.15;
        let particle_y = fract((pos.y + 0.5) + params.time * fall_speed + fi * 0.2) - 0.5;
        let particle_pos = vec2<f32>(pos.x - offset_x, particle_y);

        let dist = length(particle_pos);
        let particle = smoothstep(0.08, 0.02, dist);

        // Trail effect
        let trail_y = pos.y - (particle_y - offset_x * 0.1);
        let trail = smoothstep(0.03, 0.0, abs(pos.x - offset_x)) *
                   smoothstep(0.0, 0.15, trail_y) *
                   smoothstep(0.25, 0.15, trail_y);

        particles = max(particles, particle + trail * 0.3);
    }

    // Background
    let base_color = params.color.rgb;
    let bg = base_color * 0.06 + vec3<f32>(0.01, 0.01, 0.02);

    // Scanlines
    let scanline = 0.95 + sin(uv.y * 180.0 + params.time * 6.0) * 0.05;

    // Pulse animation
    let pulse = 1.0 + sin(params.time * 4.0) * 0.15;

    // Compose
    var color = bg + base_color * particles * 0.6;
    color = mix(color, base_color * 1.4, glow_mask * 0.35 * pulse);
    color = mix(color, base_color * 2.0, bracket);
    color *= scanline;

    return vec4<f32>(color, 1.0);
}
