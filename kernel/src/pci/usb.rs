use crate::{
    Result,
    pci::usb::xhci::{usb_xhci_Controller, usb_xhci_Controller_Initialize},
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
