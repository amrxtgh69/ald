mod elf;
mod parser;

use std::env;

#[derive(Debug, PartialEq)]
enum SymbolBind {
    Local,
    Global,
    Weak,
}

#[derive(Debug)]
#[allow(unused)]
struct SymbolDef {
    name: String,
    index: u32,
    value: u64,
    bind: SymbolBind,
    is_defined: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ald <file.o> [file.o ...]");
        std::process::exit(1);
    }

    for path in &args[1..] {
        println!("\n=== {} ===", path);
        let bytes = std::fs::read(path)?;

        let hdr = parser::parse_elf_header(&bytes)?;
        println!("ELF Header:");
        println!(
            "  class={} endian={} type={:#x} machine={:#x} entry={:#x}",
            hdr.class, hdr.endian, hdr.file_type, hdr.machine, hdr.entry
        );
        println!(
            "  shoff={} shnum={} shstrndx={}",
            hdr.shoff, hdr.shnum, hdr.shstrndx
        );

        let syms = parser::parse_symbols(&bytes)?;
        println!("ALL SYMBOLS:");
        for s in &syms {
            println!(
                "  {} {:?} value={:#x} is_defined:{}",
                s.name, s.bind, s.value, s.is_defined
            );
        }

        println!("Local:");
        for s in parser::get_local_symbols(&bytes)? {
            println!("  {} is_defined:{}", s.name, s.is_defined);
        }
        println!("Global:");
        for s in parser::get_global_symbols(&bytes)? {
            println!("  {} is_defined:{}", s.name, s.is_defined);
        }
        println!("External:");
        for s in parser::get_external_symbols(&bytes)? {
            println!("  {} is_defined:{}", s.name, s.is_defined);
        }
    }

    Ok(())
}
