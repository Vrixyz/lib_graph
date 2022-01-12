struct ColorMaterial {
    color: vec4<f32>;
};
[[group(1), binding(0)]]
var<uniform> material: ColorMaterial;

[[stage(fragment)]]
fn fragment() -> [[location(0)]] vec4<f32> {
    let white = vec3<f32>(1.0, 1.0, 0.0);
    return vec4<f32>(white, 1.0);
//    return material.color;
}