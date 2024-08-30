use super::{bitmap::Bitmap, color::Color};

pub struct ScanLine {
    pub min: u16,
    pub max: u16,
}

impl ScanLine {
    fn new(min: u16, max: u16) -> Self {
        Self { min, max }
    }
}

pub struct ScanBuffer {
    pub lines: Vec<ScanLine>,
}

impl ScanBuffer {
    pub fn new() -> Self {
        Self { lines: vec![] }
    }

    pub fn push(&mut self, min: u16, max: u16) {
        self.lines.push(ScanLine::new(min, max));
    }

    pub fn draw(&self, bitmap: &mut Bitmap<u8>) {
        for (y, line) in self.lines.iter().enumerate() {
            for x in line.min..line.max {
                bitmap.set_pixel(x as u32, y as u32, &Color::WHITE);
            }
        }
    }
}
