#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct TransitionUniform {
    base_color : vec4<f32>,
    progress : f32,
    _padding : vec3<f32>,
};

@group(2) @binding(0)
var<uniform> material : TransitionUniform;

fn hash(
    p: vec2<f32>
) -> f32 {
    return fract(
        sin(
            dot(
                p,
                vec2<f32>(
                    127.1,
                    311.7
                )
            )
        )
        * 43758.5453123
    );
}

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {

    if (
        material.progress > 100.0
    ) {
        discard;
    }

    // Rectangle(10000,10000)
    // world_position が
    // -5000～5000
    let uv =
        in.world_position.xy
        / 10000.0
        + vec2<f32>(0.5);

    var color =
        vec3<f32>(0.0);

    var alpha =
        0.0;

    let count:u32 = 256u;

    for (
        var i:u32 = 0u;
        i < count;
        i++
    ) {

        let fi =
            f32(i);

        let seed =
            vec2<f32>(
                fi * 17.31,
                fi * 43.17,
            );

        let center =
            vec2<f32>(
                hash(seed),
                hash(seed + 1.0),
            );

        let start_size =
            0.05
            + hash(seed + 2.0)
            * 0.18;

        let shrink_speed =
            0.6
            + hash(seed + 3.0)
            * 1.4;

        let delay =
            hash(seed + 4.0)
            * 0.4;

        let t =
            max(
                0.0,
                material.progress
                - delay,
            );

        let size =
            start_size
            * max(
                0.0,
                1.0
                - t
                * shrink_speed,
            );

        if (
            size <= 0.0
        ) {
            continue;
        }

        let d =
            abs(
                uv
                - center,
            );

        if (
            d.x < size
            && d.y < size
        ) {

            let tint =
                (
                    hash(seed + 5.0)
                    - 0.5
                )
                * 0.25;

            let square_color =
                material.base_color.rgb
                + vec3<f32>(
                    tint,
                    tint,
                    tint,
                );

            let square_alpha =
                size
                / start_size;

            color +=
                square_color
                * square_alpha;

            alpha +=
                square_alpha
                * 0.08;
        }
    }

    color =
        min(
            color,
            vec3<f32>(1.0),
        );

    alpha =
        clamp(
            alpha,
            0.0,
            1.0,
        );

    return vec4<f32>(
        color,
        alpha,
    );
}
