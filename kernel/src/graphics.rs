use share::frame_buffer::FrameBufferConfig;

use crate::font::FONT_LUT;

#[derive(Debug, Clone, Copy)]
pub struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
}

impl Default for PixelColor {
    fn default() -> Self {
        Self::WHITE
    }
}

#[allow(unused)]
impl PixelColor {
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };
}

#[derive(Debug)]
pub struct PixelWriter {
    config: &'static mut FrameBufferConfig,
}

impl PixelWriter {
    pub fn new(config: &'static mut FrameBufferConfig) -> Self {
        Self { config }
    }

    pub fn pixel_at(&mut self, x: u32, y: u32) -> Option<&mut [u8]> {
        self.config
            .frame_buffer
            .chunks_mut(self.config.pixels_per_scanline as usize * 4)
            .nth(y as _)
            .and_then(|row| row.chunks_mut(4).nth(x as _))
    }

    pub fn write(&mut self, x: u32, y: u32, c: PixelColor) -> Option<()> {
        let p = self.pixel_at(x, y)?;
        p[0] = c.b;
        p[1] = c.g;
        p[2] = c.r;
        Some(())
    }

    pub fn write_ascii(&mut self, x: u32, y: u32, c: char, color: PixelColor) -> Option<()> {
        for dy in 0..16 {
            for dx in 0..8 {
                if (FONT_LUT[c as usize][dy as usize] << dx) & 0x80 > 0 {
                    self.write(x + dx, y + dy, color)?;
                }
            }
        }
        Some(())
    }

    pub fn write_string(&mut self, x: u32, y: u32, s: &str, color: PixelColor) -> Option<()> {
        for (i, c) in s.chars().enumerate() {
            let i = i as u32;
            self.write_ascii(x + i * 8, y, c, color)?;
        }
        Some(())
    }
}
