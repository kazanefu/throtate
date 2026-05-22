#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct SpaceParams {
    camera_pos: vec2<f32>,
    resolution: vec2<f32>,
    time: f32,
    scale_factor: f32,
}

@group(2) @binding(0)
var<uniform> params: SpaceParams;

const EPSILON: f32 = 1e-6;
const MIN_CELL_SIZE: f32 = 0.001;

// --------------------------------------------------
// Hash
// --------------------------------------------------

fn hash(p: vec2<f32>) -> f32 {
    let clamped_p = clamp(
        p,
        vec2<f32>(-10000.0),
        vec2<f32>(10000.0),
    );

    let dot_result = dot(
        clamped_p,
        vec2<f32>(127.1, 311.7),
    );

    let modded =
        dot_result
        - floor(dot_result / 6.28318530718)
        * 6.28318530718;

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

// --------------------------------------------------
// Large scale galaxy density variation
// --------------------------------------------------

fn galaxy_density(world: vec2<f32>) -> f32 {
    let g = floor(world * 0.0015);

    let a = hash(g);
    let b = hash(g + 17.0);
    let c = hash(g + 43.0);

    return (a + b + c) / 3.0;
}

// --------------------------------------------------
// Realistic star layer
// --------------------------------------------------

fn star_layer(
    world: vec2<f32>,
    cell_size: f32,
    threshold: f32,
) -> vec3<f32> {

    let safe_cell_size =
        max(cell_size, MIN_CELL_SIZE);

    let cell = floor(world / safe_cell_size);

    let rnd = hash(cell);

    // --------------------------------------------------
    // Galaxy density
    // --------------------------------------------------

    let density =
        galaxy_density(world);

    let adjusted_threshold =
        threshold
        + (0.5 - density) * 0.015;

    if rnd < adjusted_threshold {
        return vec3<f32>(0.0);
    }

    // --------------------------------------------------
    // Star position
    // --------------------------------------------------

    let star_pos =
        (hash2(cell) - 0.5)
        * safe_cell_size
        * 0.9;

    let center =
        cell * safe_cell_size
        + star_pos;

    let local = world - center;

    let d = length(local);

    // --------------------------------------------------
    // Realistic star size
    // Most stars tiny
    // Few stars large
    // --------------------------------------------------

    let radius =
        0.08
        + pow(rnd, 12.0) * 1.4;

    // --------------------------------------------------
    // Gaussian-like falloff
    // --------------------------------------------------

    let core =
        exp(-(d * d) / max(radius, EPSILON));

    let glow =
        exp(-(d * d) / max(radius * 6.0, EPSILON));

    // --------------------------------------------------
    // Realistic star colors
    // Mostly white
    // --------------------------------------------------

    let temp = hash(cell + 91.7);

    var star_color = vec3<f32>(1.0);

    if temp < 0.1 {
        // blue-white
        star_color = vec3<f32>(
            0.55,
            0.72,
            1.35,
        );
    } else if temp < 0.9 {
        // white
        star_color = vec3<f32>(
            1.1,
            1.1,
            1.1,
        );
    } else if temp < 0.95 {
        // yellow
        star_color = vec3<f32>(
            1.35,
            1.15,
            0.55,
        );
    } else {
        // red
        star_color = vec3<f32>(
            1.35,
            0.45,
            0.4,
        );
    }

    // --------------------------------------------------
    // Realistic brightness distribution
    // Most stars dim
    // --------------------------------------------------

    let brightness =
        pow(rnd, 10.0) * 14.0;

    // --------------------------------------------------
    // Slight twinkle
    // --------------------------------------------------

    let twinkle =
        0.97
        + sin(
            params.time * 2.0
            + rnd * 100.0
        ) * 0.03;

    // --------------------------------------------------
    // Final light
    // --------------------------------------------------

    let final_light =
        core
        + glow * 0.02;

    return
        star_color
        * brightness
        * final_light
        * twinkle;
}

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {

    // --------------------------------------------------
    // Base space color
    // Slight blue atmospheric tint
    // --------------------------------------------------

    let uv = in.position.xy / params.resolution;

    var color = mix(
        vec3<f32>(0.0002, 0.002, 0.01),
        vec3<f32>(0.0003, 0.006, 0.03),
        uv.y,
    );

    // --------------------------------------------------
    // Parallax control
    //
    // 0.0 = fixed to camera
    // lower = farther away
    // --------------------------------------------------

    let near_parallax = 0.2;
    let mid_parallax  = 0.05;
    let far_parallax  = 0.01;

    // --------------------------------------------------
    // Layer worlds
    // Only parallax changes
    // Star scale independent
    // --------------------------------------------------

    let near_world =
        in.world_position.xy
        * params.scale_factor
        + params.camera_pos
        * (near_parallax - 1.0);

    let mid_world =
        in.world_position.xy
        * params.scale_factor
        + params.camera_pos
        * (mid_parallax - 1.0);

    let far_world =
        in.world_position.xy
        * params.scale_factor
        + params.camera_pos
        * (far_parallax - 1.0);

    // --------------------------------------------------
    // Layers
    // --------------------------------------------------

    // Bright rare stars
    color += star_layer(
        near_world,
        42.0,
        0.9993,
    );

    // Medium stars
    color += star_layer(
        mid_world,
        15.0,
        0.995,
    );

    // Dense tiny stars
    color += star_layer(
        far_world,
        10.0,
        0.995,
    );

    color = clamp(
        color,
        vec3<f32>(0.0),
        vec3<f32>(50.0),
    );

    return vec4<f32>(color, 1.0);
}
