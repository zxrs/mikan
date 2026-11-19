#![no_std]
#![no_main]

#[macro_use]
mod console;
mod font;
mod graphics;
mod pci;

use core::fmt::Write;
use share::{frame_buffer::FrameBufferConfig, x86};

use crate::{
    console::CONSOLE,
    graphics::{PixelColor, PixelWriter},
    pci::{DEVICES, read_vendor_id, scan_all_bus},
};

type Result<T> = core::result::Result<T, &'static str>;

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
    pci::init();

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

    pixel_writer.fill_rectangle((200, 150).into(), (200, 150).into(), (128, 0, 128).into());
    pixel_writer.draw_rectangele((200, 150).into(), (150, 120).into(), (255, 255, 255).into());

    scan_all_bus().unwrap();
    let Some(ref devices) = *DEVICES.inner.borrow() else {
        panic!("failed to get devices")
    };

    devices.iter().filter_map(|d| d.as_ref()).for_each(|d| {
        let vendor_id = read_vendor_id(d.bus, d.device, d.function);
        println!("0x{:x}", vendor_id);
    });

    loop {
        x86::halt();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{:?}", info);
    loop {
        x86::halt();
    }
}
