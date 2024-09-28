use crate::cpu::flags::Flags;
use crate::cpu::test::run_test;
use crate::cpu::Step;

use crate::cpu::registers::Reg16;
use quickcheck::quickcheck;

fn test_dec16(opcode: u8, x: u16, reg: Reg16) -> bool {
    let mut machine = run_test(&[opcode], |machine| {
        machine.cpu.registers.write16(reg, x);
    });
    let expected = x.wrapping_sub(1);
    machine.t_cycles == 8 && machine.cpu.registers.read16(reg) == expected
}

#[test]
fn test_0b() {
    fn prop(x: u16) -> bool {
        test_dec16(0x0b, x, Reg16::BC)
    }
    quickcheck(prop as fn(u16) -> bool);
}

#[test]
fn test_0b_overflow() {
    assert!(test_dec16(0x0b, 0x0000, Reg16::BC))
}

#[test]
fn test_1b() {
    fn prop(x: u16) -> bool {
        test_dec16(0x1b, x, Reg16::DE)
    }
    quickcheck(prop as fn(u16) -> bool);
}

#[test]
fn test_1b_overflow() {
    assert!(test_dec16(0x1b, 0x0000, Reg16::DE))
}

#[test]
fn test_2b() {
    fn prop(x: u16) -> bool {
        test_dec16(0x2b, x, Reg16::HL)
    }
    quickcheck(prop as fn(u16) -> bool);
}

#[test]
fn test_2b_overflow() {
    assert!(test_dec16(0x2b, 0x0000, Reg16::HL))
}

#[test]
fn test_3b() {
    fn prop(x: u16) -> bool {
        test_dec16(0x3b, x, Reg16::SP)
    }
    quickcheck(prop as fn(u16) -> bool);
}

#[test]
fn test_3b_overflow() {
    assert!(test_dec16(0x3b, 0x0000, Reg16::SP))
}
