use crate::cpu::test::run_test;

#[test]
fn test_c3() {
    let machine = run_test(
        &[0xc3, 0x04, 0x00, 0xed], // JP nn
        |_| {},
    );
    assert_eq!(machine.t_cycles, 16);
    assert_eq!(machine.cpu.registers.pc, 0x05);
}
