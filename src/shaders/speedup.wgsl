#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct SpeedupUniform {
    time: f32,
    base_color: vec4<f32>,
}

@group(2) @binding(0)
var<uniform> params: SpeedupUniform;

// -----------------------------------------------------
// Utility
// -----------------------------------------------------

fn line_segment(
    p: vec2<f32>,
    a: vec2<f32>,
    b: vec2<f32>,
    thickness: f32
) -> f32 {
    let pa = p - a;
    let ba = b - a;

    let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    let d = length(pa - ba * h);

    return smoothstep(thickness, thickness * 0.35, d);
}

fn arrow_shape(
    p: vec2<f32>,
    scale: f32
) -> f32 {
    let lp = p / scale;

    // 縦長 ↑
    let left = line_segment(
        lp,
        vec2<f32>(-0.11, -0.18),
        vec2<f32>(0.0, 0.20),
        0.024
    );

    let right = line_segment(
        lp,
        vec2<f32>(0.11, -0.18),
        vec2<f32>(0.0, 0.20),
        0.024
    );

    return max(left, right);
}

fn hash(n: f32) -> f32 {
    return fract(sin(n * 127.1) * 43758.5453123);
}

// -----------------------------------------------------
// Fragment
// -----------------------------------------------------

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    let pos = vec2<f32>(
        uv.x - 0.5,
        0.5 - uv.y
    );

    let p = abs(pos) * 2.0;

    let fw = fwidth(p.x);
    let transition = max(fw, 0.004);

    // -----------------------------------------------------
    // Corner brackets
    // -----------------------------------------------------

    let in_outer =
        smoothstep(1.0 + transition, 1.0 - transition, p.x) *
        smoothstep(1.0 + transition, 1.0 - transition, p.y);

    let L_norm = 0.30;
    let T_norm = 0.045;

    let horiz =
        smoothstep(
            1.0 - T_norm - transition,
            1.0 - T_norm + transition,
            p.y
        ) *
        smoothstep(
            1.0 - L_norm - transition,
            1.0 - L_norm + transition,
            p.x
        );

    let vert =
        smoothstep(
            1.0 - T_norm - transition,
            1.0 - T_norm + transition,
            p.x
        ) *
        smoothstep(
            1.0 - L_norm - transition,
            1.0 - L_norm + transition,
            p.y
        );

    let bracket = max(horiz, vert) * in_outer;

    // soft glow
    let glow =
        smoothstep(
            1.0 - 0.10,
            1.0 - 0.04,
            max(p.x, p.y)
        );

    // -----------------------------------------------------
    // Arrow groups
    // -----------------------------------------------------

    var arrows = 0.0;
    var trails = 0.0;

    let speed = 0.8;

    // 少数の列だけ
    for (var i = 0; i < 6; i = i + 1) {
        let fi = f32(i);

        // ランダム横位置
        let x =
            (hash(fi * 3.71) - 0.5) * 0.85;

        // ランダム開始位置
        let base_y =
            fract(
                hash(fi * 8.13) +
                params.time * speed
            ) - 0.5;

        let scale =
            0.20 +
            hash(fi * 5.91) * 0.04;

        // 縦に3つ固定
        for (var j = 0; j < 3; j = j + 1) {
            let fj = f32(j);

            let y =
                base_y -
                fj * 0.12;

            let local = vec2<f32>(
                pos.x - x,
                pos.y - y
            );

            let a =
                arrow_shape(
                    local,
                    scale
                );

            arrows = max(arrows, a);
        }

        // trailing line
        let trail_x =
            abs(pos.x - x);

        let trail_y =
            pos.y - base_y + 0.18;

        let trail =
            smoothstep(0.03, 0.0, trail_x) *
            smoothstep(0.30, 0.02, trail_y) *
            smoothstep(-0.02, 0.02, trail_y);

        trails = max(trails, trail);
    }

    // -----------------------------------------------------
    // Background
    // -----------------------------------------------------

    let base = params.base_color.rgb;

    let bg =
        base * 0.08 +
        vec3<f32>(0.01, 0.015, 0.02);

    // subtle scanline
    let scanline =
        0.97 +
        sin(uv.y * 220.0) * 0.03;

    // glow pulse
    let pulse =
        1.0 +
        sin(params.time * 4.5) * 0.05;

    let arrow_color =
        base *
        (arrows * 1.8 + trails * 0.55) *
        pulse;

    var color =
        bg +
        arrow_color;

    color =
        mix(
            color,
            base * 1.6,
            glow * 0.18
        );

    color =
        mix(
            color,
            base * 2.2,
            bracket
        );

    color *= scanline;

    return vec4<f32>(color, 1.0);
}
