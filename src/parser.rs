//! ELF Symbol Parser
//!
//! This module provides functionality for parsing symbol tables from ELF relocatable objects.

use goblin::elf::Elf;

use crate::{SymbolBind, SymbolDef};
use std::{error::Error, fs};

/// Checks if a symbol is defined (has a valid section index).
///
/// A symbol is defined if its section index is not SHN_UNDEF.
fn is_defined(sym: &goblin::elf::Sym) -> bool {
    sym.st_shndx != goblin::elf::section_header::SHN_UNDEF as usize
}

/// Parses all symbols from an ELF relocatable object file.
///
/// # Arguments
/// * `path` - Path to the ELF relocatable object file
///
/// # Returns
/// A vector of `SymbolDef` containing all parsed symbols
///
/// # Errors
/// Returns an error if the file cannot be read or parsed as ELF
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

/// Returns only local (STB_LOCAL) symbols from an ELF file.
///
/// Local symbols are only visible within the object file that defines them.
///
/// # Arguments
/// * `path` - Path to the ELF relocatable object file
///
/// # Returns
/// A vector of `SymbolDef` containing only local symbols
pub fn get_local_symbols(path: &str) -> Result<Vec<SymbolDef>, Box<dyn Error>> {
    let symbols = parse_symbols(path)?;
    Ok(symbols
        .into_iter()
        .filter(|s| s.bind == SymbolBind::Local)
        .collect())
}

/// Returns global (STB_GLOBAL) symbols that are defined in an ELF file.
///
/// Global symbols are visible across multiple object files and can be referenced
/// by other files during linking.
///
/// # Arguments
/// * `path` - Path to the ELF relocatable object file
///
/// # Returns
/// A vector of `SymbolDef` containing only defined global symbols
pub fn get_global_symbols(path: &str) -> Result<Vec<SymbolDef>, Box<dyn Error>> {
    let symbols = parse_symbols(path)?;
    Ok(symbols
        .into_iter()
        .filter(|s| s.bind == SymbolBind::Global && s.is_defined)
        .collect())
}

/// Returns external (undefined) symbols from an ELF file.
///
/// These are global symbols that are referenced but not defined in this object file.
/// They need to be resolved by the linker from other object files.
///
/// # Arguments
/// * `path` - Path to the ELF relocatable object file
///
/// # Returns
/// A vector of `SymbolDef` containing only undefined (external) symbols
pub fn get_external_symbols(path: &str) -> Result<Vec<SymbolDef>, Box<dyn Error>> {
    let symbols = parse_symbols(path)?;
    Ok(symbols
        .into_iter()
        .filter(|s| s.bind == SymbolBind::Global && !s.is_defined)
        .collect())
}
