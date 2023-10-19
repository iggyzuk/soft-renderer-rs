// #![allow(arithmetic_overflow)]

use std::ops::{Deref, DerefMut};

use crate::math::lerp;

use super::color::Color;


/// Bitmap with format: RGBA
#[derive(Debug, Clone)]
pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

// Bitmap is a list of pixels in RGBA format
impl Bitmap {

    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn from_bytes(width: u32, height: u32, pixels: Vec<u8>) -> Self {
        Self { width, height, pixels }
    }

    pub fn fill(&mut self, color: &Color) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_pixel(x, y, &color);
            }
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: &Color) {
        let index = ((x as usize + y as usize * self.width as usize) * 4) as usize;

        if index < 0 || index >= (self.width * self.height * 4) as usize {
            return;
        }

        let prev_color = Color::new(self.pixels[index + 0], self.pixels[index + 1], self.pixels[index + 2], 0xFF);

        let blend = (color.a as f32) / 255.0;
        self.pixels[index + 0] = lerp(prev_color.r as f32, color.r as f32, blend) as u8;
        self.pixels[index + 1] = lerp(prev_color.g as f32, color.g as f32, blend) as u8;
        self.pixels[index + 2] = lerp(prev_color.b as f32, color.b as f32, blend) as u8;
        self.pixels[index + 3] = 0xFF;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let index = ((x + y * self.width) * 4) as usize;

        if index < 0 || index >= (self.width * self.height * 4) as usize {
            return Color::BLACK;
        }

        Color::new(self.pixels[index + 0], self.pixels[index + 1], self.pixels[index + 2], self.pixels[index + 3])
    }

    // todo: remove this complexity (was a fun exersize)
    // pub fn copy_to_rgb(&self) -> Vec<u8> {
    //     let total_pixels = (self.width * self.height) as usize;
    //     let mut result = vec![0; total_pixels * Self::RGB as usize];

    //     for index in 0..total_pixels {
    //         let rgb_index = index * Self::RGB as usize;
    //         let rgba_index = index * Self::RGBA as usize;

    //         result[rgb_index + 0] = self.pixels[rgba_index + 0];
    //         result[rgb_index + 1] = self.pixels[rgba_index + 1];
    //         result[rgb_index + 2] = self.pixels[rgba_index + 2];
    //     }
    //     result
    // }
}

impl Deref for Bitmap {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}

impl DerefMut for Bitmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_rgba_to_rgb() {
    //     let mut bitmap_rgba = Bitmap::new(2, 2);
    //     bitmap_rgba.fill(&Color::GREEN);

    //     let bitmap_rgb = Bitmap::from_bytes(bitmap_rgba.width, bitmap_rgba.height, bitmap_rgba.copy_to_rgb());

    //     assert_eq!(bitmap_rgb.pixels[0], 0x00);
    //     assert_eq!(bitmap_rgb.pixels[1], 0xFF);
    //     assert_eq!(bitmap_rgb.pixels[2], 0x00);

    //     assert_eq!(bitmap_rgb.pixels[3], 0x00);
    //     assert_eq!(bitmap_rgb.pixels[4], 0xFF);
    //     assert_eq!(bitmap_rgb.pixels[5], 0x00);

    //     assert_eq!(bitmap_rgb.pixels[6], 0x00);
    //     assert_eq!(bitmap_rgb.pixels[7], 0xFF);
    //     assert_eq!(bitmap_rgb.pixels[8], 0x00);
    // }

    #[test]
    fn test_bitmap_color_bytes() {
        let mut bitmap = Bitmap::new(2, 2);
        bitmap.fill(&Color::GREEN);

        assert_eq!(bitmap.pixels[0], 0x00);
        assert_eq!(bitmap.pixels[1], 0xFF);
        assert_eq!(bitmap.pixels[2], 0x00);
        assert_eq!(bitmap.pixels[3], 0xFF);

        assert_eq!(bitmap.pixels[4], 0x00);
        assert_eq!(bitmap.pixels[5], 0xFF);
        assert_eq!(bitmap.pixels[6], 0x00);
        assert_eq!(bitmap.pixels[7], 0xFF);

        assert_eq!(bitmap.pixels[8], 0x00);
        assert_eq!(bitmap.pixels[9], 0xFF);
        assert_eq!(bitmap.pixels[10], 0x00);
        assert_eq!(bitmap.pixels[11], 0xFF);
    }
}
