#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct CheckpointParams {
    time: f32,
}

@group(2) @binding(0)
var<uniform> params: CheckpointParams;

fn get_pin_mask(pos: vec2<f32>, fw: f32) -> f32 {
    let px = abs(pos.x);
    let py = pos.y;
    
    // Top boundary of the Y-wings
    let top_y = 0.16 + 0.19 * pow(px / 0.26, 2.0);
    let in_top = smoothstep(top_y + fw, top_y - fw, py);
    
    // Bottom tip boundary
    let in_bottom = smoothstep(-0.20 - fw, -0.20 + fw, py);
    
    // Outer flared boundary
    let px_outer = 0.26 * pow((py + 0.20) / 0.55, 1.5);
    let in_outer = smoothstep(px_outer + fw, px_outer - fw, px);
    
    // Inner V-shaped cutout
    var in_inner = 1.0;
    if (py >= 0.04) {
        let px_inner = 0.15 * pow((py - 0.04) / 0.31, 1.3);
        in_inner = smoothstep(px_inner - fw, px_inner + fw, px);
    }
    
    return in_top * in_bottom * in_outer * in_inner;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    // Map coordinate system such that X goes right (-0.5 to 0.5) and Y goes up (-0.5 to 0.5)
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
    // 2. Concentric rings at the bottom
    // -----------------------------------------------------
    let center_base = vec2<f32>(0.0, -0.28);
    let scale_base = vec2<f32>(0.26, 0.08);
    let d_base = length((pos - center_base) / scale_base);
    
    let ring1 = 1.0 - smoothstep(0.0, 0.08, abs(d_base - 1.0));
    let ring2 = 1.0 - smoothstep(0.0, 0.08, abs(d_base - 0.65));
    let rings = max(ring1, ring2) * (1.0 - smoothstep(-0.1, 0.1, pos.y));

    // -----------------------------------------------------
    // 3. Y-Shaped Map Pin (floating animation)
    // -----------------------------------------------------
    let float_offset = sin(params.time * 2.5) * 0.035;
    let pin_pos = pos - vec2<f32>(0.0, float_offset);
    
    let pin_mask = get_pin_mask(pin_pos, transition);
    let pin_glow = get_pin_mask(pin_pos, transition * 8.0);

    // -----------------------------------------------------
    // 4. Color composition & Scanlines
    // -----------------------------------------------------
    let cyan_glow = vec3<f32>(0.1, 0.85, 1.0);
    
    // Background space
    let bg_color = vec3<f32>(0.04, 0.06, 0.08);
    
    // Emissive elements
    let rings_color = cyan_glow * rings * 1.25;
    let pin_color = mix(cyan_glow * 0.6, vec3<f32>(0.9, 0.98, 1.0), pin_mask) * (pin_mask + pin_glow * 0.45);
    
    // Combine base colors
    var composed = bg_color + rings_color + pin_color;
    
    // Holographic scanlines (running down over time)
    let scanline = 0.94 + sin(uv.y * 150.0 + params.time * 7.0) * 0.06;
    composed = composed * scanline;
    
    // Pulse animation for the glowing corners
    let pulse = 1.0 + sin(params.time * 4.0) * 0.18;
    
    // Apply glowing brackets
    let color_with_glow = mix(composed, cyan_glow * 1.5, glow_mask * 0.38 * pulse);
    let final_color = mix(color_with_glow, cyan_glow * 2.0, bracket);
    
    return vec4<f32>(final_color, 1.0);
}
