#![no_std]
#![no_main]

mod font;
mod graphics;

use share::{frame_buffer::FrameBufferConfig, x86};

use crate::graphics::{PIXEL_WRITER, PixelColor, init_pixel_writer};

#[unsafe(no_mangle)]
extern "C" fn kernel_main(config: &'static mut FrameBufferConfig) -> ! {
    // let mut pixel_writer = PixelWriter::new(config);
    init_pixel_writer(config);

    if let Some(pixel_writer) = PIXEL_WRITER.inner.borrow_mut().as_mut() {
        for y in 100..150 {
            for x in 100..200 {
                pixel_writer.write(x, y, PixelColor::BLUE);
            }
        }

        pixel_writer
            .write_string(0, 0, "Hello, World!", PixelColor::default())
            .unwrap();
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
