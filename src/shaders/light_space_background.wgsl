#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct SpaceParams {
    camera_pos: vec2<f32>,
    resolution: vec2<f32>,
    time: f32,
    _padding: f32,
}

@group(2) @binding(0)
var<uniform> params: SpaceParams;

// Hash functions for procedural stars
fn hash(p: vec2<f32>) -> f32 {
    return fract(
        sin(dot(p, vec2<f32>(127.1, 311.7)))
        * 43758.5453123
    );
}

fn hash2(p: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(
        hash(p),
        hash(p + 17.13),
    );
}

// Lightweight star layer rendering
fn star_layer(
    world: vec2<f32>,
    cell_size: f32,
    threshold: f32,
) -> vec3<f32> {
    let cell = floor(world / cell_size);
    let rnd = hash(cell);

    if rnd < threshold {
        return vec3<f32>(0.0);
    }

    // Star position offset in cell
    let star_pos = (hash2(cell) - 0.5) * cell_size * 0.30;
    let center = cell * cell_size + star_pos;
    let local = world - center;
    let d = length(local);

    // Star radius
    let radius = 0.45 + rnd * 1.1;

    // Core light
    let core = 1.0 - smoothstep(0.0, radius, d);

    // Glow light
    let glow = 1.0 - smoothstep(radius, radius * 1.5, d);

    // Star color temperature
    let temp = hash(cell + 91.7);
    var star_color = vec3<f32>(1.0);

    if temp < 0.25 {
        star_color = vec3<f32>(0.55, 0.9, 2.0); // Blue-white
    } else if temp < 0.5 {
        star_color = vec3<f32>(1.8, 1.8, 1.8); // White
    } else if temp < 0.75 {
        star_color = vec3<f32>(2.0, 1.6, 0.45); // Yellow
    } else {
        star_color = vec3<f32>(2.0, 0.45, 0.45); // Red
    }

    let brightness = 1.5 + rnd * 2.2;
    let final_light = core + glow * 0.18;

    return star_color * brightness * final_light;
}

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    let screen_uv = vec2<f32>(
        (in.position.x / params.resolution.x) - 0.5,
        0.5 - (in.position.y / params.resolution.y)
    );
    let world = params.camera_pos + screen_uv * params.resolution;

    // Dark space base color
    var color = vec3<f32>(0.0, 0.0, 0.015);

    // 3 layers of stars for parallax effect:
    // Near layer
    color += star_layer(world, 18.0, 0.982);
    // Mid layer (moves at 30% speed)
    color += star_layer(world * 0.3, 28.0, 0.988);
    // Far layer (moves at 10% speed)
    color += star_layer(world * 0.1, 45.0, 0.993);

    return vec4<f32>(color, 1.0);
}
