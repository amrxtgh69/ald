use goblin::elf::Elf;

use crate::elf::{ElfHeader, ElfSection, SymbolBind, SymbolDef};
use std::error::Error;

fn is_defined(sym: &goblin::elf::Sym) -> bool {
    sym.st_shndx != goblin::elf::section_header::SHN_UNDEF as usize
}

pub fn parse_symbols(elf: &Elf<'_>) -> Vec<SymbolDef> {
    let mut symbols = Vec::new();

    for (i, sym) in elf.syms.iter().enumerate() {
        let name = match elf.strtab.get_at(sym.st_name) {
            Some(n) => n.to_string(),
            None => continue,
        };
        let defined = is_defined(&sym);

        let bind = match sym.st_bind() {
            goblin::elf::sym::STB_LOCAL => SymbolBind::Local,
            goblin::elf::sym::STB_GLOBAL => SymbolBind::Global,
            goblin::elf::sym::STB_WEAK => SymbolBind::Weak,
            _ => SymbolBind::Local,
        };
        let value = sym.st_value;
        symbols.push(SymbolDef {
            name,
            index: i as u32,
            value,
            bind,
            is_defined: defined,
        });
    }
    symbols
}

pub fn get_local_symbols(symbols: &[SymbolDef]) -> Vec<SymbolDef> {
    symbols
        .iter()
        .filter(|s| s.bind == SymbolBind::Local)
        .cloned()
        .collect()
}

pub fn get_global_symbols(symbols: &[SymbolDef]) -> Vec<SymbolDef> {
    symbols
        .iter()
        .filter(|s| s.bind == SymbolBind::Global && s.is_defined)
        .cloned()
        .collect()
}

pub fn get_external_symbols(symbols: &[SymbolDef]) -> Vec<SymbolDef> {
    symbols
        .iter()
        .filter(|s| s.bind == SymbolBind::Global && !s.is_defined)
        .cloned()
        .collect()
}

pub fn parse_elf_header(elf: &Elf<'_>) -> ElfHeader {
    let hdr = &elf.header;
    ElfHeader {
        class: hdr.e_ident[4],
        endian: hdr.e_ident[5],
        file_type: hdr.e_type,
        machine: hdr.e_machine,
        entry: hdr.e_entry,
        flags: hdr.e_flags,
        ehsize: hdr.e_ehsize,
        phentsize: hdr.e_phentsize,
        phnum: hdr.e_phnum,
        shentsize: hdr.e_shentsize,
        shnum: hdr.e_shnum,
        shstrndx: hdr.e_shstrndx,
        shoff: hdr.e_shoff,
    }
}

pub fn parse_elf_sections(elf: &Elf<'_>, bytes: &[u8]) -> Result<Vec<ElfSection>, Box<dyn Error>> {
    let mut sections = Vec::new();

    for shdr in &elf.section_headers {
        let name = elf
            .shdr_strtab
            .get_at(shdr.sh_name)
            .unwrap_or("")
            .to_string();
        let start = shdr.sh_offset as usize;
        let end = start + shdr.sh_size as usize;
        if end > bytes.len() {
            return Err(format!(
                "section '{}' data at offset {} size {} exceeds file size {}",
                name,
                shdr.sh_offset,
                shdr.sh_size,
                bytes.len()
            )
            .into());
        }
        let data = bytes[start..end].to_vec();
        sections.push(ElfSection {
            name,
            sh_type: shdr.sh_type,
            flags: shdr.sh_flags,
            addr: shdr.sh_addr,
            offset: shdr.sh_offset,
            size: shdr.sh_size,
            addralign: shdr.sh_addralign,
            data,
        });
    }
    Ok(sections)
}
