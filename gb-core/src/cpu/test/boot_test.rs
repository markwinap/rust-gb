use crate::cpu::flags::Flags;
use crate::cpu::test::run_test;

#[test]
fn test_20() {
    let machine = run_test(
        &[
            0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB,
        ], // JR NZ, e
        |_| {},
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.registers.pc, 0x04);
}
