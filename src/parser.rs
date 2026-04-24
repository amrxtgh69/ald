use std::{fs::File, io::Read};
use goblin::elf::Elf;

use crate::Symbol;

pub fn parse_object_file(path: &str) -> Result<Vec<Symbol>, Box<dyn std::error::Error>>{
    let mut file_data = Vec::new();
    File::open(path)?.read_to_end(&mut file_data)?;

    let elf = Elf::parse(&file_data);
}
