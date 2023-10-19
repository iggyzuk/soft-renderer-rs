pub use linear_algebra::matrix::Matrix4;
pub use linear_algebra::vector::Vector4;
pub use linear_algebra::quaternion::Quaternion;

pub mod linear_algebra;

pub const PI: f32 = 3.14159265;

pub fn clamp(value: f32, lower: f32, upper: f32) -> f32 {
    if value <= lower {
        lower
    } else if value >= upper {
        upper
    } else {
        value
    }
}

pub fn lerp(a: f32, b: f32, factor: f32) -> f32 {
    a * (1.0 - factor) + b * factor
}
