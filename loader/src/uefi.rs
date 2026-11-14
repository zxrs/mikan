use core::{cell::RefCell, fmt, mem::offset_of, ptr};

use share::memory_map::{EFIPhysicalAddress, MemoryMap};

pub type EFIHandle = *const u8;

#[repr(C)]
pub struct EFIGuid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

const EFI_GOP_GUID: EFIGuid = EFIGuid {
    data1: 0x9042a9de,
    data2: 0x23dc,
    data3: 0x4a38,
    data4: [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
};

const EFI_LOADED_IMAGE_PROTOCOL_GUID: EFIGuid = EFIGuid {
    data1: 0x5B1B31A1,
    data2: 0x9562,
    data3: 0x11d2,
    data4: [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
};

const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID: EFIGuid = EFIGuid {
    data1: 0x0964e5b22,
    data2: 0x6459,
    data3: 0x11d2,
    data4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
};
pub trait Guid {
    fn guid() -> EFIGuid;
}

pub static WRITER: Global<TextWriter> = Global::new();

pub fn init_writer(system_table: &'static EFISystemTable) {
    let protocol = system_table.con_out;
    *WRITER.inner.borrow_mut() = Some(TextWriter { protocol })
}

pub struct Global<T: Sync> {
    pub inner: RefCell<Option<T>>,
}

unsafe impl<T: Sync> Sync for Global<T> {}

impl<T: Sync> Global<T> {
    const fn new() -> Self {
        Self {
            inner: RefCell::new(None),
        }
    }
}

#[repr(transparent)]
pub struct EFIStatus(usize);

impl EFIStatus {
    fn is_success(&self) -> bool {
        self.0 == 0
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EFITableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    #[doc(hidden)]
    _reserved: u32,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct CChar(*const u16);

impl CChar {
    pub fn from(p: *const u16) -> Self {
        Self(p)
    }

    fn iter(self) -> CCharIter {
        CCharIter::new(self)
    }
}

impl fmt::Display for CChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        char::decode_utf16(self.iter())
            .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
            .for_each(|c| {
                _ = write!(f, "{c}");
            });
        Ok(())
    }
}

struct CCharIter {
    offset: usize,
    inner: CChar,
}

impl CCharIter {
    fn new(cchar: CChar) -> Self {
        Self {
            offset: 0,
            inner: cchar,
        }
    }
}

impl Iterator for CCharIter {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { *self.inner.0.add(self.offset) } == 0 {
            return None;
        }
        let result = unsafe { *self.inner.0.add(self.offset) };
        self.offset += 1;
        Some(result)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EFISimpleTextOutputProtocol {
    #[doc(hidden)]
    _padding0: [*const u8; 1],

    output_string: fn(*const EFISimpleTextOutputProtocol, *const u16) -> EFIStatus,

    #[doc(hidden)]
    _padding1: [*const u8; 4],

    clear_screan: fn(*const EFISimpleTextOutputProtocol) -> EFIStatus,
}

impl EFISimpleTextOutputProtocol {
    pub fn clear_screen(&self) {
        (self.clear_screan)(self);
    }
}

#[derive(Debug)]
pub struct TextWriter {
    protocol: &'static EFISimpleTextOutputProtocol,
}

unsafe impl Sync for TextWriter {}

impl TextWriter {
    fn write(&self, c: char) {
        let buf = [c as u16, 0];
        (self.protocol.output_string)(self.protocol, buf.as_ptr());
    }
}

impl fmt::Write for TextWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.chars().for_each(|c| {
            if c == '\n' {
                self.write('\r');
            }
            self.write(c);
        });
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EFIGopMode {
    pub max_mode: u32,
    pub mode: u32,
    _info: *const u8,
    pub size_of_info: usize,
    pub frame_buffer_base: EFIPhysicalAddress,
    pub frame_buffer_size: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct EFIGop<'a> {
    #[doc(hidden)]
    _padding0: [*const u8; 3],

    pub mode: &'a EFIGopMode,
}

impl<'a> Guid for EFIGop<'a> {
    fn guid() -> EFIGuid {
        EFI_GOP_GUID
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EFILoadedImageProtocol {
    pub revision: u32,

    #[doc(hidden)]
    _padding0: [*const u8; 2],

    pub device_handle: EFIHandle,
}

impl Guid for EFILoadedImageProtocol {
    fn guid() -> EFIGuid {
        EFI_LOADED_IMAGE_PROTOCOL_GUID
    }
}

#[repr(u64)]
pub enum EFIFileMode {
    Read = 0x0000000000000001,
    Write = 0x0000000000000002,
    Create = 0x8000000000000000,
}

#[repr(u64)]
pub enum EFIFileAttributes {
    Default = 0,
    ReadOnly = 0x0000000000000001,
    Hidden = 0x0000000000000002,
    System = 0x0000000000000004,
    Reserved = 0x0000000000000008,
    Directory = 0x0000000000000010,
    Archive = 0x0000000000000020,
    ValidAttr = 0x0000000000000037,
}

#[repr(C)]
#[derive(Debug)]
pub struct EFIFileProtocol {
    pub revision: u64,
    open: fn(*const EFIFileProtocol, *mut *mut EFIFileProtocol, CChar, u64, u64) -> EFIStatus,
}

impl EFIFileProtocol {
    pub fn open(
        &self,
        file_name: CChar,
        mode: EFIFileMode,
        attributes: EFIFileAttributes,
    ) -> &EFIFileProtocol {
        let mut p = ptr::null_mut();
        if !(self.open)(self, &mut p, file_name, mode as _, attributes as _).is_success() {
            panic!("failed to open: {}", file_name);
        }
        unsafe { &*(p as *const EFIFileProtocol) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EFISimpleFileSystemProtocol {
    pub revision: u64,
    open_volumu: fn(*const EFISimpleFileSystemProtocol, *mut *mut EFIFileProtocol) -> EFIStatus,
}

impl Guid for EFISimpleFileSystemProtocol {
    fn guid() -> EFIGuid {
        EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID
    }
}

impl EFISimpleFileSystemProtocol {
    pub fn open_volume(&self) -> &EFIFileProtocol {
        let mut p = ptr::null_mut();
        if !(self.open_volumu)(self, &mut p).is_success() {
            panic!("failed to open volume");
        }
        unsafe { &*(p as *const EFIFileProtocol) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EFIBootServices {
    pub header: EFITableHeader,

    #[doc(hidden)]
    _padding0: [*const u8; 4],

    get_memory_map: fn(*mut usize, *mut u8, *mut usize, *mut usize, *mut u32) -> EFIStatus,

    _padding1: [*const u8; 27],

    open_protocol:
        fn(EFIHandle, *const EFIGuid, *mut *mut u8, EFIHandle, EFIHandle, u32) -> EFIStatus,

    _padding2: [*const u8; 4],

    locate_protocol: fn(*const EFIGuid, *const u8, *mut *mut u8) -> EFIStatus,
}

const _: () = {
    ["get_memory_map"][offset_of!(EFIBootServices, get_memory_map) - 56];
    ["open_protocol"][offset_of!(EFIBootServices, open_protocol) - 280];
    ["locate_protocol"][offset_of!(EFIBootServices, locate_protocol) - 320];
};

impl EFIBootServices {
    const EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL: u32 = 0x00000001;

    pub fn get_memory_map(&self) -> MemoryMap {
        let mut mmap = MemoryMap::default();
        if !(self.get_memory_map)(
            &mut mmap.map_size,
            mmap.buffer.as_mut_ptr(),
            &mut mmap.map_key,
            &mut mmap.descriptor_size,
            &mut mmap.descriptor_version,
        )
        .is_success()
        {
            panic!("failed to get memory map");
        }
        mmap
    }

    pub fn open_protocol<T: Guid>(&self, handle: EFIHandle, agent_handle: EFIHandle) -> &T {
        let mut p = ptr::null_mut();
        if !(self.open_protocol)(
            handle,
            &T::guid(),
            &mut p,
            agent_handle,
            ptr::null(),
            Self::EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
        )
        .is_success()
        {
            panic!("failed to open protocol: {}", stringify!(T));
        }
        unsafe { &*(p as *const T) }
    }

    pub fn locate_protocol<T: Guid>(&self) -> &T {
        let mut p = ptr::null_mut();
        if !(self.locate_protocol)(&T::guid(), ptr::null(), &mut p).is_success() {
            panic!("failed to locate protocol");
        }
        unsafe { &*(p as *const T) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EFISystemTable {
    pub header: EFITableHeader,
    pub firmware_vendor: CChar,
    pub firmware_version: u32,

    #[doc(hidden)]
    _padding0: [*const u8; 3],

    pub con_out: &'static EFISimpleTextOutputProtocol,

    #[doc(hidden)]
    _padding1: [*const u8; 3],

    pub boot_services: &'static EFIBootServices,
}

const _: () = {
    ["boot_services"][offset_of!(EFISystemTable, boot_services) - 96];
};
