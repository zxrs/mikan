#[derive(Debug)]
pub struct FrameBufferConfig {
    pub frame_buffer: &'static mut [u8],
    pub pixels_per_scanline: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
}

impl FrameBufferConfig {
    pub fn new(
        frame_buffer: &'static mut [u8],
        pixels_per_scanline: u32,
        horizontal_resolution: u32,
        vertical_resolution: u32,
    ) -> Self {
        Self {
            frame_buffer,
            pixels_per_scanline,
            horizontal_resolution,
            vertical_resolution,
        }
    }
}
