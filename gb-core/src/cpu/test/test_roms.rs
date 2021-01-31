
use std::fs;

use crate::cpu::test::run_test;
use crate::cpu::flags::Flags;
use crate::cpu::Step;

use quickcheck::quickcheck;
use crate::cpu::registers::Reg16;



pub fn load_rom(zip_file: &str, rom_name: &str) {
    let file = fs::File::open(&fname).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

   // archive.iter().filter( |files| )
}