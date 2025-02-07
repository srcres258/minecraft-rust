pub mod vector2xz;
pub mod frustum;
pub mod noise_generator;
pub mod ray;
pub mod matrix;

/// @brief Clamp function that regulates values between limits.
/// @param x
/// @param lowerlimit
/// @param upperlimit
/// @return x
fn clamp(mut x: f32, lower_limit: f32, upper_limit: f32) -> f32 {
    if x < lower_limit {
        x = lower_limit;
    }
    if x > upper_limit {
        x = upper_limit;
    }
    x
}

fn smooth_step(edge0: f32, edge1: f32, mut x: f32) -> f32 {
    // Scale, bias and saturate x to 0..1 range
    x = x * x * (3. - 2. * x);
    // Evaluate polynomial
    edge0 * x + edge1 * (1. - x)
}

pub fn bilinear_interpolation(
    bottom_left: f32, top_left: f32,
    bottom_right: f32, top_right: f32,
    x_min: f32, x_max: f32,
    z_min: f32, z_max: f32,
    x: f32, z: f32
) -> f32 {
    let width = x_max - x_min;
    let height = z_max - z_min;
    let x_distance_to_max_value = x_max - x;
    let z_distance_to_max_value = z_max - z;
    let x_distance_to_min_value = x - x_min;
    let z_distance_to_min_value = z - z_min;

    1.0 / (width * height) *
        (bottom_left * x_distance_to_max_value * z_distance_to_max_value +
        bottom_right * x_distance_to_min_value * z_distance_to_max_value +
        top_left * x_distance_to_max_value * z_distance_to_min_value +
        top_right * x_distance_to_min_value * z_distance_to_min_value)
}

pub fn smooth_interpolation(
    bottom_left: f32, top_left: f32,
    bottom_right: f32, top_right: f32,
    x_min: f32, x_max: f32,
    z_min: f32, z_max: f32,
    x: f32, z: f32
) -> f32 {
    let width = x_max - x_min;
    let height = z_max - z_min;
    let x_value = 1. - (x - x_min) / width;
    let z_value = 1. - (z - z_min) / height;

    let a = smooth_step(bottom_left, bottom_right, x_value);
    let b = smooth_step(top_left, top_right, x_value);
    smooth_step(a, b, z_value)
}