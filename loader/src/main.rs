#![no_std]
#![no_main]

#[macro_use]
mod macros;
mod elf;
mod uefi;

use crate::{
    elf::{ElfHeader, calc_load_address_range, copy_load_segments},
    uefi::{
        CChar, EFIAllocateType, EFIFileAttributes, EFIFileInfo, EFIFileMode, EFIGop, EFIHandle,
        EFILoadedImageProtocol, EFISimpleFileSystemProtocol, EFISystemTable, WRITER, init_writer,
    },
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

    const FILE_INFO_SIZE: usize = size_of::<EFIFileInfo>() + size_of::<u16>() * 12;
    let mut file_info_buffer = [0u8; FILE_INFO_SIZE];

    let file_info: &EFIFileInfo = kernel_file.get_info(&mut file_info_buffer);
    let kernel_file_size = file_info.file_size;
    println!("{kernel_file_size}");

    let kernel_buffer = system_table
        .boot_services
        .allocate_pool(EFIMemoryType::LoaderData, kernel_file_size as _);

    kernel_file.read(kernel_buffer);

    let kernel_elf_header = unsafe { &*(kernel_buffer.as_ptr() as *const ElfHeader) };
    let (first, last) = calc_load_address_range(kernel_elf_header);
    let num_pages = (last - first).div_ceil(0x1000);
    println!("{num_pages}");

    let kernel_base_address = system_table.boot_services.allocate_pages(
        EFIAllocateType::AllocateMaxAddress,
        EFIMemoryType::LoaderData,
        num_pages as _,
        first,
    );

    println!("0x{:x}", kernel_base_address);
    println!("0x{:x}", kernel_elf_header.entry_addr());

    copy_load_segments(system_table, kernel_elf_header);

    let entry_point = unsafe {
        core::mem::transmute::<*const u8, extern "sysv64" fn() -> !>(
            kernel_elf_header.entry_addr() as *const u8
        )
    };

    if !system_table
        .boot_services
        .exit_boot_services(image_handle, mmap.map_key)
        .is_success()
    {
        let mmap = system_table.boot_services.get_memory_map();
        if !system_table
            .boot_services
            .exit_boot_services(image_handle, mmap.map_key)
            .is_success()
        {
            panic!("failed to exit boot services");
        }
    }

    entry_point();

    // loop {
    //     x86::halt();
    // }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");
    loop {
        x86::halt();
    }
}
