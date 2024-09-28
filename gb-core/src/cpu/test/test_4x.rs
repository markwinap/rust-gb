use crate::cpu::flags::Flags;
use crate::cpu::test::run_test;

#[test]
fn test_40() {
    let machine = run_test(
        &[0x40], // LD B, B
        |machine| {
            machine.cpu.registers.b = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.b, 0x42);
}

#[test]
fn test_41() {
    let machine = run_test(
        &[0x41], // LD B, C
        |machine| {
            machine.cpu.registers.c = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.b, 0x42);
}

#[test]
fn test_42() {
    let machine = run_test(
        &[0x42], // LD B, D
        |machine| {
            machine.cpu.registers.d = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.b, 0x42);
}

#[test]
fn test_43() {
    let machine = run_test(
        &[0x43], // LD B, E
        |machine| {
            machine.cpu.registers.e = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.b, 0x42);
}

#[test]
fn test_44() {
    let machine = run_test(
        &[0x44], // LD B, H
        |machine| {
            machine.cpu.registers.h = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.b, 0x42);
}

#[test]
fn test_45() {
    let machine = run_test(
        &[0x45], // LD B, L
        |machine| {
            machine.cpu.registers.l = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.b, 0x42);
}

#[test]
fn test_46() {
    let machine = run_test(
        &[0x46, 0xed, 0x42], // LD B, (HL)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.b, 0x42);
}

#[test]
fn test_47() {
    let machine = run_test(
        &[0x47], // LD B, A
        |machine| {
            machine.cpu.registers.a = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.b, 0x42);
}

#[test]
fn test_48() {
    let machine = run_test(
        &[0x48], // LD C, B
        |machine| {
            machine.cpu.registers.b = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.c, 0x42);
}

#[test]
fn test_49() {
    let machine = run_test(
        &[0x49], // LD C, C
        |machine| {
            machine.cpu.registers.c = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.c, 0x42);
}

#[test]
fn test_4a() {
    let machine = run_test(
        &[0x4a], // LD C, D
        |machine| {
            machine.cpu.registers.d = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.c, 0x42);
}

#[test]
fn test_4b() {
    let machine = run_test(
        &[0x4b], // LD C, E
        |machine| {
            machine.cpu.registers.e = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.c, 0x42);
}

#[test]
fn test_4c() {
    let machine = run_test(
        &[0x4c], // LD C, H
        |machine| {
            machine.cpu.registers.h = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.c, 0x42);
}

#[test]
fn test_4d() {
    let machine = run_test(
        &[0x4d], // LD C, L
        |machine| {
            machine.cpu.registers.l = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.c, 0x42);
}

#[test]
fn test_4e() {
    let machine = run_test(
        &[0x4e, 0xed, 0x42], // LD C, (HL)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.c, 0x42);
}

#[test]
fn test_4f() {
    let machine = run_test(
        &[0x4f], // LD C, A
        |machine| {
            machine.cpu.registers.a = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.c, 0x42);
}
