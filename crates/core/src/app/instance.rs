use std::rc::Rc;

use crate::graphics::bitmap::Bitmap;
use crate::graphics::light::Light;
use crate::graphics::material::Material;
use crate::graphics::mesh::Mesh;
use crate::math::linear_algebra::matrix::Matrix4;

use super::renderer::Renderer;

#[derive(Debug)]
pub struct Instance {
    pub mesh: Rc<Box<Mesh>>,
    // @todo: use material instead of bitmap
    pub bitmap: Rc<Box<Bitmap<u8>>>,
    pub transform: Matrix4,
    pub light: bool,
}

impl Instance {
    pub fn new(mesh: Rc<Box<Mesh>>, bitmap: Rc<Box<Bitmap<u8>>>, light: bool) -> Self {
        Self {
            mesh,
            bitmap,
            transform: Matrix4::new_identity(),
            light,
        }
    }

    pub fn draw(&self, renderer: &mut Renderer, view_projection: &Matrix4, light: Option<&Light>) {
        renderer.draw_mesh(
            self.mesh.as_ref(),
            view_projection,
            &self.transform,
            // @todo: use Rc Box Material instead of Bitmap
            &Material::new(self.light, self.bitmap.clone()),
            light,
        );
    }
}
