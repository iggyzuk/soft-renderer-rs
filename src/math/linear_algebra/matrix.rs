use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::math::PI;

use super::vector::Vector4;

#[derive(Clone)]
pub struct Matrix4 {
    pub matrix: [[f32; 4]; 4],
}

// Matrix
// [1 0 0 tx]
// [0 1 0 ty] < [3][1]
// [0 0 1 tz]
// [0 0 0  1]
// tx, ty, tz for translation
// [col][row]
impl Matrix4 {
    pub fn new() -> Self {
        Self {
            matrix: Default::default(),
        }
    }

    pub fn new_identity() -> Self {
        let mut matrix = Self::new();
        matrix.identity();
        matrix
    }

    pub fn identity(&mut self) {
        for i in 0..4 {
            for j in 0..4 {
                self[i][j] = if i == j { 1.0 } else { 0.0 }
            }
        }
    }

    pub fn perspective(fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        // aspect*1/(tan(ø)/2) | 0            | 0                    | 0
        // 0                   | 1/(tan(ø)/2) | 0                    | 0
        // 0                   | 0            | z_far/(z_far-z_near) | -(z_far*z_near)/(z_far-z_near)
        // 0                   | 0            | 1                    | 0

        let mut matrix = Self::new();

        // fov
        //
        // --d------
        //  \* | <--- how far is * on d and z?
        //   \ z /
        //    \|/
        //    eye
        //
        // we can use trig for this!
        //
        // d = opposite
        // z = adjacent
        //
        // O/A = tan(ø)=d/z => tan(ø)z=d
        // d = tan(ø) * z
        //
        // but keep in mind it's just half (reference the image for why)

        let fov_in_radians = fov * (PI / 180.0);
        let tan_half_fov = (fov_in_radians / 2.0).tan();

        // z range tells us the min and max z values that we can accept, normalize and visualize
        //
        // example:
        //   far:  10
        //   near: 0.1
        //   range = 0.9

        let z_range = z_far - z_near;

        // 1 / fov because the bigger the fov the smaller the objects will appear
        // only the x is scaled by the aspect ratio, the y stays the same
        matrix[0][0] = 1.0 / (tan_half_fov * aspect_ratio); // x scale by field of view and stretch for aspect
        matrix[1][1] = 1.0 / tan_half_fov; // y scale by field of view

        // z_far is the largest z value that we can visualize
        // z_near is the smallest z value that we can visualize
        //
        // normalize the z into the image space -1 to +1
        // it remaps our world z value into normalized range

        matrix[2][2] = z_far / z_range; // z normalization
        matrix[3][2] = -(z_far * z_near) / z_range; // z translation towards the z near plane

        // perspective divide will happen later
        // we save the original z value inside of w (of the resulting vector)
        matrix[2][3] = 1.0;

        // alternative, why?
        // matrix[2][2] = -(z_far + z_near) / z_range;
        // matrix[3][2] = 2.0 * (z_far * z_near) / z_range;
        // matrix[2][3] = -1.0;

        matrix
    }

    // screen space matrix that takes values in (-1, 1) and transforms them to (0, width) and (0 height).
    pub fn screenspace(width: f32, height: f32) -> Self {
        let mut matrix = Matrix4::new_identity();

        let half_width = width / 2.0;
        let half_height = height / 2.0;

        // scale x by half_width
        // example: if width is 800 we take the half of it 400 and scale the x value by it
        //   so -1 would be -400
        matrix[0][0] = half_width;
        // scale y by half_height
        // and keep in mind the in screen-space coordinates on y zero is the top, not bottom
        matrix[1][1] = -half_height;
        // translate by half_width
        // -0.5 so it rounds properly with our fill convenction of `ceil`
        // example: from the previous example we have x:-400, now we add another 400
        //   and so we get 0 for the initial value of -1
        //   for 0 it would be 400 and 1 it would be 800
        matrix[3][0] = half_width - 0.5;
        // the same but on y
        matrix[3][1] = half_height - 0.5;

        matrix
    }

    pub fn look_at(&mut self, eye: Vector4, target: Vector4, up_axis: Vector4) {
        let forward = (target - eye).normalize();
        let right = up_axis.cross(forward).normalize();
        let up = forward.cross(right).normalize();

        let mut m = Self::new_identity();

        m[0][0] = right.x;
        m[1][0] = right.y;
        m[2][0] = right.z;

        m[0][1] = up.x;
        m[1][1] = up.y;
        m[2][1] = up.z;

        m[0][2] = forward.x;
        m[1][2] = forward.y;
        m[2][2] = forward.z;

        m.translate(-eye.x, -eye.y, -eye.z);

        self.matrix = Self::multiply(self, &m).matrix;
    }

