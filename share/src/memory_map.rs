pub type EFIPhysicalAddress = u64;
pub type EFIVirtualAddress = u64;

const MEMORY_MAP_SIZE: usize = 4096 * 4;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum EFIMemoryType {
    ReservedMemoryType,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    UnacceptedMemoryType,
    MaxMemoryType,
}

#[repr(C)]
#[derive(Debug)]
pub struct EFIMemoryDescriptor {
    pub typ: EFIMemoryType,
    pub physical_start: EFIPhysicalAddress,
    pub virtual_start: EFIVirtualAddress,
    pub number_of_pages: u64,
    pub attribute: u64,
}

#[derive(Debug)]
pub struct MemoryMap {
    pub buffer: [u8; MEMORY_MAP_SIZE],
    pub map_size: usize,
    pub map_key: usize,
    pub descriptor_size: usize,
    pub descriptor_version: u32,
}

impl Default for MemoryMap {
    fn default() -> Self {
        Self {
            buffer: [0u8; _],
            map_size: MEMORY_MAP_SIZE,
            map_key: 0,
            descriptor_size: 0,
            descriptor_version: 0,
        }
    }
}

impl MemoryMap {
    pub fn iter(&self) -> MemoryMapIter {
        MemoryMapIter::new(self)
    }
}

pub struct MemoryMapIter<'a> {
    offset: usize,
    mmap: &'a MemoryMap,
}

impl<'a> MemoryMapIter<'a> {
    fn new(mmap: &'a MemoryMap) -> Self {
        Self { offset: 0, mmap }
    }
}

impl<'a> Iterator for MemoryMapIter<'a> {
    type Item = &'a EFIMemoryDescriptor;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.mmap.map_size {
            return None;
        }
        let descriptor =
            unsafe { &*(self.mmap.buffer.as_ptr().add(self.offset) as *const EFIMemoryDescriptor) };
        self.offset += self.mmap.descriptor_size;
        Some(descriptor)
    }
}
