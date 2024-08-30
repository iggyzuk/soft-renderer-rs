use std::rc::Rc;

use super::bitmap::Bitmap;

pub struct Material {
    pub light: bool,
    pub bitmap: Rc<Box<Bitmap<u8>>>,
}

impl Material {
    pub fn new(light: bool, bitmap: Rc<Box<Bitmap<u8>>>) -> Self {
        Self { light, bitmap }
    }
}
