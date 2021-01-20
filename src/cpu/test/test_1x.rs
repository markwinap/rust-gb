use crate::cpu::test::run_test;
use crate::cpu::flags::Flags;



#[test]
fn test_12() {
    let mut machine = run_test(
        &[0x12, 0xed, 0x00], // LD (DE), A
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.d = 0x00;
            machine.cpu.registers.e = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x42);
}

#[test]
fn test_14() {
    let machine = run_test(
        &[0x14], // INC D
        |machine| {
            machine.cpu.registers.d = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.d, 0x43);
    assert_eq!(machine.cpu.registers.flags, Flags::default());
}

#[test]
fn test_14_zero() {
    let machine = run_test(
        &[0x14], // INC D
        |machine| {
            machine.cpu.registers.d = 0xff;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.d, 0x00);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: true,
        n: false,
        h: true,
        c: false
    });
}

#[test]
fn test_14_half_carry() {
    let machine = run_test(
        &[0x14], // INC D
        |machine| {
            machine.cpu.registers.d = 0x0f;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.d, 0x10);
    assert_eq!(machine.cpu.registers.flags,  Flags {
        z: false,
        n: false,
        h: true,
        c: false
    });
}

#[test]
fn test_15() {
    let machine = run_test(
        &[0x15], // DEC D
        |machine| {
            machine.cpu.registers.d = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.d, 0x41);
    assert_eq!(machine.cpu.registers.flags,  Flags {
        z: false,
        n: true,
        h: false,
        c: false
    });
}

#[test]
fn test_15_zero() {
    let machine = run_test(
        &[0x15], // DEC D
        |machine| {
            machine.cpu.registers.d = 0x01;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.d, 0x00);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: true,
        n: true,
        h: false,
        c: false
    });
}
// Flags {
//         z: false,
//         n: false,
//         h: false,
//         c: false
//     }
#[test]
fn test_15_half_carry() {
    let machine = run_test(
        &[0x15], // DEC D
        |machine| {
            machine.cpu.registers.d = 0x00;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.d, 0xff);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: false,
        n: true,
        h: true,
        c: false
    });
}

#[test]
fn test_16() {
    let machine = run_test(
        &[0x16, 0x42], // LD D, n
        |_| {},
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.d, 0x42);
}

#[test]
fn test_17() {
    let machine = run_test(
        &[0x17], // RLA
        |machine| {
            machine.cpu.registers.a = 0x55;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0xaa);
    assert_eq!(machine.cpu.registers.flags, Flags::default());
}

#[test]
fn test_17_carry() {
    let machine = run_test(
        &[0x17], // RLA
        |machine| {
            machine.cpu.registers.a = 0xaa;
            machine.cpu.registers.flags = Flags {
                z: false,
                n: false,
                h: false,
                c: true
            };
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x55);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: false,
        n: false,
        h: false,
        c: true
    });
}

#[test]
fn test_1a() {
    let machine = run_test(
        &[0x1a, 0xed, 0x42], // LD A, (DE)
        |machine| {
            machine.cpu.registers.d = 0x00;
            machine.cpu.registers.e = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.a, 0x42);
}

#[test]
fn test_1c() {
    let machine = run_test(
        &[0x1c], // INC E
        |machine| {
            machine.cpu.registers.e = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.e, 0x43);
    assert_eq!(machine.cpu.registers.flags, Flags::default());
}

#[test]
fn test_1c_zero() {
    let machine = run_test(
        &[0x1c], // INC E
        |machine| {
            machine.cpu.registers.e = 0xff;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.e, 0x00);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: true,
        n: false,
        h: true,
        c: false
    });
}

#[test]
fn test_1c_half_carry() {
    let machine = run_test(
        &[0x1c], // INC E
        |machine| {
            machine.cpu.registers.e = 0x0f;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.e, 0x10);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: false,
        n: false,
        h: true,
        c: false
    });
}

#[test]
fn test_1d() {
    let machine = run_test(
        &[0x1d], // DEC E
        |machine| {
            machine.cpu.registers.e = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.e, 0x41);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: false,
        n: true,
        h: false,
        c: false
    });
}

#[test]
fn test_1d_zero() {
    let machine = run_test(
        &[0x1d], // DEC E
        |machine| {
            machine.cpu.registers.e = 0x01;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.e, 0x00);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: true,
        n: true,
        h: false,
        c: false
    });
}

#[test]
fn test_1d_half_carry() {
    let machine = run_test(
        &[0x1d], // DEC E
        |machine| {
            machine.cpu.registers.e = 0x00;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.e, 0xff);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: false,
        n: true,
        h: true,
        c: false
    });
}

#[test]
fn test_1e() {
    let machine = run_test(
        &[0x1e, 0x42], // LD E, n
        |_| {},
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.e, 0x42);
}

#[test]
fn test_1f() {
    let machine = run_test(
        &[0x1f], // RRA
        |machine| {
            machine.cpu.registers.a = 0xaa;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x55);
    assert_eq!(machine.cpu.registers.flags, Flags::default());
}

#[test]
fn test_1f_carry() {
    let machine = run_test(
        &[0x1f], // RRA
        |machine| {
            machine.cpu.registers.a = 0x55;
            machine.cpu.registers.flags = Flags {
                z: false,
                n: false,
                h: false,
                c: true
            };
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0xaa);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: false,
        n: false,
        h: false,
        c: true
    });
}
