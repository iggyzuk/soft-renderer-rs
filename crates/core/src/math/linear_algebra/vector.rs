use std::ops::{AddAssign, Neg};

use rand::Rng;

use crate::math::PI;

use super::quaternion::Quaternion;

/// # Vector
/// Vector in homogeneous coordinates
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for Vector4 {
    fn default() -> Self {
        return Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
    }
}

impl Vector4 {
    pub const ZERO: Vector4 = Vector4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };

    pub const ONE: Vector4 = Vector4 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        w: 0.0,
    };

    pub const RIGHT: Vector4 = Vector4 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
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
        w: 0.0,
    };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        return Self { x, y, z, w };
    }

    pub fn from_random(min: f32, max: f32) -> Self {
        return Self {
            x: rand::thread_rng().gen_range(min..max),
            y: rand::thread_rng().gen_range(min..max),
            z: rand::thread_rng().gen_range(min..max),
            w: rand::thread_rng().gen_range(min..max),
        };
    }

    pub fn length(&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();

        if len <= 0.0 {
            panic!("can't divide by zero when normalizing a vector")
        }

        return Self::new(self.x / len, self.y / len, self.z / len, self.w / len);
    }

    pub fn dot(&self, v: Vector4) -> f32 {
        return self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w;
    }

    pub fn cross(&self, v: Vector4) -> Self {
        let xx = self.y * v.z - self.z * v.y;
        let yy = self.z * v.x - self.x * v.z;
        let zz = self.x * v.y - self.y * v.x;
        return Self::new(xx, yy, zz, 0.0);
    }

    pub fn lerp(&self, dest: Vector4, factor: f32) -> Self {
        return (*self) * (1.0 - factor) + dest * factor;
    }

    pub fn rotate_quaternion(&self, rotation: Quaternion) -> Self {
        let w = (rotation * self) * rotation.conjugate();
        return Vector4::new(w.x, w.y, w.z, 1.0);
    }

    pub fn rotate_angle(&self, axis: Vector4, angle: f32) -> Self {
        let sin_angle = (-angle).sin();
        let cos_angle = (-angle).cos();

        return self.cross(axis * sin_angle) +             // Rotation on local X
               *self * cos_angle +                        // Rotation on local Z
               axis * self.dot(axis * (1.0 - cos_angle)); // Rotation on local Y
    }

    pub fn polar(radius: f32, inclination: f32, azimuth: f32) -> Self {
        return Self {
            x: radius * (inclination.sin()) * (azimuth.cos()),
            y: radius * (inclination.sin()) * (azimuth.sin()),
            z: radius * (inclination.cos()),
            w: 1.0,
        };
    }

    pub fn polar_degrees(radius: f32, inclination: f32, azimuth: f32) -> Self {
        return Self::polar(radius, inclination * PI / 180.0, azimuth * PI / 180.0);
    }
}

impl std::ops::Add for Vector4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        );
    }
}

impl std::ops::Sub for Vector4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        );
    }
}

impl std::ops::Mul<f32> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        return Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs);
    }
}

impl std::ops::Div<f32> for Vector4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        assert_ne!(rhs, 0.0);
        return Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs);
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
        return Self::new(-self.x, -self.y, -self.z, -self.w);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_new() {
        let mut v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);
        assert_eq!(v1.w, 4.0);

        v1.x = 7.0;
        v1.y = 7.0;
        v1.z = 7.0;
        v1.w = 7.0;
        assert_eq!(v1.x, 7.0);
        assert_eq!(v1.y, 7.0);
        assert_eq!(v1.z, 7.0);
        assert_eq!(v1.w, 7.0);
    }

    #[test]
    fn test_vector_add() {
        let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vector4::new(5.0, 6.0, 7.0, 8.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 6.0);
        assert_eq!(v3.y, 8.0);
        assert_eq!(v3.z, 10.0);
        assert_eq!(v3.w, 12.0);
    }

    #[test]
    fn test_vector_sub() {
        let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vector4::new(5.0, 6.0, 7.0, 8.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x, -4.0);
        assert_eq!(v3.y, -4.0);
        assert_eq!(v3.z, -4.0);
        assert_eq!(v3.w, -4.0);
    }

    #[test]
    fn test_vector_length() {
        let v1 = Vector4::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(v1.length(), 1.0);
    }

    #[test]
    fn test_vector_normalized() {
        let v1 = Vector4::new(0.0, 123.0, 0.0, 0.0);
        assert_eq!(v1.normalized(), Vector4::new(0.0, 1.0, 0.0, 0.0));
    }

    // #[test]
    // fn test_vector_rotate() {
    //     let v1 = Vector4::new(1.0, 0.0, 0.0, 0.0);
    //     assert_eq!(
    //         v1.rotate_angle(Vector4::UP, 3.1),
    //         Vector4::new(0.0, 0.0, 1.0, 0.0)
    //     );
    // }
}
