# mikan

Mikan OS fork written in Rust

# How to create usb crate

- Clone Mikan-OS repo & checkout tag day06c

```
gh repo clone uchan-nos/mikanos
cd mikanos
git checkout tags/osbook_day06c
```

- Download x86_64-elf libraries

```
curl -LO https://github.com/uchan-nos/mikanos-build/releases/download/v2.0/x86_64-elf.tar.gz
tar xvf x86_64-elf.tar.gz
```

- Initialize cargo

```
cargo init --lib
```

- Add .cargo/config.toml

```
[build]
target = "x86_64-unknown-none"
```

- Edit Cargo.toml & src/lib.rs

```
# Cargo.toml
[package]
name = "usb"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["rlib"]

[dependencies]

[build-dependencies]
cc = "=1.11.2"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

```
// src/lib.rs
#![no_std]

mod shim;

type Result<T> = core::result::Result<T, &'static str>;

use crate::shim::{usb_xhci_Controller, usb_xhci_Controller_Initialize, usb_xhci_Controller_Run};

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
        if unsafe { usb_xhci_Controller_Initialize(&mut self.inner).code_ > 0 } {
            return Err("failed to initialize controller");
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        if unsafe { usb_xhci_Controller_Run(&mut self.inner).code_ > 0 } {
            return Err("failed to run controller");
        }
        Ok(())
    }
}
```

```
// src/shim.rs
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

include!("bindings.rs");
```

- Add kernel/usb/lib.cpp

```
#include "./classdriver/mouse.hpp"

typedef void (*mouse_observer)(int8_t, int8_t);

extern "C" {
  void RegisterMouseObserver(mouse_observer mouse_observer) {
    usb::HIDMouseDriver::default_observer = mouse_observer;
  }
}

enum LogLevel {
  kError = 3,
  kWarn  = 4,
  kInfo  = 6,
  kDebug = 7,
};

int Log(LogLevel level, const char* format, ...) {
  return 0;
}

extern "C" void __cxa_pure_virtual() {
  while (1) __asm__("hlt");
}
```

- Add build.rs

```
fn main() {
    println!("cargo:rustc-link-search=./x86_64-elf/lib");
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=c");

    let files = [
        "./kernel/usb/lib.cpp",
        // "./kernel/pci.cpp",
        "./kernel/libcxx_support.cpp",
        "./kernel/usb/memory.cpp",
        "./kernel/usb/device.cpp",
        "./kernel/usb/xhci/ring.cpp",
        "./kernel/usb/xhci/trb.cpp",
        "./kernel/usb/xhci/xhci.cpp",
        "./kernel/usb/xhci/port.cpp",
        "./kernel/usb/xhci/device.cpp",
        "./kernel/usb/xhci/devmgr.cpp",
        "./kernel/usb/xhci/registers.cpp",
        "./kernel/usb/classdriver/base.cpp",
        "./kernel/usb/classdriver/hid.cpp",
        "./kernel/usb/classdriver/keyboard.cpp",
        "./kernel/usb/classdriver/mouse.cpp",
    ];

    cc::Build::new()
        .include("./kernel")
        .include("./x86_64-elf/include/c++/v1")
        .include("./x86_64-elf/include")
        .cpp(true)
        .std("c++17")
        .flag("-O2")
        .flag("-mno-red-zone")
        .flag("-ffreestanding")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .flag("-fpermissive")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-sign-compare")
        .define("__ELF__", None)
        .compiler("clang++")
        .no_default_flags(true)
        .cpp_link_stdlib(None)
        .cargo_warnings(false)
        .files(files.iter())
        .compile("usb");

    cc::Build::new()
        .file("./kernel/newlib_support.c")
        .include("./x86_64-elf/include")
        .flag("-O2")
        .flag("-ffreestanding")
        .flag("-mno-red-zone")
        .flag("-nostdlibinc")
        .flag("-Wno-unused-parameter")
        .no_default_flags(true)
        .define("__ELF__", None)
        .cargo_warnings(false)
        .compiler("clang")
        .compile("newlib_support");
}
```

- Generate bindings.rs

```
bindgen -o src/bindings.rs --use-core --no-layout-tests --allowlist-item .*Controller.*  kernel/usb/xhci/xhci.hpp  -- -I kernel
```
