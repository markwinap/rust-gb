
use crate::cpu::test::run_test;
use crate::cpu::flags::Flags;
use crate::cpu::Step;

use quickcheck::quickcheck;
use crate::cpu::registers::Reg16;



fn test_load16_hl_sp_e<F: Fn(Flags) -> bool>(sp: u16, e: i8, check_flags: F) -> bool {
    let machine = run_test(&[0xf8, e as u8], |machine| {
        machine.cpu.registers.write16(Reg16::SP, sp);
    });
    let expected = sp.wrapping_add(e as i16 as u16);
    machine.t_cycles == 12
        && machine.cpu.registers.read16(Reg16::HL) == expected
        && check_flags(machine.cpu.registers.flags)
}

#[test]
fn test_f8() {
    fn prop(sp: u16, e: i8) -> bool {
        test_load16_hl_sp_e(sp, e, |_| true)
    }
    quickcheck(prop as fn(u16, i8) -> bool);
}

#[test]
fn test_f8_overflow_inc() {
    assert!(test_load16_hl_sp_e(0xffff, 1, |f| f == Flags {
        z: false,
        n: false,
        h: true,
        c: true
    }));
}

#[test]
fn test_f8_overflow_dec() {
    assert!(test_load16_hl_sp_e(0x0000, -1, |f| f == Flags::empty()));
}
