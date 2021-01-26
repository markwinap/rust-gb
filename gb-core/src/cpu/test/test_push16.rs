
use crate::cpu::test::run_test;
use crate::cpu::flags::Flags;
use crate::cpu::Step;

use quickcheck::quickcheck;
use crate::cpu::registers::Reg16;


fn test_push16(opcode: u8, reg: Reg16, x: u16) -> bool {
    let machine = run_test(&[opcode, 0xed, 0x00, 0x00], |machine| {
        machine.cpu.registers.write16(reg, x);
        machine.cpu.registers.sp = 0x0004;
    });
    machine.t_cycles == 16
        && machine.cpu.registers.sp == 0x0002
        && machine.cpu.interface.memory[0x03] == (x >> 8) as u8
        && machine.cpu.interface.memory[0x02] == (x as u8)
}

#[test]
fn test_c5() {
    fn prop(x: u16) -> bool {
        test_push16(0xc5, Reg16::BC, x)
    }
    quickcheck(prop as fn(u16) -> bool);
}

#[test]
fn test_d5() {
    fn prop(x: u16) -> bool {
        test_push16(0xd5, Reg16::DE, x)
    }
    quickcheck(prop as fn(u16) -> bool);
}

#[test]
fn test_e5() {
    fn prop(x: u16) -> bool {
        test_push16(0xe5, Reg16::HL, x)
    }
    quickcheck(prop as fn(u16) -> bool);
}

#[test]
fn test_f5() {
    fn prop(a: u8, f: u8) -> bool {
        let machine = run_test(&[0xf5, 0xed, 0x00, 0x00], |machine| {
            machine.cpu.registers.a = a;
            machine.cpu.registers.flags = Flags::from(f);
            machine.cpu.registers.sp = 0x0004;
        });
        machine.t_cycles == 16
            && machine.cpu.registers.sp == 0x0002
            && machine.cpu.interface.memory[0x03] == a
            && machine.cpu.interface.memory[0x02] == (f & 0xF0)
    }
    quickcheck(prop as fn(u8, u8) -> bool);
}
