#import bevy_sprite::mesh2d_vertex_output::VertexOutput

fn hash(p: vec2<f32>) -> vec2<f32> {
    return fract(sin(vec2<f32>(
        dot(p, vec2<f32>(127.1, 311.7)),
        dot(p, vec2<f32>(269.5, 183.3))
    )) * 43758.5453);
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
    
    // Base rock texture detail using simple hash noise
    let noise_scale = uv * 12.0;
    let n = hash(floor(noise_scale)).x;
    
    // Bright grey base color
    let base_brightness = 0.65 + n * 0.15;
    var color = vec3<f32>(base_brightness, base_brightness, base_brightness * 0.98);

    // Primary cracks
    let vor = voronoi(uv * 4.0);
    let edge_dist = vor.x;
    let crack_width = 0.035;
    let crack = 1.0 - smoothstep(0.0, crack_width, edge_dist);
    
    // Fine cracks
    let vor_fine = voronoi(uv * 10.0);
    let crack_fine = 1.0 - smoothstep(0.0, 0.015, vor_fine.x);

    // Crack color (very dark grey)
    let crack_color = vec3<f32>(0.12, 0.12, 0.12);
    
    // Highlights along crack edges to add depth
    let highlight = smoothstep(crack_width, crack_width + 0.02, edge_dist);
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
