struct FragmentInput {
    [[builtin(front_facing)]] is_front: bool;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};
struct ColorMaterial {
    color: vec4<f32>;
};
[[group(1), binding(0)]]
var<uniform> material: ColorMaterial;

fn draw_circle_hard(coord: vec2<f32>, radius: f32) -> f32 {
    return step(length(coord),radius);
}

[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    let c = draw_circle_hard((in.uv / 2.0) - vec2<f32>(0.25, 0.25), 0.25);
    if (c <= 0.0) {
        discard;
    }
    let c = material.color * c;
    return vec4<f32>(c);
}