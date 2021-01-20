use crate::cpu::test::run_test;
use crate::cpu::flags::Flags;


#[test]
fn test_50() {
    let machine = run_test(
        &[0x50], // LD D, B
        |machine| {
            machine.cpu.registers.b = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.d, 0x42);
}

#[test]
fn test_51() {
    let machine = run_test(
        &[0x51], // LD D, C
        |machine| {
            machine.cpu.registers.c = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.d, 0x42);
}

#[test]
fn test_52() {
    let machine = run_test(
        &[0x52], // LD D, D
        |machine| {
            machine.cpu.registers.d = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.d, 0x42);
}

#[test]
fn test_53() {
    let machine = run_test(
        &[0x53], // LD D, E
        |machine| {
            machine.cpu.registers.e = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.d, 0x42);
}

#[test]
fn test_54() {
    let machine = run_test(
        &[0x54], // LD D, H
        |machine| {
            machine.cpu.registers.h = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.d, 0x42);
}

#[test]
fn test_55() {
    let machine = run_test(
        &[0x55], // LD D, L
        |machine| {
            machine.cpu.registers.l = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.d, 0x42);
}

#[test]
fn test_56() {
    let machine = run_test(
        &[0x56, 0xed, 0x42], // LD D, (HL)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.d, 0x42);
}

#[test]
fn test_57() {
    let machine = run_test(
        &[0x57], // LD D, A
        |machine| {
            machine.cpu.registers.a = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.d, 0x42);
}

#[test]
fn test_58() {
    let machine = run_test(
        &[0x58], // LD E, B
        |machine| {
            machine.cpu.registers.b = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.e, 0x42);
}

#[test]
fn test_59() {
    let machine = run_test(
        &[0x59], // LD E, C
        |machine| {
            machine.cpu.registers.c = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.e, 0x42);
}

#[test]
fn test_5a() {
    let machine = run_test(
        &[0x5a], // LD E, D
        |machine| {
            machine.cpu.registers.d = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.e, 0x42);
}

#[test]
fn test_5b() {
    let machine = run_test(
        &[0x5b], // LD E, E
        |machine| {
            machine.cpu.registers.e = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.e, 0x42);
}

#[test]
fn test_5c() {
    let machine = run_test(
        &[0x5c], // LD E, H
        |machine| {
            machine.cpu.registers.h = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.e, 0x42);
}

#[test]
fn test_5d() {
    let machine = run_test(
        &[0x5d], // LD E, L
        |machine| {
            machine.cpu.registers.l = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.e, 0x42);
}

#[test]
fn test_5e() {
    let machine = run_test(
        &[0x5e, 0xed, 0x42], // LD E, (HL)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.e, 0x42);
}

#[test]
fn test_5f() {
    let machine = run_test(
        &[0x5f], // LD E, A
        |machine| {
            machine.cpu.registers.a = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.e, 0x42);
}
