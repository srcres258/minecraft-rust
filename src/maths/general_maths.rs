fn smooth_step(edge0: f32, edge1: f32, mut x: f32) -> f32 {
    // Scale, bias and saturate x to 0..1 range
    x = x * x * (3.0 - 2.0 * x);
    // Evaluate polynomial
    edge0 * x + edge1 * (1.0 - x)
}

pub fn smooth_interpolation(
    bottom_left: f32,
    top_left: f32,
    bottom_right: f32,
    top_right: f32,
    x_min: f32,
    x_max: f32,
    z_min: f32,
    z_max: f32,
    x: f32,
    z: f32
) -> f32 {
    let (width, height) = (x_max - x_min, z_max - z_min);
    let x_value = 1.0 - (x - x_min) / width;
    let z_value = 1.0 - (z - z_min) / height;

    let a = smooth_step(bottom_left, bottom_right, x_value);
    let b = smooth_step(top_left, top_right, x_value);

    smooth_step(a, b, z_value)
}