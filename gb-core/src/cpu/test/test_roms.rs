use std::fs;

use crate::cpu::test::run_test;
use crate::cpu::flags::Flags;
use crate::cpu::Step;

use quickcheck::quickcheck;
use crate::cpu::registers::Reg16;
use zip::read::ZipFile;
use zip::result::ZipError;
use std::io::Read;


#[test]
fn test_70() {
    let mut machine = run_test(
        &load_rom("fds", "ads"), // LD (HL), B
        |machine| {
            machine.cpu.registers.b = 0x42;
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x42);
}


pub fn load_rom(zip_file: &str, rom_name: &str) -> Vec<u8> {
    let file = fs::File::open(&zip_file).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    let bytes = match archive.by_name(rom_name) {
        Ok(rom_file) => {
            rom_file.bytes()
        }
        Err(_) => { panic!() }
    };
    let data: Result<Vec<_>, _> = bytes.collect();
    data.unwrap()
}