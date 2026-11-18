use core::ops::Add;

use share::frame_buffer::FrameBufferConfig;

use crate::font::FONT_LUT;

pub struct Vector2D<T: Add> {
    x: T,
    y: T,
}

impl<T: Add> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add> From<(T, T)> for Vector2D<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> Add for Vector2D<T>
where
    T: Add<Output = T>,
{
    type Output = Vector2D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
}

impl From<(u8, u8, u8)> for PixelColor {
    fn from(value: (u8, u8, u8)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
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

#[derive(Debug, Clone, Copy)]
pub struct PixelWriter {
    config: &'static FrameBufferConfig,
}

unsafe impl Sync for PixelWriter {}

impl PixelWriter {
    pub fn new(config: &'static FrameBufferConfig) -> Self {
        Self { config }
    }

    pub fn pixel_at(&mut self, x: u32, y: u32) -> Option<&mut [u8]> {
        self.config
            .frame_buffer()
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

    pub fn fill_rectangle(&mut self, pos: Vector2D<u32>, size: Vector2D<u32>, color: PixelColor) {
        (0..size.y).for_each(|dy| {
            (0..size.x).for_each(|dx| {
                self.write(pos.x + dx, pos.y + dy, color);
            });
        });
    }
}
