//! # How to build libusb.a
//!
//! - Clone Mikan-OS repo & checkout tag day06c
//!
//! ```
//! gh repo clone uchan-nos/mikanos
//! cd mikanos
//! git checkout tags/osbook_day06c
//! ```
//!
//! - Download x86_64-elf libraries
//!
//! ```
//! curl -LO https://github.com/uchan-nos/mikanos-build/releases/download/v2.0/x86_64-elf.tar.gz
//! tar xvf x86_64-elf.tar.gz
//! ```
//!
//! - Initialize cargo
//!
//! ```
//! cargo init --lib
//! ```
//!
//! - Add .cargo/config.toml
//!
//! ```
//! [build]
//! target = "x86_64-unknown-none"
//! ```
//!
//! - Edit Cargo.toml & src/lib.rs
//!
//! ```
//! [package]
//! name = "usb"
//! version = "0.1.0"
//! edition = "2024"
//!
//! [lib]
//! crate-type = ["rlib"]
//!
//! [dependencies]
//!
//! [build-dependencies]
//! cc = "=1.11.1"
//!
//! [profile.dev]
//! panic = "abort"
//!
//! [profile.release]
//! panic = "abort"
//! ```
//!
//! ```
//! // src/lib.rs
//! #![no_std]
//! ```
//!
//! - Add kernel/usb/lib.cpp
//!
//! ```
//! #include "./classdriver/mouse.hpp"
//!
//! typedef void (*mouse_observer)(int8_t, int8_t);
//!
//! extern "C" {
//!   void RegisterMouseObserver(mouse_observer mouse_observer) {
//!     usb::HIDMouseDriver::default_observer = mouse_observer;
//!   }
//! }
//! ```
//!
//! - Add build.rs
//!
//! ```
//! fn main() {
//!     println!("cargo:rustc-link-search=./x86_64-elf/lib");
//!     println!("cargo:rustc-link-lib=c++");
//!     println!("cargo:rustc-link-lib=c");
//!
//!     let files = [
//!         "./kernel/usb/lib.cpp",
//!         // "./kernel/pci.cpp",
//!         // "./kernel/libcxx_support.cpp",
//!         "./kernel/usb/memory.cpp",
//!         "./kernel/usb/device.cpp",
//!         "./kernel/usb/xhci/ring.cpp",
//!         "./kernel/usb/xhci/trb.cpp",
//!         "./kernel/usb/xhci/xhci.cpp",
//!         "./kernel/usb/xhci/port.cpp",
//!         "./kernel/usb/xhci/device.cpp",
//!         "./kernel/usb/xhci/devmgr.cpp",
//!         "./kernel/usb/xhci/registers.cpp",
//!         "./kernel/usb/classdriver/base.cpp",
//!         "./kernel/usb/classdriver/hid.cpp",
//!         "./kernel/usb/classdriver/keyboard.cpp",
//!         "./kernel/usb/classdriver/mouse.cpp",
//!     ];
//!
//!     cc::Build::new()
//!         .include("./kernel")
//!         .include("./x86_64-elf/include/c++/v1")
//!         .include("./x86_64-elf/include")
//!         .cpp(true)
//!         .std("c++17")
//!         .flag("-O2")
//!         .flag("-mno-red-zone")
//!         .flag("-ffreestanding")
//!         .flag("-fno-exceptions")
//!         .flag("-fno-rtti")
//!         .flag("-fpermissive")
//!         .flag("-Wno-unused-parameter")
//!         .flag("-Wno-sign-compare")
//!         .define("__ELF__", None)
//!         .compiler("clang++")
//!         .no_default_flags(true)
//!         .cpp_link_stdlib(None)
//!         .cargo_warnings(false)
//!         .files(files.iter())
//!         .compile("usb");
//! }
//! ```
//!
//! - Build and copy libusb.a
//!
//! ```
//! cargo b --release
//! cp target/x86_64-unknown-none/release/build/usb/{random number}/out/libusb.a /path/to/mikan/kernel/lib
//! ```
//!
//! - Generate bindings.rs
//!
//! ```
//! bindgen -o src/bindings.rs --use-core --no-layout-tests --allowlist-item .*Controller.*  kernel/usb/xhci/xhci.hpp  -- -I kernel
//! cp src/bindings.rs /path/to/mikan/kernel/src/pci/usb
//! ```

use crate::{
    pci::usb::xhci::{usb_xhci_Controller, usb_xhci_Controller_Initialize},
    Result,
};

mod xhci;

pub struct Controller {
    inner: usb_xhci_Controller,
}

impl Controller {
    pub fn new(mmio_base: usize) -> Self {
        Self {
            inner: unsafe { usb_xhci_Controller::new(mmio_base) },
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        if !unsafe { usb_xhci_Controller_Initialize(&mut self.inner).code_ != 0 } {
            return Err("failed to initialize controller");
        }
        Ok(())
    }

    pub fn run(&self) {
        todo!()
    }
}
