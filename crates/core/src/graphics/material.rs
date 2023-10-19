use std::rc::Rc;

use super::bitmap::Bitmap;

pub struct Material {
    pub light: bool,
    pub bitmap: Rc<Box<Bitmap>>,
}

impl Material {
    pub fn new(light: bool, bitmap: Rc<Box<Bitmap>>) -> Self {
        Self { light, bitmap }
    }
}
