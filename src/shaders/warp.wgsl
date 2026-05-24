#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct WarpParams {
    time: f32,
}

@group(2) @binding(0)
var<uniform> params: WarpParams;

// Hash function for pseudo-random values
fn hash(p: vec2<f32>) -> f32 {
    var h = fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453123);
    return h;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let pos = vec2<f32>(uv.x - 0.5, 0.5 - uv.y);

    // Mirror coordinates around the center for symmetrical design
    let p = abs(pos) * 2.0;

    let fw = fwidth(p.x);
    let transition = max(fw, 0.005);

    // -----------------------------------------------------
    // 1. Corner brackets "「" / "」"
    // -----------------------------------------------------
    let in_outer = smoothstep(1.0 + transition, 1.0 - transition, p.x) *
                   smoothstep(1.0 + transition, 1.0 - transition, p.y);

    let L_norm = 0.32;
    let T_norm = 0.05;

    let horiz = smoothstep(1.0 - T_norm - transition, 1.0 - T_norm + transition, p.y) *
                smoothstep(1.0 - L_norm - transition, 1.0 - L_norm + transition, p.x);
    let vert = smoothstep(1.0 - T_norm - transition, 1.0 - T_norm + transition, p.x) *
               smoothstep(1.0 - L_norm - transition, 1.0 - L_norm + transition, p.y);
    let bracket = max(horiz, vert) * in_outer;

    // Outer glow for the brackets
    let T_glow = T_norm * 2.8;
    let L_glow = L_norm + T_norm;
    let horiz_glow = smoothstep(1.0 - T_glow - transition * 4.0, 1.0 - T_glow + transition * 4.0, p.y) *
                     smoothstep(1.0 - L_glow - transition * 4.0, 1.0 - L_glow + transition * 4.0, p.x);
    let vert_glow = smoothstep(1.0 - T_glow - transition * 4.0, 1.0 - T_glow + transition * 4.0, p.x) *
                    smoothstep(1.0 - L_glow - transition * 4.0, 1.0 - L_glow + transition * 4.0, p.y);
    let glow_mask = max(horiz_glow, vert_glow) * in_outer;

    // -----------------------------------------------------
    // 2. Spiral warp effect
    // -----------------------------------------------------
    let dist = length(pos);
    let angle = atan2(pos.y, pos.x);

    // Rotating spiral pattern
    let spiral_coord = dist * 12.0 - angle * 3.0 - params.time * 3.5;
    let spiral_pattern = smoothstep(0.35, 0.15, abs(fract(spiral_coord) - 0.5));

    // Fade out spiral at edges
    let spiral_mask = smoothstep(0.48, 0.1, dist);
    let spiral = spiral_pattern * spiral_mask;

    // -----------------------------------------------------
    // 3. Pulsing rings
    // -----------------------------------------------------
    let ring_coord = dist * 8.0 + params.time * 2.0;
    let ring_line = smoothstep(0.15, 0.05, abs(fract(ring_coord) - 0.5));
    let ring_glow = smoothstep(0.40, 0.0, abs(fract(ring_coord) - 0.5)) * 0.5;
    let rings = (ring_line + ring_glow) * smoothstep(0.45, 0.1, dist);

    // -----------------------------------------------------
    // 4. Particle/star field effect
    // -----------------------------------------------------
    let grid_size = 20.0;
    let grid_uv = uv * grid_size;
    let grid_id = floor(grid_uv);
    let grid_local = fract(grid_uv);

    let particle_random = hash(grid_id + vec2<f32>(params.time * 0.5));
    let particle_pos = vec2<f32>(hash(grid_id), hash(grid_id + vec2<f32>(100.0)));
    let particle_dist = length(grid_local - particle_pos);
    let particle_size = 0.02 + particle_random * 0.03;
    let particle = smoothstep(particle_size, 0.0, particle_dist) * particle_random;

    // -----------------------------------------------------
    // 5. Central vortex
    // -----------------------------------------------------
    let vortex_intensity = smoothstep(0.25, 0.0, dist);
    let vortex_rotation = angle * 6.0 - params.time * 4.0;
    let vortex_pattern = sin(vortex_rotation) * 0.5 + 0.5;
    let vortex = vortex_intensity * vortex_pattern;

    // -----------------------------------------------------
    // 6. Color composition & Scanlines
    // -----------------------------------------------------
    let purple_color = vec3<f32>(0.6, 0.2, 1.0);
    let blue_color = vec3<f32>(0.2, 0.5, 1.0);

    // Background space
    let bg_color = vec3<f32>(0.05, 0.04, 0.08);

    // Mix purple and blue based on distance and time
    let color_mix = sin(dist * 5.0 - params.time * 2.0) * 0.5 + 0.5;
    let warp_color = mix(purple_color, blue_color, color_mix);

    // Combine all effects
    let spiral_effect = spiral * warp_color * 1.2;
    let ring_effect = rings * warp_color * 1.5;
    let vortex_effect = vortex * warp_color * 0.8;
    let particle_effect = particle * warp_color * 2.0;

    var composed = bg_color + spiral_effect + ring_effect + vortex_effect + particle_effect;

    // Holographic scanlines
    let scanline = 0.94 + sin(uv.y * 150.0 + params.time * 7.0) * 0.06;
    composed = composed * scanline;

    // Pulse animation for the glowing corners
    let pulse = 1.0 + sin(params.time * 4.5) * 0.2;

    // Apply glowing brackets
    let color_with_glow = mix(composed, warp_color * 1.5, glow_mask * 0.38 * pulse);
    let final_color = mix(color_with_glow, warp_color * 2.0, bracket);

    return vec4<f32>(final_color, 1.0);
}
