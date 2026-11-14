#![no_std]
#![no_main]

use share::x86;

#[unsafe(no_mangle)]
extern "C" fn kernel_main() -> ! {
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
