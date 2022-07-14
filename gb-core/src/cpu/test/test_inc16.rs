use crate::cpu::test::run_test;
use crate::cpu::flags::Flags;
use crate::cpu::Step;

use quickcheck::quickcheck;
use crate::cpu::registers::Reg16;


fn test_inc16(opcode: u8, x: u16, reg: Reg16) -> bool {
    let machine = run_test(&[opcode], |machine| {
        machine.cpu.registers.write16(reg, x);
    });
    let expected = x.wrapping_add(1);
    machine.t_cycles == 8 && machine.cpu.registers.read16(reg) == expected
}

#[test]
fn test_03() {
    fn prop(x: u16) -> bool {
        test_inc16(0x03, x, Reg16::BC)
    }
    quickcheck(prop as fn(u16) -> bool);
}

#[test]
fn test_03_overflow() {
    assert!(test_inc16(0x03, 0xffff, Reg16::BC))
}

#[test]
fn test_13() {
    fn prop(x: u16) -> bool {
        test_inc16(0x13, x, Reg16::DE)
    }
    quickcheck(prop as fn(u16) -> bool);
}

#[test]
fn test_13_overflow() {
    assert!(test_inc16(0x13, 0xffff, Reg16::DE))
}

#[test]
fn test_23() {
    fn prop(x: u16) -> bool {
        test_inc16(0x23, x, Reg16::HL)
    }
    quickcheck(prop as fn(u16) -> bool);
}

#[test]
fn test_23_overflow() {
    assert!(test_inc16(0x23, 0xffff, Reg16::HL))
}

#[test]
fn test_33() {
    fn prop(x: u16) -> bool {
        test_inc16(0x33, x, Reg16::SP)
    }
    quickcheck(prop as fn(u16) -> bool);
}

#[test]
fn test_33_overflow() {
    assert!(test_inc16(0x33, 0xffff, Reg16::SP))
}
