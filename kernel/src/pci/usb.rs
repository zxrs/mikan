use crate::pci::usb::xhci::usb_xhci_Controller;

mod xhci;

pub struct Controller {
    _impl: usb_xhci_Controller,
}

impl Controller {}
