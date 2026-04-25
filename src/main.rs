mod parser;

use std::process::Command;

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
    Command::new("gcc")
        .args(["-c", "tests/test_add.c", "-o", "tests/test_add.o"])
        .status()?;

    Command::new("gcc")
        .args(["-c", "tests/test_main.c", "-o", "tests/test_main.o"])
        .status()?;

    let add_syms = parser::parse_symbols("tests/test_add.o")?;
    let main_syms = parser::parse_symbols("tests/test_main.o")?;

    println!("=== ALL SYMBOLS: test_add.o ===");
    for symb in &add_syms {
        println!(
            "{} {:?} is_defined:{}",
            symb.name, symb.bind, symb.is_defined
        );
    }

    println!("\n=== ALL SYMBOLS: test_main.o ===");
    for symb in &main_syms {
        println!(
            "{} {:?} is_defined:{}",
            symb.name, symb.bind, symb.is_defined
        );
    }

    println!("\n=== test_add.o ===");
    println!("Local:");
    for s in parser::get_local_symbols("tests/test_add.o")? {
        println!("  {} is_defined:{}", s.name, s.is_defined);
    }
    println!("Global:");
    for s in parser::get_global_symbols("tests/test_add.o")? {
        println!("  {} is_defined:{}", s.name, s.is_defined);
    }
    println!("External:");
    for s in parser::get_external_symbols("tests/test_add.o")? {
        println!("  {} is_defined:{}", s.name, s.is_defined);
    }

    println!("\n=== test_main.o ===");
    println!("Local:");
    for s in parser::get_local_symbols("tests/test_main.o")? {
        println!("  {} is_defined:{}", s.name, s.is_defined);
    }
    println!("Global:");
    for s in parser::get_global_symbols("tests/test_main.o")? {
        println!("  {} is_defined:{}", s.name, s.is_defined);
    }
    println!("External:");
    for s in parser::get_external_symbols("tests/test_main.o")? {
        println!("  {} is_defined:{}", s.name, s.is_defined);
    }

    Ok(())
}
