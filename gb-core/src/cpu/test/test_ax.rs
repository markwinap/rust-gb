use crate::cpu::flags::Flags;
use crate::cpu::test::run_test;

#[test]
fn test_a8() {
    let machine = run_test(
        &[0xa8], // XOR B
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.b = 0x38;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x7a);
    assert_eq!(machine.cpu.registers.flags, Flags::empty());
}

#[test]
fn test_a8_zero() {
    let machine = run_test(
        &[0xa8], // XOR B
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.b = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x00);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: true,
            n: false,
            h: false,
            c: false,
        }
    );
}

#[test]
fn test_a9() {
    let machine = run_test(
        &[0xa9], // XOR C
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.c = 0x38;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x7a);
    assert_eq!(machine.cpu.registers.flags, Flags::empty());
}

#[test]
fn test_a9_zero() {
    let machine = run_test(
        &[0xa9], // XOR C
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.c = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x00);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: true,
            n: false,
            h: false,
            c: false,
        }
    );
}

#[test]
fn test_aa() {
    let machine = run_test(
        &[0xaa], // XOR D
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.d = 0x38;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x7a);
    assert_eq!(machine.cpu.registers.flags, Flags::empty());
}

#[test]
fn test_aa_zero() {
    let machine = run_test(
        &[0xaa], // XOR D
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.d = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x00);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: true,
            n: false,
            h: false,
            c: false,
        }
    );
}

#[test]
fn test_ab() {
    let machine = run_test(
        &[0xab], // XOR E
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.e = 0x38;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x7a);
    assert_eq!(machine.cpu.registers.flags, Flags::empty());
}

#[test]
fn test_ab_zero() {
    let machine = run_test(
        &[0xab], // XOR E
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.e = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x00);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: true,
            n: false,
            h: false,
            c: false,
        }
    );
}

#[test]
fn test_ac() {
    let machine = run_test(
        &[0xac], // XOR H
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.h = 0x38;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x7a);
    assert_eq!(machine.cpu.registers.flags, Flags::empty());
}

#[test]
fn test_ac_zero() {
    let machine = run_test(
        &[0xac], // XOR H
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.h = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x00);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: true,
            n: false,
            h: false,
            c: false,
        }
    );
}

#[test]
fn test_ad() {
    let machine = run_test(
        &[0xad], // XOR L
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.l = 0x38;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x7a);
    assert_eq!(machine.cpu.registers.flags, Flags::empty());
}

#[test]
fn test_ad_zero() {
    let machine = run_test(
        &[0xad], // XOR L
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.l = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x00);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: true,
            n: false,
            h: false,
            c: false,
        }
    );
}

#[test]
fn test_ae() {
    let machine = run_test(
        &[0xae, 0xed, 0x38], // XOR (HL)
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.a, 0x7a);
    assert_eq!(machine.cpu.registers.flags, Flags::empty());
}

#[test]
fn test_ae_zero() {
    let machine = run_test(
        &[0xae, 0xed, 0x42], // XOR (HL)
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.a, 0x00);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: true,
            n: false,
            h: false,
            c: false,
        }
    );
}

#[test]
fn test_af() {
    let machine = run_test(
        &[0xaf], // XOR A
        |machine| {
            machine.cpu.registers.a = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x00);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: true,
            n: false,
            h: false,
            c: false,
        }
    );
}
