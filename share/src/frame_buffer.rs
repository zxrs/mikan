#[derive(Debug, Clone, Copy)]
pub struct FrameBufferConfig {
    pub frame_buffer_base: *mut u8,
    pub frame_buffer_size: usize,
    pub pixels_per_scanline: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
}

impl FrameBufferConfig {
    pub fn new(
        frame_buffer_base: *mut u8,
        frame_buffer_size: usize,
        pixels_per_scanline: u32,
        horizontal_resolution: u32,
        vertical_resolution: u32,
    ) -> Self {
        Self {
            frame_buffer_base,
            frame_buffer_size,
            pixels_per_scanline,
            horizontal_resolution,
            vertical_resolution,
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn frame_buffer(&self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.frame_buffer_base, self.frame_buffer_size) }
    }
}
