use crate::cpu::flags::Flags;
use crate::cpu::test::run_test;
use crate::cpu::Step;

use crate::cpu::registers::Reg16;
use quickcheck::quickcheck;

fn test_add16<F: Fn(Flags) -> bool>(
    opcode: u8,
    hl: u16,
    reg: Reg16,
    x: u16,
    check_flags: F,
) -> bool {
    let mut machine = run_test(&[opcode], |machine| {
        machine.cpu.registers.write16(Reg16::HL, hl);
        machine.cpu.registers.write16(reg, x);
    });
    let expected = hl.wrapping_add(x);
    machine.t_cycles == 8
        && machine.cpu.registers.read16(Reg16::HL) == expected
        && check_flags(machine.cpu.registers.flags)
}

#[test]
fn test_09() {
    assert!(test_add16(0x09, 0x0003, Reg16::BC, 0x0ffc, |f| f == Flags::default()));
}

#[test]
fn test_09_carry() {
    assert!(test_add16(0x09, 0x5002, Reg16::BC, 0xb7fd, |f| f
        == Flags {
            z: false,
            n: false,
            h: false,
            c: true,
        }));
}

#[test]
fn test_09_gb_manual() {
    assert!(test_add16(0x09, 0x8a23, Reg16::BC, 0x0605, |f| f
        == Flags {
            z: false,
            n: false,
            h: true,
            c: false,
        }));
}

#[test]
fn test_19() {
    assert!(test_add16(0x19, 0x0003, Reg16::DE, 0x0ffc, |f| f == Flags::default()));
}

#[test]
fn test_19_half_carry() {
    assert!(test_add16(0x19, 0x8a23, Reg16::DE, 0x0605, |f| f
        == Flags {
            z: false,
            n: false,
            h: true,
            c: false,
        }));
}

#[test]
fn test_19_carry() {
    assert!(test_add16(0x19, 0x5002, Reg16::DE, 0xb7fd, |f| f
        == Flags {
            z: false,
            n: false,
            h: false,
            c: true,
        }));
}

#[test]
fn test_29() {
    assert!(test_add16(0x29, 0x02aa, Reg16::HL, 0x02aa, |f| f == Flags::default()));
}

#[test]
fn test_29_half_carry() {
    assert!(test_add16(0x29, 0x0fff, Reg16::HL, 0x0fff, |f| f
        == Flags {
            z: false,
            n: false,
            h: true,
            c: false,
        }));
}

#[test]
fn test_29_carry() {
    assert!(test_add16(0x29, 0x8001, Reg16::HL, 0x8001, |f| f
        == Flags {
            z: false,
            n: false,
            h: false,
            c: true,
        }));
}

#[test]
fn test_29_gb_manual() {
    assert!(test_add16(0x29, 0x8a23, Reg16::HL, 0x8a23, |f| f
        == Flags {
            z: false,
            n: false,
            h: true,
            c: true,
        }));
}

#[test]
fn test_39() {
    assert!(test_add16(0x39, 0x0003, Reg16::SP, 0x0ffc, |f| f == Flags::default()));
}

#[test]
fn test_39_half_carry() {
    assert!(test_add16(0x39, 0x8a23, Reg16::SP, 0x0605, |f| f
        == Flags {
            z: false,
            n: false,
            h: true,
            c: false,
        }));
}

#[test]
fn test_39_carry() {
    assert!(test_add16(0x39, 0x5002, Reg16::SP, 0xb7fd, |f| f
        == Flags {
            z: false,
            n: false,
            h: false,
            c: true,
        }));
}
