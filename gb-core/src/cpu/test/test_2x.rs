use crate::cpu::test::run_test;
use crate::cpu::flags::Flags;


#[test]
fn test_20() {
    let machine = run_test(
        &[0x20, 0x01, 0xed, 0xed], // JR NZ, e
        |_| {},
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.registers.pc, 0x04);
}

#[test]
fn test_20_negative() {
    let machine = run_test(
        &[0x00, 0xed, 0x20, -3i8 as u8], // JR NZ, e
        |machine| {
            machine.cpu.registers.pc = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.registers.pc, 0x02);
}

#[test]
fn test_20_nojump() {
    let machine = run_test(
        &[0x20, 0x01, 0xed, 0x00], // JR NZ, e
        |machine| {
            machine.cpu.registers.flags = Flags {
                z: true,
                n: false,
                h: false,
                c: false
            };
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.pc, 0x03);
}

#[test]
fn test_22() {
    let mut machine = run_test(
        &[0x22, 0xed, 0x00], // LDI (HL), A
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.h, 0x00);
    assert_eq!(machine.cpu.registers.l, 0x03);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x42);
}

#[test]
fn test_24() {
    let machine = run_test(
        &[0x24], // INC H
        |machine| {
            machine.cpu.registers.h = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.h, 0x43);
    assert_eq!(machine.cpu.registers.flags, Flags::default());
}

#[test]
fn test_24_zero() {
    let machine = run_test(
        &[0x24], // INC H
        |machine| {
            machine.cpu.registers.h = 0xff;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.h, 0x00);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: true,
        n: false,
        h: true,
        c: false
    });
}

#[test]
fn test_24_half_carry() {
    let machine = run_test(
        &[0x24], // INC H
        |machine| {
            machine.cpu.registers.h = 0x0f;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.h, 0x10);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: false,
        n: false,
        h: true,
        c: false
    });
}

#[test]
fn test_25() {
    let machine = run_test(
        &[0x25], // DEC H
        |machine| {
            machine.cpu.registers.h = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.h, 0x41);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: false,
        n: true,
        h: false,
        c: false
    });
}

#[test]
fn test_25_zero() {
    let machine = run_test(
        &[0x25], // DEC H
        |machine| {
            machine.cpu.registers.h = 0x01;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.h, 0x00);
    assert_eq!(machine.cpu.registers.flags,Flags {
        z: true,
        n: true,
        h: false,
        c: false
    });
}

#[test]
fn test_25_half_carry() {
    let machine = run_test(
        &[0x25], // DEC H
        |machine| {
            machine.cpu.registers.h = 0x00;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.h, 0xff);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: false,
        n: true,
        h: true,
        c: false
    });
}

#[test]
fn test_26() {
    let machine = run_test(
        &[0x26, 0x42], // LD H, n
        |_| {},
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.h, 0x42);
}

#[test]
fn test_28() {
    let machine = run_test(
        &[0x28, 0x01, 0xed, 0xed], // JR Z, e
        |machine| {
            machine.cpu.registers.flags = Flags {
                z: true,
                n: false,
                h: false,
                c: false
            };
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.registers.pc, 0x04);
}

#[test]
fn test_28_negative() {
    let machine = run_test(
        &[0x00, 0xed, 0x28, -3i8 as u8], // JR Z, e
        |machine| {
            machine.cpu.registers.flags = Flags {
                z: true,
                n: false,
                h: false,
                c: false
            };
            machine.cpu.registers.pc = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.registers.pc, 0x02);
}

#[test]
fn test_28_nojump() {
    let machine = run_test(
        &[0x28, 0x01, 0xed, 0x00], // JR Z, e
        |_| {},
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.pc, 0x03);
}

#[test]
fn test_2a() {
    let machine = run_test(
        &[0x2a, 0xed, 0x42], // LD A, (HL+)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.a, 0x42);
    assert_eq!(machine.cpu.registers.h, 0x00);
    assert_eq!(machine.cpu.registers.l, 0x03);
}

#[test]
fn test_2c() {
    let machine = run_test(
        &[0x2c], // INC L
        |machine| {
            machine.cpu.registers.l = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.l, 0x43);
    assert_eq!(machine.cpu.registers.flags, Flags::default());
}

#[test]
fn test_2c_zero() {
    let machine = run_test(
        &[0x2c], // INC L
        |machine| {
            machine.cpu.registers.l = 0xff;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.l, 0x00);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: true,
        n: false,
        h: true,
        c: false
    });
}

#[test]
fn test_2c_half_carry() {
    let machine = run_test(
        &[0x2c], // INC L
        |machine| {
            machine.cpu.registers.l = 0x0f;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.l, 0x10);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: false,
        n: false,
        h: true,
        c: false
    });
}

#[test]
fn test_2d() {
    let machine = run_test(
        &[0x2d], // DEC L
        |machine| {
            machine.cpu.registers.l = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.l, 0x41);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: false,
        n: true,
        h: false,
        c: false
    });
}

#[test]
fn test_2d_zero() {
    let machine = run_test(
        &[0x2d], // DEC L
        |machine| {
            machine.cpu.registers.l = 0x01;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.l, 0x00);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: true,
        n: true,
        h: false,
        c: false
    });
}

#[test]
fn test_2d_half_carry() {
    let machine = run_test(
        &[0x2d], // DEC L
        |machine| {
            machine.cpu.registers.l = 0x00;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.l, 0xff);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: false,
        n: true,
        h: true,
        c: false
    });
}

#[test]
fn test_2e() {
    let machine = run_test(
        &[0x2e, 0x42], // LD L, n
        |_| {},
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.l, 0x42);
}

#[test]
fn test_2f() {
    let machine = run_test(
        &[0x2f], // CPL
        |machine| {
            machine.cpu.registers.a = 0xaa;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x55);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: false,
        n: true,
        h: true,
        c: false
    });
}
