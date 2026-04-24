mod parser;

use std::process::Command;

#[derive(Debug)]
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

    println!("=== test_add.o ===");
    for s in &add_syms {
        println!("{} {:?}", s.name, s.bind);
    }

    println!("=== test_main.o ===");
    for s in &main_syms {
        println!("{} {:?}", s.name, s.bind);
    }

    Ok(())
}