    // @todo: figure out how to do M * M
    pub fn multiply(lhs: &Matrix4, rhs: &Matrix4) -> Self {
        let mut result = Self::new();
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = lhs[0][j] * rhs[i][0] + lhs[1][j] * rhs[i][1] + lhs[2][j] * rhs[i][2] + lhs[3][j] * rhs[i][3];
            }
        }
        result
    }

    // @todo: figure out how to do M * V
    pub fn multiply_vector(lhs: &Matrix4, rhs: Vector4) -> Vector4 {
        let mut a = [0.0, 0.0, 0.0, 0.0];
        for i in 0..4 {
            a[i] = lhs[0][i] * rhs.x + lhs[1][i] * rhs.y + lhs[2][i] * rhs.z + lhs[3][i] * rhs.w;
        }
        Vector4::new(a[0], a[1], a[2], a[3])
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let mut m = Self::new_identity();

        m[3][0] = x;
        m[3][1] = y;
        m[3][2] = z;

        self.matrix = Self::multiply(self, &m).matrix;
    }

    pub fn rotate_x(&mut self, angle: f32) {
        let radian = angle * (PI / 180.0);
        let sinus = radian.sin();
        let cosinus = radian.cos();

        let mut m = Self::new_identity();
        m[1][1] = cosinus;
        m[2][2] = cosinus;
        m[1][2] = sinus;
        m[2][1] = -sinus;

        self.matrix = Self::multiply(self, &m).matrix;
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let radian = angle * (PI / 180.0);
        let sinus = radian.sin();
        let cosinus = radian.cos();

        let mut m = Self::new_identity();
        m[0][0] = cosinus;
        m[2][2] = cosinus;
        m[0][2] = -sinus;
        m[2][0] = sinus;

        self.matrix = Self::multiply(self, &m).matrix;
    }

    pub fn rotate_z(&mut self, angle: f32) {
        let radian = angle * (PI / 180.0);
        let sinus = (radian).sin();
        let cosinus = (radian).cos();

        let mut m = Self::new_identity();
        m[0][0] = cosinus;
        m[1][1] = cosinus;
        m[0][1] = sinus;
        m[1][0] = -sinus;

        self.matrix = Self::multiply(self, &m).matrix;
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        let mut m = Self::new_identity();
        m[0][0] = x;
        m[1][1] = y;
        m[2][2] = z;
        self.matrix = Self::multiply(self, &m).matrix;
    }

    pub fn invert(&mut self) -> bool {
        let mut inv = Self::new();

        inv[0][0] = self[1][1] * self[2][2] * self[3][3] - self[1][1] * self[3][2] * self[2][3] - self[1][2] * self[2][1] * self[3][3]
            + self[1][2] * self[3][1] * self[2][3]
            + self[1][3] * self[2][1] * self[3][2]
            - self[1][3] * self[3][1] * self[2][2];

        inv[0][1] = -self[0][1] * self[2][2] * self[3][3] + self[0][1] * self[3][2] * self[2][3] + self[0][2] * self[2][1] * self[3][3]
            - self[0][2] * self[3][1] * self[2][3]
            - self[0][3] * self[2][1] * self[3][2]
            + self[0][3] * self[3][1] * self[2][2];

        inv[0][2] = self[0][1] * self[1][2] * self[3][3] - self[0][1] * self[3][2] * self[1][3] - self[0][2] * self[1][1] * self[3][3]
            + self[0][2] * self[3][1] * self[1][3]
            + self[0][3] * self[1][1] * self[3][2]
            - self[0][3] * self[3][1] * self[1][2];

        inv[0][3] = -self[0][1] * self[1][2] * self[2][3] + self[0][1] * self[2][2] * self[1][3] + self[0][2] * self[1][1] * self[2][3]
            - self[0][2] * self[2][1] * self[1][3]
            - self[0][3] * self[1][1] * self[2][2]
            + self[0][3] * self[2][1] * self[1][2];

        inv[1][0] = -self[1][0] * self[2][2] * self[3][3] + self[1][0] * self[3][2] * self[2][3] + self[1][2] * self[2][0] * self[3][3]
            - self[1][2] * self[3][0] * self[2][3]
            - self[1][3] * self[2][0] * self[3][2]
            + self[1][3] * self[3][0] * self[2][2];

        inv[1][1] = self[0][0] * self[2][2] * self[3][3] - self[0][0] * self[3][2] * self[2][3] - self[0][2] * self[2][0] * self[3][3]
            + self[0][2] * self[3][0] * self[2][3]
            + self[0][3] * self[2][0] * self[3][2]
            - self[0][3] * self[3][0] * self[2][2];

        inv[1][2] = -self[0][0] * self[1][2] * self[3][3] + self[0][0] * self[3][2] * self[1][3] + self[0][2] * self[1][0] * self[3][3]
            - self[0][2] * self[3][0] * self[1][3]
            - self[0][3] * self[1][0] * self[3][2]
            + self[0][3] * self[3][0] * self[1][2];

        inv[1][3] = self[0][0] * self[1][2] * self[2][3] - self[0][0] * self[2][2] * self[1][3] - self[0][2] * self[1][0] * self[2][3]
            + self[0][2] * self[2][0] * self[1][3]
            + self[0][3] * self[1][0] * self[2][2]
            - self[0][3] * self[2][0] * self[1][2];

        inv[2][0] = self[1][0] * self[2][1] * self[3][3] - self[1][0] * self[3][1] * self[2][3] - self[1][1] * self[2][0] * self[3][3]
            + self[1][1] * self[3][0] * self[2][3]
            + self[1][3] * self[2][0] * self[3][1]
            - self[1][3] * self[3][0] * self[2][1];

        inv[2][1] = -self[0][0] * self[2][1] * self[3][3] + self[0][0] * self[3][1] * self[2][3] + self[0][1] * self[2][0] * self[3][3]
            - self[0][1] * self[3][0] * self[2][3]
            - self[0][3] * self[2][0] * self[3][1]
            + self[0][3] * self[3][0] * self[2][1];

        inv[2][2] = self[0][0] * self[1][1] * self[3][3] - self[0][0] * self[3][1] * self[1][3] - self[0][1] * self[1][0] * self[3][3]
            + self[0][1] * self[3][0] * self[1][3]
            + self[0][3] * self[1][0] * self[3][1]
            - self[0][3] * self[3][0] * self[1][1];

        inv[2][3] = -self[0][0] * self[1][1] * self[2][3] + self[0][0] * self[2][1] * self[1][3] + self[0][1] * self[1][0] * self[2][3]
            - self[0][1] * self[2][0] * self[1][3]
            - self[0][3] * self[1][0] * self[2][1]
            + self[0][3] * self[2][0] * self[1][1];

        inv[3][0] = -self[1][0] * self[2][1] * self[3][2] + self[1][0] * self[3][1] * self[2][2] + self[1][1] * self[2][0] * self[3][2]
            - self[1][1] * self[3][0] * self[2][2]
            - self[1][2] * self[2][0] * self[3][1]
            + self[1][2] * self[3][0] * self[2][1];

        inv[3][1] = self[0][0] * self[2][1] * self[3][2] - self[0][0] * self[3][1] * self[2][2] - self[0][1] * self[2][0] * self[3][2]
            + self[0][1] * self[3][0] * self[2][2]
            + self[0][2] * self[2][0] * self[3][1]
            - self[0][2] * self[3][0] * self[2][1];

        inv[3][2] = -self[0][0] * self[1][1] * self[3][2] + self[0][0] * self[3][1] * self[1][2] + self[0][1] * self[1][0] * self[3][2]
            - self[0][1] * self[3][0] * self[1][2]
            - self[0][2] * self[1][0] * self[3][1]
            + self[0][2] * self[3][0] * self[1][1];

        inv[3][3] = self[0][0] * self[1][1] * self[2][2] - self[0][0] * self[2][1] * self[1][2] - self[0][1] * self[1][0] * self[2][2]
            + self[0][1] * self[2][0] * self[1][2]
            + self[0][2] * self[1][0] * self[2][1]
            - self[0][2] * self[2][0] * self[1][1];

        // find determinant and check if it's zero meaning matrix is not invertable
        let mut det = self[0][0] * inv[0][0] + self[1][0] * inv[0][1] + self[2][0] * inv[0][2] + self[3][0] * inv[0][3];

        if det == 0.0 {
            return false;
        }
        det = 1.0 / det;

        // fill the matrix with inverted values
        for j in 0..4 {
            for i in 0..4 {
                self[j][i] = inv[j][i] * det;
            }
        }

        return true;
    }

    #[inline(always)]
    pub fn translation(&self) -> Vector4 {
        Vector4::new(self[3][0], self[3][1], self[3][2], 1.0)
    }
}

impl Deref for Matrix4 {
    type Target = [[f32; 4]; 4];

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl DerefMut for Matrix4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.matrix
    }
}

impl Debug for Matrix4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..4 {
            writeln!(f, "[{},{},{},{}]", self.matrix[0][i], self.matrix[1][0], self.matrix[2][i], self.matrix[3][i])?;
        }
        Ok(())
    }
}

// impl std::ops::Mul<Vector4> for &mut Matrix4 {
//     type Output = Vector4;

//     fn mul(self, rhs: Vector4) -> Self::Output {
//         let mut a = [0.0, 0.0, 0.0, 0.0];
//         for i in 0..4 {
//             a[i] = self[0][i] * rhs.x + self[1][i] * rhs.y + self[2][i] * rhs.z + self[3][i] * rhs.w;
//         }
//         Vector4::new(a[0], a[1], a[2], a[3])
//     }
// }
