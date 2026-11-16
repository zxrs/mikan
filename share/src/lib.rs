#![no_std]

use core::cell::RefCell;

pub mod frame_buffer;
pub mod memory_map;
pub mod x86;

pub struct Global<T: Sync> {
    pub inner: RefCell<Option<T>>,
}

unsafe impl<T: Sync> Sync for Global<T> {}

impl<T: Sync> Default for Global<T> {
    fn default() -> Self {
        Global::new()
    }
}

impl<T: Sync> Global<T> {
    pub const fn new() -> Self {
        Self {
            inner: RefCell::new(None),
        }
    }
}
