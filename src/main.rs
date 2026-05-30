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
        let syms = parser::parse_symbols(path)?;
        println!("ALL SYMBOLS:");
        for s in &syms {
            println!(
                "  {} {:?} value={:#x} is_defined:{}",
                s.name, s.bind, s.value, s.is_defined
            );
        }

        println!("Local:");
        for s in parser::get_local_symbols(path)? {
            println!("  {} is_defined:{}", s.name, s.is_defined);
        }
        println!("Global:");
        for s in parser::get_global_symbols(path)? {
            println!("  {} is_defined:{}", s.name, s.is_defined);
        }
        println!("External:");
        for s in parser::get_external_symbols(path)? {
            println!("  {} is_defined:{}", s.name, s.is_defined);
        }
    }

    Ok(())
}
