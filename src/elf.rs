#[allow(dead_code)]
pub struct ElfHeader {
    pub class: u8,
    pub endian: u8,
    pub file_type: u16,
    pub machine: u16,
    pub entry: u64,
    pub flags: u32,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shentsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
    pub shoff: u64,
}
