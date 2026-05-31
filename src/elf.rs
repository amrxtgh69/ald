#[allow(dead_code)]
#[derive(Debug)]
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

#[allow(dead_code)]
#[derive(Debug)]
pub struct ElfSection {
    pub name: String,
    pub sh_type: u32,
    pub flags: u64,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub addralign: u64,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolBind {
    Local,
    Global,
    Weak,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SymbolDef {
    pub name: String,
    pub index: u32,
    pub value: u64,
    pub bind: SymbolBind,
    pub is_defined: bool,
}
