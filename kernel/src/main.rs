#![no_std]
#![no_main]

mod font;
mod graphics;

use share::{frame_buffer::FrameBufferConfig, x86};

use crate::graphics::{PixelColor, PixelWriter};

#[unsafe(no_mangle)]
extern "C" fn kernel_main(config: &mut FrameBufferConfig) -> ! {
    let mut pixel_writer = PixelWriter::new(config);

    for y in 100..150 {
        for x in 100..200 {
            pixel_writer.write(x, y, PixelColor::BLUE);
        }
    }

    pixel_writer
        .write_ascii(0, 0, 'B', PixelColor::default())
        .unwrap();

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
