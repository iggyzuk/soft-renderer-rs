use crate::math::linear_algebra::{matrix::Matrix4, vector::Vector4};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex {
    pub position: Vector4,
    pub texcoords: Vector4,
    pub normal: Vector4,
}

impl Vertex {
    pub fn new(position: Vector4, texcoords: Vector4, normal: Vector4) -> Self {
        Self { position, texcoords, normal }
    }

    pub fn transform(mut self, transform_mat: &Matrix4, normal_mat: &Matrix4) -> Self {
        self.position = Matrix4::multiply_vector(transform_mat, self.position);
        self.normal = Matrix4::multiply_vector(normal_mat, self.normal);
        self
    }

    // performs perspective divide with original z-value that is now stored in w
    // position will be in image space
    //      +1  +1
    //       | /
    //  -1 ------ +1
    //     / |
    //   -1 -1
    //
    pub fn perspective_divide(mut self) -> Self {
        self.position.x /= self.position.w;
        self.position.y /= self.position.w;
        self.position.z /= self.position.w;
        self
    }

    // cross product of vertex with two others can tell us its handedness
    // it's doubled the size of a triangle, but we don't care as we only use the > 0 for right and < 0 for left
    // start at min -> max and min -> mid
    // ------
    // |   /|
    // |  / |
    // | /  |
    // |/   |
    // ------
    pub fn triangle_area_times_two(&self, b: &Vertex, c: &Vertex) -> f32 {
        let x1 = b.position.x - self.position.x;
        let y1 = b.position.y - self.position.y;

        let x2 = c.position.x - self.position.x;
        let y2 = c.position.y - self.position.y;

        return x1 * y2 - x2 * y1;
    }

    // lerp all vertex values, it's used for clipping vertices
    pub fn lerp(&self, other: &Vertex, lerp_amt: f32) -> Self {
        Self::new(
            self.position.lerp(other.position, lerp_amt),
            self.texcoords.lerp(other.texcoords, lerp_amt),
            self.normal.lerp(other.normal, lerp_amt),
        )
    }

    // -1 ≤ xp ≤ +1     x projected
    // -1 ≤ x/w ≤ +1    x projected is x divided by w (perspective divide)
    // -w ≤ x ≤ +w      multiply both sides by w
    //
    // clipping before perspective divide
    pub fn is_inside_view_frustum(&self) -> bool {
        (self.position.x).abs() <= (self.position.w).abs()
            && (self.position.y).abs() <= (self.position.w).abs()
            && (self.position.z).abs() <= (self.position.w).abs()
    }

    pub fn get(&self, index: usize) -> f32 {
        match index {
            0 => self.position.x,
            1 => self.position.y,
            2 => self.position.z,
            3 => self.position.w,
            _ => panic!("vertex has no index: ({})", index),
        }
    }
}
