use rand::Rng;

use crate::math::linear_algebra::vector::Vector4;

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Color = Self {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
        a: 0xFF,
    };

    pub const BLACK: Color = Self {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    };

    pub const GRAY: Color = Self {
        r: 0xAA,
        g: 0xAA,
        b: 0xAA,
        a: 0xFF,
    };

    pub const RED: Color = Self {
        r: 0xFF,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    };

    pub const GREEN: Color = Self {
        r: 0x00,
        g: 0xFF,
        b: 0x00,
        a: 0xFF,
    };

    pub const BLUE: Color = Self {
        r: 0x00,
        g: 0x00,
        b: 0xFF,
        a: 0xFF,
    };

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 24) & 0xFF) as u8,
            g: ((hex >> 16) & 0xFF) as u8,
            b: ((hex >> 8) & 0xFF) as u8,
            a: ((hex) & 0xFF) as u8,
        }
    }

    pub fn newf(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
            a: (a * 255.0) as u8,
        }
    }

    pub fn from_random(min: u8, max: u8) -> Self {
        Self {
            r: rand::thread_rng().gen_range(min..max),
            g: rand::thread_rng().gen_range(min..max),
            b: rand::thread_rng().gen_range(min..max),
            a: rand::thread_rng().gen_range(min..max),
        }
    }
}

impl From<Vector4> for Color {
    fn from(v: Vector4) -> Self {
        Self {
            r: v.x as u8,
            g: v.y as u8,
            b: v.z as u8,
            a: v.w as u8,
        }
    }
}
