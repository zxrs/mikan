#![no_std]
#![no_main]

#[macro_use]
mod console;
mod font;
mod graphics;
mod pci;

use core::fmt::Write;
use share::{frame_buffer::FrameBufferConfig, x86};
use usb::Controller;

use crate::{
    console::CONSOLE,
    graphics::{PixelColor, PixelWriter},
    pci::{DEVICES, read_bar, read_class_code, read_vendor_id, scan_all_bus},
};

type Result<T> = core::result::Result<T, &'static str>;

type MouseObserver = extern "C" fn(i8, i8);

unsafe extern "C" {
    fn RegisterMouseObserver(cb: MouseObserver);
}

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

extern "C" fn mouse_observer(x: i8, y: i8) {
    println!("x: {x}, y: {y}");
}

#[unsafe(no_mangle)]
extern "C" fn kernel_main(config: &'static mut FrameBufferConfig) -> ! {
    let mut pixel_writer = PixelWriter::new(config);
    console::init(pixel_writer);
    pci::init();

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
        let class_code = read_class_code(d.bus, d.device, d.function);
        println!(
            "vendor id: 0x{:x}, class code: 0x{:x}, 0x{:x}, 0x{:x}",
            vendor_id, class_code.base, class_code.sub, class_code.interface
        );
    });

    let device = devices
        .iter()
        .find_map(|d| {
            let d = d.as_ref()?;
            if d.class_code.match_base_sub_interface(0x0c, 0x03, 0x30)
                && read_vendor_id(d.bus, d.device, d.function) == 0x1033
            // for QEMU nec-usb-xhci
            {
                return Some(d);
            }
            None
        })
        .unwrap();

    let xhc_bar = read_bar(device, 0).unwrap();
    println!("xhc_bar: {:b}", xhc_bar);

    let mmio_base = xhc_bar & !0xf;
    println!("mmio_base: {:b}", mmio_base);

    let mut xhc = Controller::new(mmio_base as _);
    xhc.initialize().unwrap();
    xhc.run().unwrap();

    unsafe { RegisterMouseObserver(mouse_observer) };

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
