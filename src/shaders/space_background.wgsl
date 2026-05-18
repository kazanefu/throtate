#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct SpaceParams {
    camera_pos: vec2<f32>,
    resolution: vec2<f32>,
    time: f32,
    _padding: f32,
}

@group(2) @binding(0)
var<uniform> params: SpaceParams;

// --------------------------------------------------
// hash
// --------------------------------------------------

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

// --------------------------------------------------
// noise
// --------------------------------------------------

fn noise(p: vec2<f32>) -> f32 {

    let i = floor(p);
    let f = fract(p);

    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));

    let u = f * f * (3.0 - 2.0 * f);

    return
        mix(a, b, u.x)
        + (c - a) * u.y * (1.0 - u.x)
        + (d - b) * u.x * u.y;
}

fn fbm(p: vec2<f32>) -> f32 {

    var value = 0.0;
    var amp = 0.5;
    var freq = 1.0;

    for (var i = 0; i < 5; i++) {

        value += noise(p * freq) * amp;

        freq *= 2.0;
        amp *= 0.5;
    }

    return value;
}

// --------------------------------------------------
// stable star layer
// --------------------------------------------------

fn star_layer(
    world: vec2<f32>,
    cell_size: f32,
    threshold: f32,
) -> vec3<f32> {

    let cell =
        floor(world / cell_size);

    let rnd =
        hash(cell);

    if rnd < threshold {
        return vec3<f32>(0.0);
    }

    // ==================================================
    // 星位置
    // ==================================================

    // セル中心付近だけ
    let star_pos =
        (hash2(cell) - 0.5)
        * cell_size
        * 0.30;

    let center =
        cell * cell_size + star_pos;

    let local =
        world - center;

    let d =
        length(local);

    // ==================================================
    // サイズ
    // ==================================================

    // 小さくする
    let radius =
        0.45 + rnd * 1.1;

    // ==================================================
    // core
    // ==================================================

    let core =
        1.0
        - smoothstep(
            0.0,
            radius,
            d
        );

    // ==================================================
    // glow
    // ==================================================

    // glowをかなり縮小
    let glow_radius =
        radius * 1.0;

    let glow =
        1.0
        - smoothstep(
            radius,
            glow_radius,
            d
        );

    // ==================================================
    // 光条
    // ==================================================

    let cross_width =
        radius * 0.08;

    let cross_x =
        1.0
        - smoothstep(
            0.0,
            cross_width,
            abs(local.x)
        );

    let cross_y =
        1.0
        - smoothstep(
            0.0,
            cross_width,
            abs(local.y)
        );

    let sparkle =
        max(cross_x, cross_y)
        * glow
        * 1.22;

    // ==================================================
    // 明るさ
    // ==================================================

    let brightness =
        1.5 + rnd * 2.2;

    // ==================================================
    // 色
    // ==================================================

    let temp =
        hash(cell + 91.7);

    var star_color =
        vec3<f32>(1.0);

    // 青白
    if temp < 0.25 {

        star_color =
            vec3<f32>(0.55, 0.9, 2.0);
    }

    // 白
    else if temp < 0.5 {

        star_color =
            vec3<f32>(1.8, 1.8, 1.8);
    }

    // 黄
    else if temp < 0.75 {

        star_color =
            vec3<f32>(2.0, 1.6, 0.45);
    }

    // 赤
    else {

        star_color =
            vec3<f32>(2.0, 0.45, 0.45);
    }

    // ==================================================
    // 最終
    // ==================================================

    let final_light =
        core
        + glow * 0.18
        + sparkle;

    return
        star_color
        * brightness
        * final_light;
}
// --------------------------------------------------
// planets
// --------------------------------------------------

fn render_planets(
    world: vec2<f32>
) -> vec3<f32> {

    let planet_grid = 2000.0;

    let cell =
        floor(world / planet_grid);

    var color =
        vec3<f32>(0.0);

    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {

            let current =
                cell
                + vec2<f32>(
                    f32(x),
                    f32(y)
                );

            let seed =
                hash(current);

            // 出現率
            if seed < 0.35 {
                continue;
            }

            // パララックス
            let parallax =
                0.03 + seed * 0.12;

            let local_world =
                params.camera_pos * parallax
                + (world - params.camera_pos);

            let center =
                current * planet_grid
                + (hash2(current) - 0.5)
                * planet_grid
                * 0.7;

            let p =
                local_world - center;

            let radius =
                100.0 + seed * 450.0;

            let d =
                length(p);

            if d > radius {
                continue;
            }

            let n =
                normalize(p);

            let light_dir =
                normalize(
                    vec2<f32>(-1.0, -0.3)
                );

            let diffuse =
                0.25
                + max(
                    dot(n, -light_dir),
                    0.0
                );

            let atmosphere =
                1.0
                - smoothstep(
                    radius * 0.7,
                    radius,
                    d
                );

            // 模様
            let surface =
                clamp(
                    fbm(
                        p * 0.008
                        + seed * 100.0
                    ),
                    0.0,
                    1.0
                );

            // 惑星タイプ
            let type_seed =
                hash(current + 777.0);

            var color_a =
                vec3<f32>(0.2);

            var color_b =
                vec3<f32>(0.8);

            // 青
            if type_seed < 0.25 {

                color_a =
                    vec3<f32>(0.1, 0.2, 0.7);

                color_b =
                    vec3<f32>(0.8, 0.9, 1.0);
            }

            // 赤
            else if type_seed < 0.5 {

                color_a =
                    vec3<f32>(0.5, 0.1, 0.1);

                color_b =
                    vec3<f32>(1.0, 0.5, 0.3);
            }

            // 緑
            else if type_seed < 0.75 {

                color_a =
                    vec3<f32>(0.1, 0.4, 0.2);

                color_b =
                    vec3<f32>(0.7, 1.0, 0.5);
            }

            // 紫
            else {

                color_a =
                    vec3<f32>(0.3, 0.1, 0.5);

                color_b =
                    vec3<f32>(0.9, 0.6, 1.0);
            }

            let base =
                mix(
                    color_a,
                    color_b,
                    surface
                );

            // rim light
            let rim =
                pow(
                    1.0
                    - abs(dot(n, -light_dir)),
                    3.0
                );

            color +=
                base
                * diffuse
                * atmosphere
                * 1.2;

            color +=
                vec3<f32>(0.4, 0.7, 1.0)
                * rim
                * 0.3;
        }
    }

    return color;
}

// --------------------------------------------------
// fragment
// --------------------------------------------------

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {

    let screen_uv =
        (in.position.xy / params.resolution)
        - 0.5;

    let world =
        params.camera_pos
        + screen_uv
        * params.resolution;

    var color =
        vec3<f32>(
            0.0,
            0.0,
            0.015
        );

    // nebula

    let nebula =
        fbm(world * 0.00025);

    color +=
        vec3<f32>(
            0.12,
            0.04,
            0.2
        )
        * nebula
        * 0.18;

    // stars

    // 近景
    color += star_layer(
        world,
        18.0,
        0.982
    );

    // 中景
    color += star_layer(
        world * 0.3,
        28.0,
        0.988
    );

    // 遠景
    color += star_layer(
        world * 0.1,
        45.0,
        0.993
    );

    // planets

    color += render_planets(world);

    return vec4<f32>(color, 1.0);
}
