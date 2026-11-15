#![no_std]
#![no_main]

use share::{frame_buffer::FrameBufferConfig, x86};

#[derive(Debug, Clone, Copy)]
pub struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
}

impl Default for PixelColor {
    fn default() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}

impl PixelColor {
    const BLUE: Self = Self { r: 0, g: 0, b: 255 };
}

#[derive(Debug)]
struct PixelWriter<'a> {
    config: &'a mut FrameBufferConfig,
}

impl<'a> PixelWriter<'a> {
    pub fn new(config: &'a mut FrameBufferConfig) -> Self {
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
}

#[unsafe(no_mangle)]
extern "C" fn kernel_main(config: &mut FrameBufferConfig) -> ! {
    let mut pixel_writer = PixelWriter::new(config);

    for y in 100..150 {
        for x in 100..200 {
            pixel_writer.write(x, y, PixelColor::BLUE);
        }
    }

    loop {
        x86::halt();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        x86::halt();
    }
}
