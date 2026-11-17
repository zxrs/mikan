#![no_std]
#![no_main]

#[macro_use]
mod console;
mod font;
mod graphics;

use core::fmt::Write;
use share::{frame_buffer::FrameBufferConfig, x86};

use crate::{console::CONSOLE, graphics::PixelWriter};

#[unsafe(no_mangle)]
extern "C" fn kernel_main(config: &'static mut FrameBufferConfig) -> ! {
    let pixel_writer = PixelWriter::new(config);
    console::init(pixel_writer);

    for i in 0..26 {
        println!("line: {i}");
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
