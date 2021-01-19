use crate::cpu::test::run_test;

#[test]
fn test_00() {
    let machine = run_test(
        &[0x00], // NOP
   |_| {} );
    assert_eq!(machine.t_cycles, 4);
}


#[test]
fn test_02() {
    let  mut machine = run_test(
        &[0x02, 0xed, 0x00], // LD (BC), A
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.b = 0x00;
            machine.cpu.registers.c = 0x02;
        },
    );

    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x42);
}
