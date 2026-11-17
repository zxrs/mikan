#![no_std]
#![no_main]

#[macro_use]
mod console;
mod font;
mod graphics;

use core::fmt::Write;
use share::{frame_buffer::FrameBufferConfig, x86};

use crate::{
    console::CONSOLE,
    graphics::{PixelColor, PixelWriter},
};

const MOUSE_CURSOR_SHAPE: [&str; 24] = [
    "@              ",
    "@@             ",
    "@.@            ",
    "@..@           ",
    "@...@          ",
    "@....@         ",
    "@.....@        ",
    "@......@       ",
    "@.......@      ",
    "@........@     ",
    "@.........@    ",
    "@..........@   ",
    "@...........@  ",
    "@............@ ",
    "@......@@@@@@@@",
    "@......@       ",
    "@....@@.@      ",
    "@...@ @.@      ",
    "@..@   @.@     ",
    "@.@    @.@     ",
    "@@      @.@    ",
    "@       @.@    ",
    "         @.@   ",
    "         @@@   ",
];

#[unsafe(no_mangle)]
extern "C" fn kernel_main(config: &'static mut FrameBufferConfig) -> ! {
    let mut pixel_writer = PixelWriter::new(config);
    console::init(pixel_writer);

    for i in 0..26 {
        println!("line: {i}");
    }

    MOUSE_CURSOR_SHAPE.iter().enumerate().for_each(|(dy, row)| {
        row.chars().enumerate().for_each(|(dx, c)| {
            if c == '@' {
                pixel_writer.write(200 + dx as u32, 100 + dy as u32, PixelColor::BLACK);
            }
            if c == '.' {
                pixel_writer.write(200 + dx as u32, 100 + dy as u32, PixelColor::WHITE);
            }
        });
    });

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
