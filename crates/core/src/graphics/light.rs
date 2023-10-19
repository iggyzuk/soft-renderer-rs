use crate::math::linear_algebra::matrix::Matrix4;

use super::bitmap::Bitmap;

pub struct Light {
    pub projection: Matrix4,
    pub transform: Matrix4,
    pub bitmap: Bitmap, // todo: use 1d format!
}

impl Light {
    pub fn new(projection: Matrix4, transform: Matrix4, depth: Bitmap) -> Self {
        return Self {
            projection,
            transform,
            bitmap: depth,
        };
    }
}
