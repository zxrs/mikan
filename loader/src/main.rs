#![no_std]
#![no_main]

#[macro_use]
mod macros;
mod uefi;

use crate::uefi::{
    CChar, EFIFileAttributes, EFIFileMode, EFIGop, EFIHandle, EFILoadedImageProtocol,
    EFISimpleFileSystemProtocol, EFISystemTable, WRITER, init_writer,
};
use core::{fmt::Write, slice};
use share::{memory_map::EFIMemoryType, x86};

#[unsafe(no_mangle)]
fn efi_main(image_handle: EFIHandle, system_table: &'static EFISystemTable) -> ! {
    system_table.con_out.clear_screen();
    init_writer(system_table);

    println!("{}", system_table.firmware_vendor);
    println!("hello");

    let gop: &EFIGop = system_table.boot_services.locate_protocol();
    println!("{:?}", gop.mode);

    let frame_buffer = unsafe {
        slice::from_raw_parts_mut(
            gop.mode.frame_buffer_base as *mut u8,
            gop.mode.frame_buffer_size,
        )
    };
    frame_buffer.fill(128);

    let mmap = system_table.boot_services.get_memory_map();
    mmap.iter()
        .filter(|d| d.typ.eq(&EFIMemoryType::ConventionalMemory))
        .for_each(|d| {
            println!("{:?}", d);
        });

    let loaded_image: &EFILoadedImageProtocol = system_table
        .boot_services
        .open_protocol(image_handle, image_handle);
    println!("{:x}", loaded_image.revision);

    let fs: &EFISimpleFileSystemProtocol = system_table
        .boot_services
        .open_protocol(loaded_image.device_handle, image_handle);

    let root_dir = fs.open_volume();

    let kernel_name = [
        'k' as u16, 'e' as u16, 'r' as u16, 'n' as u16, 'e' as u16, 'l' as u16, '.' as u16,
        'e' as u16, 'l' as u16, 'f' as u16, 0,
    ];

    let kernel_file = root_dir.open(
        CChar::from(kernel_name.as_ptr()),
        EFIFileMode::Read,
        EFIFileAttributes::Default,
    );

    loop {
        x86::halt();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");
    loop {
        x86::halt();
    }
}
