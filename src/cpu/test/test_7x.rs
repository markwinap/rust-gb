use crate::cpu::test::run_test;
use crate::cpu::flags::Flags;
use crate::cpu::Step;


#[test]
fn test_70() {
    let mut machine = run_test(
        &[0x70, 0xed, 0x00], // LD (HL), B
        |machine| {
            machine.cpu.registers.b = 0x42;
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x42);
}

#[test]
fn test_71() {
    let mut machine = run_test(
        &[0x71, 0xed, 0x00], // LD (HL), C
        |machine| {
            machine.cpu.registers.c = 0x42;
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x42);
}

#[test]
fn test_72() {
    let mut machine = run_test(
        &[0x72, 0xed, 0x00], // LD (HL), D
        |machine| {
            machine.cpu.registers.d = 0x42;
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x42);
}

#[test]
fn test_73() {
    let mut machine = run_test(
        &[0x73, 0xed, 0x00], // LD (HL), E
        |machine| {
            machine.cpu.registers.e = 0x42;
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x42);
}

#[test]
fn test_74() {
    let mut machine = run_test(
        &[0x74, 0xed, 0x42], // LD (HL), H
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x00);
}

#[test]
fn test_75() {
    let mut machine = run_test(
        &[0x75, 0xed, 0x00], // LD (HL), L
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x02);
}

#[test]
fn test_76() {
    let machine = run_test(
        &[0x76], // HALT
        |machine| {
            machine.cpu.interface.interrupt_handler.interrupt_master_enabled = true;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    // assert_eq!(machine.t_cycles, 8); TODO verify, original version
    assert_eq!(machine.cpu.state, Step::Halt);
}

#[test]
fn test_77() {
    let mut machine = run_test(
        &[0x77, 0xed, 0x00], // LD (HL), A
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x42);
}

#[test]
fn test_78() {
    let machine = run_test(
        &[0x78], // LD A, B
        |machine| {
            machine.cpu.registers.b = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x42);
}

#[test]
fn test_79() {
    let machine = run_test(
        &[0x79], // LD A, C
        |machine| {
            machine.cpu.registers.c = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x42);
}

#[test]
fn test_7a() {
    let machine = run_test(
        &[0x7a], // LD A, D
        |machine| {
            machine.cpu.registers.d = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x42);
}

#[test]
fn test_7b() {
    let machine = run_test(
        &[0x7b], // LD A, E
        |machine| {
            machine.cpu.registers.e = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x42);
}

#[test]
fn test_7c() {
    let machine = run_test(
        &[0x7c], // LD A, H
        |machine| {
            machine.cpu.registers.h = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x42);
}

#[test]
fn test_7d() {
    let machine = run_test(
        &[0x7d], // LD A, L
        |machine| {
            machine.cpu.registers.l = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x42);
}

#[test]
fn test_7e() {
    let machine = run_test(
        &[0x7e, 0xed, 0x42], // LD A, (HL)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.a, 0x42);
}

#[test]
fn test_7f() {
    let machine = run_test(
        &[0x7f], // LD A, A
        |machine| {
            machine.cpu.registers.a = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x42);
}
