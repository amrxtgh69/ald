use std::process::Command;

//////////////  
// ~amrxtgh //
//////////////

enum SymbolBind { LOCAL, GLOBAL, WEAK }
struct Symbol {
    // core identification
    name: String,
    index: u32,
    // address/value (changes during linking)
    value: u64,

    // Symbol characteristics
    bind: SymbolBind,
}

fn main() {
    Command::new("gcc")
        .args(&["-c", "tests/test_add.c", "-o", "tests/test_add.o"])
        .status()
        .unwrap();
    
    Command::new("gcc")
        .args(&["-c", "tests/test_main.c", "-o", "tests/test_main.o"])
        .status()
        .unwrap();
    
    println!("cargo:rerun-if-changed=tests/test_add.c");
    println!("cargo:rerun-if-changed=tests/test_main.c");
}
