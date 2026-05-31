mod elf;
mod parser;
mod resolve;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ald <file.o> [file.o ...]");
        std::process::exit(1);
    }

    for path in &args[1..] {
        println!("\n=== {} ===", path);
        let bytes = std::fs::read(path)?;
        let elf = goblin::elf::Elf::parse(&bytes)?;

        let hdr = parser::parse_elf_header(&elf);
        println!("ELF Header:");
        println!(
            "  class={} endian={} type={:#x} machine={:#x} entry={:#x}",
            hdr.class, hdr.endian, hdr.file_type, hdr.machine, hdr.entry
        );
        println!(
            "  shoff={} shnum={} shstrndx={}",
            hdr.shoff, hdr.shnum, hdr.shstrndx
        );

        let syms = parser::parse_symbols(&elf);
        println!("ALL SYMBOLS:");
        for s in &syms {
            println!(
                "  {} {:?} value={:#x} is_defined:{}",
                s.name, s.bind, s.value, s.is_defined
            );
        }

        println!("Local:");
        for s in parser::get_local_symbols(&syms) {
            println!("  {} is_defined:{}", s.name, s.is_defined);
        }
        println!("Global:");
        for s in parser::get_global_symbols(&syms) {
            println!("  {} is_defined:{}", s.name, s.is_defined);
        }
        println!("External:");
        for s in parser::get_external_symbols(&syms) {
            println!("  {} is_defined:{}", s.name, s.is_defined);
        }

        let sections = parser::parse_elf_sections(&elf, &bytes)?;
        println!("SECTIONS:");
        for s in sections {
            println!(
                "  {} type={:#x} flags={:#x} addr={:#x} size={} align={}",
                s.name, s.sh_type, s.flags, s.addr, s.size, s.addralign
            );
        }
    }

    Ok(())
}
