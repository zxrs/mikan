use super::Result;
use share::{
    Global,
    x86::{io_in32, io_out32},
};

pub mod usb;

pub const CONFIG_ADDRESS: u16 = 0x0cf8;
pub const CONFIG_DATA: u16 = 0x0cfc;

pub static DEVICES: Global<[Option<Device>; 32]> = Global::new();

pub fn init() {
    *DEVICES.inner.borrow_mut() = Some([None; _]);
}

#[derive(Debug, Clone, Copy)]
pub struct Device {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub header_type: u8,
    pub class_code: ClassCode,
}

#[derive(Debug, Clone, Copy)]
pub struct ClassCode {
    pub base: u8,
    pub sub: u8,
    pub interface: u8,
}

impl ClassCode {
    pub fn match_base(&self, base: u8) -> bool {
        self.base == base
    }

    pub fn match_base_sub(&self, base: u8, sub: u8) -> bool {
        self.match_base(base) && self.sub == sub
    }

    pub fn match_base_sub_interface(&self, base: u8, sub: u8, interface: u8) -> bool {
        self.match_base_sub(base, sub) && self.interface == interface
    }
}

pub fn add_device(device: Device) -> Result<()> {
    let Some(ref mut devices) = *DEVICES.inner.borrow_mut() else {
        return Err("failed to get devices");
    };
    if devices.iter().all(|d| d.is_some()) {
        return Err("DEVICES is full");
    }

    devices.rotate_left(1);
    if let Some(last) = devices.last_mut() {
        *last = Some(device);
    }
    Ok(())
}

pub fn make_address(bus: u8, device: u8, function: u8, reg_addr: u8) -> u32 {
    fn shl(x: u8, bits: usize) -> u32 {
        (x as u32) << bits
    }
    shl(1, 31) | shl(bus, 16) | shl(device, 11) | shl(function, 8) | (reg_addr as u32 & 0xfc)
}

pub fn scan_function(bus: u8, device: u8, function: u8) -> Result<()> {
    let class_code = read_class_code(bus, device, function);
    let header_type = read_header_type(bus, device, function);
    let dev = Device {
        bus,
        device,
        function,
        header_type,
        class_code,
    };
    add_device(dev)?;

    if class_code.match_base_sub(0x06, 0x04) {
        let bus_numbers = read_bus_numbers(bus, device, function);
        let secondary_bus = ((bus_numbers >> 8) & 0xff) as u8;
        return scan_bus(secondary_bus);
    }
    Ok(())
}

pub fn scan_device(bus: u8, device: u8) -> Result<()> {
    scan_function(bus, device, 0)?;
    if is_single_function_device(read_header_type(bus, device, 0)) {
        return Ok(());
    }

    for function in 1..8 {
        if read_vendor_id(bus, device, function) == 0xffff {
            continue;
        }
        scan_function(bus, device, function)?;
    }
    Ok(())
}

pub fn scan_bus(bus: u8) -> Result<()> {
    for device in 0..32 {
        if read_vendor_id(bus, device, 0) == 0xffff {
            continue;
        }
        scan_device(bus, device)?;
    }
    Ok(())
}

pub fn write_address(address: u32) {
    io_out32(CONFIG_ADDRESS, address);
}

pub fn write_data(value: u32) {
    io_out32(CONFIG_DATA, value);
}

pub fn read_data() -> u32 {
    io_in32(CONFIG_DATA)
}

pub fn read_vendor_id(bus: u8, device: u8, function: u8) -> u16 {
    write_address(make_address(bus, device, function, 0x00));
    (read_data() & 0xffff) as _
}

pub fn read_device_id(bus: u8, device: u8, function: u8) -> u16 {
    write_address(make_address(bus, device, function, 0x00));
    (read_data() >> 16) as _
}

pub fn read_header_type(bus: u8, device: u8, function: u8) -> u8 {
    write_address(make_address(bus, device, function, 0x0c));
    ((read_data() >> 16) & 0xff) as _
}

pub fn read_class_code(bus: u8, device: u8, function: u8) -> ClassCode {
    write_address(make_address(bus, device, function, 0x08));
    let reg = read_data();
    ClassCode {
        base: (reg >> 24) as _,
        sub: (reg >> 16) as _,
        interface: (reg >> 8) as _,
    }
}

pub fn read_bus_numbers(bus: u8, device: u8, function: u8) -> u32 {
    write_address(make_address(bus, device, function, 0x18));
    read_data()
}

pub fn is_single_function_device(header_type: u8) -> bool {
    (header_type & 0x80) == 0
}

pub fn scan_all_bus() -> Result<()> {
    let header_type = read_header_type(0, 0, 0);
    if is_single_function_device(header_type) {
        return scan_bus(0);
    }

    Ok(())
}
