#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct SpaceParams {
    camera_pos: vec2<f32>,
    resolution: vec2<f32>,
    time: f32,
    scale_factor: f32,
}

@group(2) @binding(0)
var<uniform> params: SpaceParams;

// Constants to prevent edge cases
const EPSILON: f32 = 1e-6;
const MIN_CELL_SIZE: f32 = 0.001;

// Hash functions for procedural stars
fn hash(p: vec2<f32>) -> f32 {
    // Clamp input to prevent extreme values
    let clamped_p = clamp(p, vec2<f32>(-10000.0), vec2<f32>(10000.0));
    let dot_result = dot(clamped_p, vec2<f32>(127.1, 311.7));
    // Use mod to prevent very large values in sin
    let modded = dot_result - floor(dot_result / 6.28318530718) * 6.28318530718;
    return fract(
        sin(modded) * 43758.5453123
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
    // Prevent zero division
    let safe_cell_size = max(cell_size, MIN_CELL_SIZE);

    let cell = floor(world / safe_cell_size);
    let rnd = hash(cell);

    if rnd < threshold {
        return vec3<f32>(0.0);
    }

    // Star position offset in cell
    let star_pos = (hash2(cell) - 0.5) * safe_cell_size * 0.30;
    let center = cell * safe_cell_size + star_pos;
    let local = world - center;
    let d = length(local);

    // Star radius
    let radius = max(0.45 + rnd * 1.1, EPSILON);

    // Core light
    let core = 1.0 - smoothstep(0.0, radius, d);

    // Glow light - ensure glow_radius > radius
    let glow_radius = max(radius * 1.5, radius + EPSILON);
    let glow = 1.0 - smoothstep(radius, glow_radius, d);

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
    let world = (in.world_position.xy - params.camera_pos) * 1.5 + params.camera_pos;

    // Dark space base color
    var color = vec3<f32>(0.0, 0.0, 0.015);

    // 3 layers of stars for parallax effect:
    // Near
    color += star_layer(world * 0.5, 18.0, 0.982);

    // Mid
    color += star_layer(world * 0.15, 28.0, 0.988);

    // Far
    color += star_layer(world * 0.03, 45.0, 0.993);

    // Ensure no NaN or Inf in final output
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(100.0));

    return vec4<f32>(color, 1.0);
}
