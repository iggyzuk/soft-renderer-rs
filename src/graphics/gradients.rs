use std::ops::{Mul, Sub};

use crate::math::{clamp, linear_algebra::vector::Vector4};

use super::vertex::Vertex;

// todo: pass the light to the gradients?
pub const LIGHT_DIR: Vector4 = Vector4 {
    x: 0.1,
    y: 0.6,
    z: 0.3,
    w: 1.0,
};

#[derive(Debug)]
pub struct Gradients {
    pub texcoords: Gradient<Vector4>,
    pub one_over_z: Gradient<f32>,
    pub depth: Gradient<f32>,
    pub light_amt: Gradient<f32>,
}

// 0 .
//  .     .
//   .       1
//    .    .
//     .  .
//      2
#[derive(Debug, Default)]
pub struct Gradient<T> {
    // these are the 3 vertex values that we are interpolating
    pub value: [T; 3],
    // how much the values change with every x and y steps
    pub step: Step<T>,
}

// 0 .
//  yxxxxx.
//   y       1
//    y    .
//     y  .
//      2
#[derive(Debug, Default)]
pub struct Step<T> {
    pub x: T,
    pub y: T,
}

/// vertices are aligned on the y-axis, min, mid, and max.
#[derive(Debug, Default, Clone)]
pub struct Triangle {
    pub min: Vertex,
    pub mid: Vertex,
    pub max: Vertex,
}

impl Gradients {
    pub fn new(triangle: Triangle) -> Self {
        // depth: interpolate between the z-axis of each vertex of the triangle
        let mut depth = Gradient::default();
        depth.value[0] = triangle.min.position.z;
        depth.value[2] = triangle.max.position.z;
        depth.value[1] = triangle.mid.position.z;

        // light amount: interpolate between light directions
        // initial light amount is calculated by taking the dot product of each vertex normal with the light direction
        let mut light_amt = Gradient::default();
        light_amt.value[0] = clamp(triangle.min.normal.dot(LIGHT_DIR), 0.0, 1.0) * 0.75 + 0.25;
        light_amt.value[1] = clamp(triangle.mid.normal.dot(LIGHT_DIR), 0.0, 1.0) * 0.75 + 0.25;
        light_amt.value[2] = clamp(triangle.max.normal.dot(LIGHT_DIR), 0.0, 1.0) * 0.75 + 0.25;

        // one over z: to get perspective correct values (texture-mapping)
        // one over z is a linear function which makes our gradient formula work as opposed to the inverse which is not linear
        // note that `w` is the perspective z-value while `z` is the occlution z-value
        let mut one_over_z = Gradient::default();
        one_over_z.value[0] = 1.0 / triangle.min.position.w;
        one_over_z.value[1] = 1.0 / triangle.mid.position.w;
        one_over_z.value[2] = 1.0 / triangle.max.position.w;

        // texture coordinates: perspective divide
        // https://youtu.be/_elt1LVUsdY?t=758
        // not everything can be linearly interpolated across the face of a triangle
        // since we can't interpolate texture coordinates directly
        // we can move them through the same transformation as the x and y (divide by z)
        // and no, we don't really care what tex-coord divided by z is but it can help us to get the actual z
        // the renderer does the transformation and gets the true tex-coords
        let mut texcoords = Gradient::default();
        texcoords.value[0] = triangle.min.texcoords * one_over_z.value[0]; // the same as: / triangle.min.position.w
        texcoords.value[1] = triangle.mid.texcoords * one_over_z.value[1]; // the same as: / triangle.mid.position.w
        texcoords.value[2] = triangle.max.texcoords * one_over_z.value[2]; // the same as: / triangle.max.position.w

        // triangle gradient interpolation formula
        // https://youtu.be/AysDWKF3CBs
        // http://www.chrishecker.com/images/4/41/Gdmtex1.pdf
        let a = triangle.mid.position.x - triangle.max.position.x;
        let b = triangle.min.position.y - triangle.max.position.y;
        let c = triangle.min.position.x - triangle.max.position.x;
        let d = triangle.mid.position.y - triangle.max.position.y;

        let delta = (a * b) - (c * d);

        let one_over_dx = 1.0 / delta;
        let one_over_dy = -one_over_dx;

        texcoords.calc_steps(&triangle, one_over_dx, one_over_dy);
        one_over_z.calc_steps(&triangle, one_over_dx, one_over_dy);
        depth.calc_steps(&triangle, one_over_dx, one_over_dy);
        light_amt.calc_steps(&triangle, one_over_dx, one_over_dy);

        Self {
            texcoords,
            one_over_z,
            depth,
            light_amt,
        }
    }
}

impl<T> Gradient<T>
where
    T: Sub<Output = T> + Mul<f32, Output = T> + Copy + Clone,
{
    pub fn calc_steps(&mut self, triangle: &Triangle, one_over_dx: f32, one_over_dy: f32) {
        self.calc_step_x(triangle, one_over_dx);
        self.calc_step_y(triangle, one_over_dy);
    }

    fn calc_step_x(&mut self, triangle: &Triangle, one_over_dx: f32) {
        // triangle gradient interpolation formula (again)
        let a = self.value[1] - self.value[2];
        let b = triangle.min.position.y - triangle.max.position.y;
        let c = self.value[0] - self.value[2];
        let d = triangle.mid.position.y - triangle.max.position.y;

        self.step.x = ((a * b) - (c * d)) * one_over_dx;
    }

    fn calc_step_y(&mut self, triangle: &Triangle, one_over_dy: f32) {
        // triangle gradient interpolation formula (again)
        let a = self.value[1] - self.value[2];
        let b = triangle.min.position.x - triangle.max.position.x;
        let c = self.value[0] - self.value[2];
        let d = triangle.mid.position.x - triangle.max.position.x;

        self.step.y = ((a * b) - (c * d)) * one_over_dy;
    }
}

impl Triangle {
    pub fn new(min: Vertex, mid: Vertex, max: Vertex) -> Self {
        return Self { min, mid, max };
    }
}
