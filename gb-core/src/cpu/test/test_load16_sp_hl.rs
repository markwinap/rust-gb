
use crate::cpu::test::run_test;
use crate::cpu::flags::Flags;
use crate::cpu::Step;

use quickcheck::quickcheck;
use crate::cpu::registers::Reg16;


#[test]
fn test_f9() {
    fn prop(hl: u16) -> bool {
        let machine = run_test(&[0xf9], |machine| {
            machine.cpu.registers.write16(Reg16::HL, hl);
        });
        machine.t_cycles == 8 && machine.cpu.registers.read16(Reg16::SP) == hl
    }
    quickcheck(prop as fn(u16) -> bool);
}
