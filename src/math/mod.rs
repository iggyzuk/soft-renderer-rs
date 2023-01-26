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
