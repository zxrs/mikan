#![no_std]
#![no_main]

mod console;
mod font;
mod graphics;

use share::{frame_buffer::FrameBufferConfig, x86};

use crate::{
    console::Console,
    graphics::{PixelColor, PixelWriter},
};

#[unsafe(no_mangle)]
extern "C" fn kernel_main(config: &'static mut FrameBufferConfig) -> ! {
    let mut pixel_writer = PixelWriter::new(config);
    let mut console = Console::new(&mut pixel_writer, PixelColor::BLACK, PixelColor::WHITE);
    for _ in 0..26 {
        console.put_string("hi\n");
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
