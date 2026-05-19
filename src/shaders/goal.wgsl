#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct GoalParams {
    time: f32,
}

@group(2) @binding(0)
var<uniform> params: GoalParams;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let pos = vec2<f32>(uv.x - 0.5, 0.5 - uv.y);
    
    // Mirror coordinates around the center to handle the 4 corner brackets
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
    
    // Outer glow for the brackets (wider, softer)
    let T_glow = T_norm * 2.8;
    let L_glow = L_norm + T_norm;
    let horiz_glow = smoothstep(1.0 - T_glow - transition * 4.0, 1.0 - T_glow + transition * 4.0, p.y) *
                     smoothstep(1.0 - L_glow - transition * 4.0, 1.0 - L_glow + transition * 4.0, p.x);
    let vert_glow = smoothstep(1.0 - T_glow - transition * 4.0, 1.0 - T_glow + transition * 4.0, p.x) *
                    smoothstep(1.0 - L_glow - transition * 4.0, 1.0 - L_glow + transition * 4.0, p.y);
    let glow_mask = max(horiz_glow, vert_glow) * in_outer;

    // -----------------------------------------------------
    // 2. Concentric squares shrinking towards center
    // -----------------------------------------------------
    let d = max(abs(pos.x), abs(pos.y));
    let square_limit = 0.40;
    let inside_mask = smoothstep(square_limit + transition, square_limit - transition, d);
    
    // Animate squares moving/shrinking inwards
    let sq_coord = d * 10.0 + params.time * 2.8;
    let line_dist = abs(fract(sq_coord) - 0.5);
    
    let squares_line = smoothstep(0.12, 0.04, line_dist) * inside_mask;
    let squares_glow = smoothstep(0.35, 0.0, line_dist) * 0.4 * inside_mask;
    
    // -----------------------------------------------------
    // 3. Color composition & Scanlines
    // -----------------------------------------------------
    let pink_color = vec3<f32>(1.0, 0.08, 0.72);
    
    // Background space
    let bg_color = vec3<f32>(0.06, 0.05, 0.08);
    
    // Combine base colors
    let squares_glow_color = pink_color * (squares_line + squares_glow) * 1.35;
    var composed = bg_color + squares_glow_color;
    
    // Holographic scanlines
    let scanline = 0.94 + sin(uv.y * 150.0 + params.time * 6.5) * 0.06;
    composed = composed * scanline;
    
    // Pulse animation for the glowing corners
    let pulse = 1.0 + sin(params.time * 3.8) * 0.16;
    
    // Apply glowing brackets
    let color_with_glow = mix(composed, pink_color * 1.5, glow_mask * 0.38 * pulse);
    let final_color = mix(color_with_glow, pink_color * 2.0, bracket);
    
    return vec4<f32>(final_color, 1.0);
}
