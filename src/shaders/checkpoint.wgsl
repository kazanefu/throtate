#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct CheckpointParams {
    time: f32,
}

@group(2) @binding(0)
var<uniform> params: CheckpointParams;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    
    // Mirror coordinates around the center to handle all four corners at once
    let p = abs(uv - vec2<f32>(0.5)) * 2.0;
    
    let fw = fwidth(p.x);
    let transition = max(fw, 0.005);
    
    // Outer boundaries check
    let in_outer = smoothstep(1.0 + transition, 1.0 - transition, p.x) * 
                   smoothstep(1.0 + transition, 1.0 - transition, p.y);
                   
    // Corner brackets "「" / "」" size and thickness parameters
    let L_norm = 0.32;
    let T_norm = 0.05;
    
    // Horizontal and vertical sharp bracket elements
    let horiz = smoothstep(1.0 - T_norm - transition, 1.0 - T_norm + transition, p.y) *
                smoothstep(1.0 - L_norm - transition, 1.0 - L_norm + transition, p.x);
    let vert = smoothstep(1.0 - T_norm - transition, 1.0 - T_norm + transition, p.x) *
               smoothstep(1.0 - L_norm - transition, 1.0 - L_norm + transition, p.y);
    let bracket = max(horiz, vert) * in_outer;
    
    // Outer glow for the brackets (wider, softer)
    let T_glow = T_norm * 2.8;
    let L_glow = L_norm + T_norm;
    let horiz_glow = smoothstep(1.0 - T_glow - transition * 4.0, 1.0 - T_glow + transition * 4.0, p.y) *
                     smoothstep(1.0 - L_glow - transition * 4.0, 1.0 - L_glow + transition * 4.0, p.x);
    let vert_glow = smoothstep(1.0 - T_glow - transition * 4.0, 1.0 - T_glow + transition * 4.0, p.x) *
                    smoothstep(1.0 - L_glow - transition * 4.0, 1.0 - L_glow + transition * 4.0, p.y);
    let glow_mask = max(horiz_glow, vert_glow) * in_outer;
    
    // Flag mask in the center
    let flag_limit = 0.80;
    let flag_mask = smoothstep(flag_limit + transition, flag_limit - transition, max(p.x, p.y));
    
    // Checker flag logic
    // Deform coordinates with sine/cos waves over time to simulate waving in the wind
    let wave_x = sin(uv.y * 7.5 + params.time * 2.4) * 0.045;
    let wave_y = cos(uv.x * 7.5 + params.time * 2.4) * 0.045;
    let deformed_uv = uv + vec2<f32>(wave_x, wave_y);
    
    let check_scale = 8.0;
    let check_pos = floor(deformed_uv * check_scale);
    let is_check = i32(check_pos.x + check_pos.y) % 2 == 0;
    
    // Colors
    let checker_white = vec3<f32>(0.96, 0.96, 0.98);
    let checker_blue = vec3<f32>(0.2, 0.72, 0.98);
    let base_checker = mix(checker_blue, checker_white, f32(is_check));
    
    // Dynamic lighting folds (shadows & highlights) based on wave derivative
    let wave_slope = cos(uv.y * 7.5 + params.time * 2.4);
    let shading = 0.85 + wave_slope * 0.15;
    var flag_color = base_checker * shading;
    
    // Scanlines to make it look like a holographic projection
    let scanline = 0.92 + sin(uv.y * 140.0 + params.time * 8.0) * 0.08;
    flag_color = flag_color * scanline;
    
    // Background space
    let bg_color = vec3<f32>(0.06, 0.08, 0.1);
    
    // Compose base flag + background
    let composed_base = mix(bg_color, flag_color, flag_mask);
    
    // Pulse animation for the glowing cyan corners
    let pulse = 1.0 + sin(params.time * 3.8) * 0.2;
    let cyan_glow = vec3<f32>(0.1, 0.82, 1.0);
    
    let color_with_glow = mix(composed_base, cyan_glow * 1.6, glow_mask * 0.4 * pulse);
    let final_color = mix(color_with_glow, cyan_glow * 2.0, bracket);
    
    return vec4<f32>(final_color, 1.0);
}
