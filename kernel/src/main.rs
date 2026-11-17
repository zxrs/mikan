#![no_std]
#![no_main]

mod console;
mod font;
mod graphics;

use core::fmt::Write;
use share::{frame_buffer::FrameBufferConfig, x86};

use crate::{
    console::Console,
    graphics::{PixelColor, PixelWriter},
};

#[unsafe(no_mangle)]
extern "C" fn kernel_main(config: &'static mut FrameBufferConfig) -> ! {
    let mut pixel_writer = PixelWriter::new(config);
    let mut console = Console::new(&mut pixel_writer, PixelColor::BLACK, PixelColor::WHITE);
    // _ = writeln!(&mut console, "Hello, World!");
    for i in 0..25 {
        _ = writeln!(&mut console, "index: {i}");
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
