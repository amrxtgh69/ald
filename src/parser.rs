use goblin::elf::Elf;

use crate::{SymbolBind, SymbolDef};
use std::{error::Error, fs};

fn is_defined(sym: &goblin::elf::Sym) -> bool {
    sym.st_shndx != goblin::elf::section_header::SHN_UNDEF as usize
}
pub fn parse_symbols(path: &str) -> Result<Vec<SymbolDef>, Box<dyn Error>> {
    let bytes = fs::read(path).unwrap();
    let elf = Elf::parse(&bytes).unwrap();

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
    Ok(symbols)
}

pub fn get_local_symbols(path: &str) -> Result<Vec<SymbolDef>, Box<dyn Error>> {
    let symbols = parse_symbols(path)?;
    Ok(symbols
        .into_iter()
        .filter(|s| s.bind == SymbolBind::Local)
        .collect())
}

pub fn get_global_symbols(path: &str) -> Result<Vec<SymbolDef>, Box<dyn Error>> {
    let symbols = parse_symbols(path)?;
    Ok(symbols
        .into_iter()
        .filter(|s| s.bind == SymbolBind::Global && s.is_defined)
        .collect())
}

pub fn get_external_symbols(path: &str) -> Result<Vec<SymbolDef>, Box<dyn Error>> {
    let symbols = parse_symbols(path)?;
    Ok(symbols
        .into_iter()
        .filter(|s| s.bind == SymbolBind::Global && !s.is_defined)
        .collect())
}
