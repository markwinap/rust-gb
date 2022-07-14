use crate::cpu::test::run_test;
use crate::cpu::flags::Flags;
use crate::cpu::Step;

use quickcheck::quickcheck;
use crate::cpu::registers::Reg16;



#[test]
fn test_e0() {
    let machine = run_test(
        &[0xe0, 0x80], // LDH (n), A
        |machine| {
            machine.cpu.registers.a = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.interface.memory[0xff80], 0x42);
}

#[test]
fn test_e2() {
    let machine = run_test(
        &[0xe2], // LD (C), A
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.c = 0x80;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.interface.memory[0xff80], 0x42);
}

#[test]
fn test_ea() {
    let machine = run_test(
        &[0xea, 0x04, 0x00, 0xed, 0x00], // LD (nn), A
        |machine| {
            machine.cpu.registers.a = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 16);
    assert_eq!(machine.cpu.interface.memory[0x04], 0x42);
}

#[test]
fn test_ee() {
    let machine = run_test(
        &[0xee, 0x38], // XOR (n)
        |machine| {
            machine.cpu.registers.a = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.a, 0x7a);
    assert_eq!(machine.cpu.registers.flags, Flags::empty());
}

#[test]
fn test_ee_zero() {
    let machine = run_test(
        &[0xee, 0x42], // XOR (n)
        |machine| {
            machine.cpu.registers.a = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.a, 0x00);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: true,
        n: false,
        h: false,
        c: false
    });
}

