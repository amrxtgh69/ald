use goblin::elf::Elf;

use std::{error::Error, fs};
use crate::SymbolDef;

fn is_defined(sym: &goblin::elf::Sym) -> bool {
    sym.st_shndx != goblin::elf::section_header::SHN_UNDEF
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
        let is_defined = is_defined(&sym);
    }
    Ok(symbols)
}
