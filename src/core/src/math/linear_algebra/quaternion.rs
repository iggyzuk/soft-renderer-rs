use super::vector::Vector4;

#[derive(Clone, Copy, Debug)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        return Self { x, y, z, w };
    }

    pub fn length(&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();

        if len <= 0.0 {
            panic!("can't divide by zero when normalizing a quaternion")
        }

        return Self::new(self.x / len, self.y / len, self.z / len, self.w / len);
    }

    pub fn conjugate(&self) -> Self {
        return Self::new(-self.x, -self.y, -self.z, self.w);
    }

    pub fn from_angle(angle: f32, axis: Vector4) -> Self {
        let sin_half_angle = (angle / 2.0).sin();
        let cos_half_angle = (angle / 2.0).cos();
        return Self::new(
            axis.x * sin_half_angle,
            axis.y * sin_half_angle,
            axis.z * sin_half_angle,
            cos_half_angle,
        );
    }

    pub fn dot(&self, q: Vector4) -> f32 {
        return self.x * q.x + self.y * q.y + self.z * q.z + self.w * q.w;
    }
}

impl std::ops::Add for Quaternion {
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

impl std::ops::Sub for Quaternion {
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

impl std::ops::Mul<Quaternion> for Quaternion {
    type Output = Self;

    fn mul(self, q: Quaternion) -> Self::Output {
        let ww = self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z;
        let xx = self.x * q.w + self.w * q.x + self.y * q.z - self.z * q.y;
        let yy = self.y * q.w + self.w * q.y + self.z * q.x - self.x * q.z;
        let zz = self.z * q.w + self.w * q.z + self.x * q.y - self.y * q.x;

        return Self::new(xx, yy, zz, ww);
    }
}

impl std::ops::Mul<Vector4> for Quaternion {
    type Output = Self;

    fn mul(self, v: Vector4) -> Self::Output {
        let ww = -self.x * v.x - self.y * v.y - self.z * v.z;
        let xx = self.w * v.x + self.y * v.z - self.z * v.y;
        let yy = self.w * v.y + self.z * v.x - self.x * v.z;
        let zz = self.w * v.z + self.x * v.y - self.y * v.x;

        return Self::new(xx, yy, zz, ww);
    }
}

impl std::ops::Mul<&Vector4> for Quaternion {
    type Output = Self;

    fn mul(self, v: &Vector4) -> Self::Output {
        let ww = -self.x * v.x - self.y * v.y - self.z * v.z;
        let xx = self.w * v.x + self.y * v.z - self.z * v.y;
        let yy = self.w * v.y + self.z * v.x - self.x * v.z;
        let zz = self.w * v.z + self.x * v.y - self.y * v.x;

        return Self::new(xx, yy, zz, ww);
    }
}
