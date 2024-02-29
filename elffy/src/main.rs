extern crate elf_rs;

use std::io::Read;
use std::fs::File;
use std::env;

use elf_rs::*;

fn main() {

    let file = std::env::args().last().unwrap();
    
    let mut elf_file = File::open(file).expect("open file failed");
    let mut elf_buf = Vec::<u8>::new();
    elf_file.read_to_end(&mut elf_buf).expect("read file failed");

    let elf = Elf::from_bytes(&elf_buf).expect("load elf file failed");

    println!("{:?} header: {:?}", elf, elf.elf_header());

    for p in elf.program_header_iter() {
        println!("{:x?}", p);
    }

    for s in elf.section_header_iter() {
        println!("{:x?}", s);
    }

    let s = elf.lookup_section(b".text");
    println!("s {:?}", s);
}
