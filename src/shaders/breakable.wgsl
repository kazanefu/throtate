#import bevy_sprite::mesh2d_vertex_output::VertexOutput

fn hash(p: vec2<f32>) -> vec2<f32> {
    return fract(sin(vec2<f32>(
        dot(p, vec2<f32>(127.1, 311.7)),
        dot(p, vec2<f32>(269.5, 183.3))
    )) * 43758.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    
    // Cubic Hermite interpolation curve
    let u = f * f * (3.0 - 2.0 * f);
    
    return mix(
        mix(hash(i + vec2<f32>(0.0, 0.0)).x, hash(i + vec2<f32>(1.0, 0.0)).x, u.x),
        mix(hash(i + vec2<f32>(0.0, 1.0)).x, hash(i + vec2<f32>(1.0, 1.0)).x, u.x),
        u.y
    );
}

fn voronoi(p: vec2<f32>) -> vec3<f32> {
    let n = floor(p);
    let f = fract(p);

    var mg = vec2<f32>(0.0, 0.0);
    var mr = vec2<f32>(0.0, 0.0);

    var md = 8.0;
    for (var j: i32 = -1; j <= 1; j = j + 1) {
        for (var i: i32 = -1; i <= 1; i = i + 1) {
            let g = vec2<f32>(f32(i), f32(j));
            let o = hash(n + g);
            let r = g + o - f;
            let d = dot(r, r);

            if (d < md) {
                md = d;
                mr = r;
                mg = g;
            }
        }
    }

    md = 8.0;
    for (var j: i32 = -2; j <= 2; j = j + 1) {
        for (var i: i32 = -2; i <= 2; i = i + 1) {
            let g = mg + vec2<f32>(f32(i), f32(j));
            let o = hash(n + g);
            let r = g + o - f;

            if (dot(mr - r, mr - r) > 0.00001) {
                md = min(md, dot(0.5 * (mr + r), normalize(r - mr)));
            }
        }
    }

    return vec3<f32>(md, mr);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    
    // Smooth bilinear value noise for the organic rock texture detail
    let n = noise(uv * 12.0);
    
    // Bright grey base color
    let base_brightness = 0.65 + n * 0.15;
    var color = vec3<f32>(base_brightness, base_brightness, base_brightness * 0.98);

    // Primary cracks with derivative anti-aliasing
    let vor = voronoi(uv * 4.0);
    let edge_dist = vor.x;
    let fw = fwidth(edge_dist);
    
    let crack_width = 0.035;
    let transition = max(fw, 0.004);
    let crack = 1.0 - smoothstep(crack_width - transition, crack_width + transition, edge_dist);
    
    // Fine cracks with derivative anti-aliasing
    let vor_fine = voronoi(uv * 10.0);
    let edge_dist_fine = vor_fine.x;
    let fw_fine = fwidth(edge_dist_fine);
    
    let crack_width_fine = 0.015;
    let transition_fine = max(fw_fine, 0.002);
    let crack_fine = 1.0 - smoothstep(crack_width_fine - transition_fine, crack_width_fine + transition_fine, edge_dist_fine);

    // Crack color (very dark grey)
    let crack_color = vec3<f32>(0.12, 0.12, 0.12);
    
    // Anti-aliased highlights along crack edges to add depth
    let highlight = smoothstep(crack_width, crack_width + transition * 2.0, edge_dist);
    let highlight_color = vec3<f32>(0.85, 0.85, 0.85);

    // Blend components
    color = mix(color, highlight_color, (1.0 - highlight) * 0.35);
    color = mix(color, crack_color, crack * 0.85);
    color = mix(color, crack_color, crack_fine * 0.55);

    // Vignette/darkening towards the borders of the quad
    let centered = uv - vec2<f32>(0.5);
    let border = smoothstep(0.42, 0.5, max(abs(centered.x), abs(centered.y)));
    color = mix(color, vec3<f32>(0.15, 0.15, 0.15), border * 0.6);

    return vec4<f32>(color, 1.0);
}
