use std::ops::{AddAssign, Neg};

use rand::Rng;

use crate::math::PI;

use super::quaternion::Quaternion;

// vector in homogeneous coordinates
#[derive(Clone, Copy, Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for Vector4 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

impl Vector4 {
    pub const ZERO: Vector4 = Vector4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    pub const ONE: Vector4 = Vector4 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        w: 1.0,
    };

    pub const RIGHT: Vector4 = Vector4 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    pub const UP: Vector4 = Vector4 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
        w: 0.0,
    };

    pub const FORWARD: Vector4 = Vector4 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
        w: 1.0,
    };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_random(min: f32, max: f32) -> Self {
        Self {
            x: rand::thread_rng().gen_range(min..max),
            y: rand::thread_rng().gen_range(min..max),
            z: rand::thread_rng().gen_range(min..max),
            w: rand::thread_rng().gen_range(min..max),
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();

        // maybe panic here?
        assert_ne!(len, 0.0);

        Self::new(self.x / len, self.y / len, self.z / len, self.w / len)
    }

    pub fn dot(&self, v: Vector4) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w
    }

    pub fn cross(&self, v: Vector4) -> Self {
        let xx = self.y * v.z - self.z * v.y;
        let yy = self.z * v.x - self.x * v.z;
        let zz = self.x * v.y - self.y * v.x;
        Self::new(xx, yy, zz, 0.0)
    }

    pub fn lerp(&self, dest: Vector4, factor: f32) -> Self {
        return (*self) * (1.0 - factor) + dest * factor;
    }

    pub fn rotate_quaternion(&self, rotation: Quaternion) -> Self {
        let w = (rotation * self) * rotation.conjugate();
        Vector4::new(w.x, w.y, w.z, 1.0)
    }

    pub fn rotate_angle(&self, axis: Vector4, angle: f32) -> Self {
        let sin_angle = (-angle).sin();
        let cos_angle = (-angle).cos();

        return self.cross(axis * sin_angle) +             // Rotation on local X
               *self * cos_angle +                        // Rotation on local Z
               axis * self.dot(axis * (1.0 - cos_angle)); // Rotation on local Y
    }

    pub fn polar(radius: f32, inclination: f32, azimuth: f32) -> Self {
        Self {
            x: radius * (inclination.sin()) * (azimuth.cos()),
            y: radius * (inclination.sin()) * (azimuth.sin()),
            z: radius * (inclination.cos()),
            w: 1.0,
        }
    }

    pub fn polar_degrees(radius: f32, inclination: f32, azimuth: f32) -> Self {
        Self::polar(radius, inclination * PI / 180.0, azimuth * PI / 180.0)
    }
}

impl std::ops::Add for Vector4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl std::ops::Sub for Vector4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl std::ops::Mul<f32> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl std::ops::Div<f32> for Vector4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        assert_ne!(rhs, 0.0);
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl AddAssign for Vector4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Neg for Vector4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}
