use crate::cpu::test::run_test;
use crate::cpu::flags::Flags;
use crate::cpu::Step;

use quickcheck::quickcheck;
use crate::cpu::registers::Reg16;


#[test]
fn test_f0() {
    let machine = run_test(
        &[0xf0, 0x80], // LDH A, (n)
        |machine| {
            machine.cpu.interface.memory[0xff80] = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.registers.a, 0x42);
}

#[test]
fn test_f2() {
    let machine = run_test(
        &[0xf2], // LDH A, (C)
        |machine| {
            machine.cpu.interface.memory[0xff80] = 0x42;
            machine.cpu.registers.c = 0x80;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.a, 0x42);
}

#[test]
fn test_f3() {
    let machine = run_test(
        &[0xf3], // DI
        |machine| {
            machine.cpu.interface.interrupt_handler.interrupt_master_enabled = true;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.interface.interrupt_handler.interrupt_master_enabled, false);
}

#[test]
fn test_fb() {
    let machine = run_test(
        &[0xfb, 0x00, 0x00], // EI
        |machine| {
            machine.cpu.interface.interrupt_handler.interrupt_master_enabled = false;
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.interface.interrupt_handler.interrupt_master_enabled, true);
}
