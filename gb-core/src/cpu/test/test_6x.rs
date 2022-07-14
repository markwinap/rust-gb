use crate::cpu::test::run_test;
use crate::cpu::flags::Flags;


#[test]
fn test_60() {
    let machine = run_test(
        &[0x60], // LD H, B
        |machine| {
            machine.cpu.registers.b = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.h, 0x42);
}

#[test]
fn test_61() {
    let machine = run_test(
        &[0x61], // LD H, C
        |machine| {
            machine.cpu.registers.c = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.h, 0x42);
}

#[test]
fn test_62() {
    let machine = run_test(
        &[0x62], // LD H, D
        |machine| {
            machine.cpu.registers.d = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.h, 0x42);
}

#[test]
fn test_63() {
    let machine = run_test(
        &[0x63], // LD H, E
        |machine| {
            machine.cpu.registers.e = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.h, 0x42);
}

#[test]
fn test_64() {
    let machine = run_test(
        &[0x64], // LD H, H
        |machine| {
            machine.cpu.registers.h = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.h, 0x42);
}

#[test]
fn test_65() {
    let machine = run_test(
        &[0x65], // LD H, L
        |machine| {
            machine.cpu.registers.l = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.h, 0x42);
}

#[test]
fn test_66() {
    let machine = run_test(
        &[0x66, 0xed, 0x42], // LD H, (HL)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.h, 0x42);
}

#[test]
fn test_67() {
    let machine = run_test(
        &[0x67], // LD H, A
        |machine| {
            machine.cpu.registers.a = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.h, 0x42);
}

#[test]
fn test_68() {
    let machine = run_test(
        &[0x68], // LD L, B
        |machine| {
            machine.cpu.registers.b = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.l, 0x42);
}

#[test]
fn test_69() {
    let machine = run_test(
        &[0x69], // LD L, C
        |machine| {
            machine.cpu.registers.c = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.l, 0x42);
}

#[test]
fn test_6a() {
    let machine = run_test(
        &[0x6a], // LD L, D
        |machine| {
            machine.cpu.registers.d = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.l, 0x42);
}

#[test]
fn test_6b() {
    let machine = run_test(
        &[0x6b], // LD L, E
        |machine| {
            machine.cpu.registers.e = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.l, 0x42);
}

#[test]
fn test_6c() {
    let machine = run_test(
        &[0x6c], // LD L, H
        |machine| {
            machine.cpu.registers.h = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.l, 0x42);
}

#[test]
fn test_6d() {
    let machine = run_test(
        &[0x6d], // LD L, L
        |machine| {
            machine.cpu.registers.l = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.l, 0x42);
}

#[test]
fn test_6e() {
    let machine = run_test(
        &[0x6e, 0xed, 0x42], // LD L, (HL)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.l, 0x42);
}

#[test]
fn test_6f() {
    let machine = run_test(
        &[0x6f], // LD L, A
        |machine| {
            machine.cpu.registers.a = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.l, 0x42);
}
