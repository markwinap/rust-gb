use crate::cpu::flags::Flags;
use crate::cpu::test::run_test;

#[test]
fn test_30() {
    let machine = run_test(
        &[0x30, 0x01, 0xed, 0xed], // JR NC, e
        |_| {},
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.registers.pc, 0x04);
}

#[test]
fn test_30_negative() {
    let machine = run_test(
        &[0x00, 0xed, 0x30, -3i8 as u8], // JR NC, e
        |machine| {
            machine.cpu.registers.pc = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.registers.pc, 0x02);
}

#[test]
fn test_30_nojump() {
    let machine = run_test(
        &[0x30, 0x01, 0xed, 0x00], // JR NC, e
        |machine| {
            machine.cpu.registers.flags = Flags {
                z: false,
                n: false,
                h: false,
                c: true,
            };
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.pc, 0x03);
}

#[test]
fn test_32() {
    let mut machine = run_test(
        &[0x32, 0xed, 0x00], // LDD (HL), A
        |machine| {
            machine.cpu.registers.a = 0x42;
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.h, 0x00);
    assert_eq!(machine.cpu.registers.l, 0x01);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x42);
}

#[test]
fn test_34() {
    let mut machine = run_test(
        &[0x34, 0xed, 0x42], // INC (HL)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x43);
    assert_eq!(machine.cpu.registers.flags, Flags::default());
}

#[test]
fn test_34_zero() {
    let mut machine = run_test(
        &[0x34, 0xed, 0xff], // INC (HL)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x00);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: true,
            n: false,
            h: true,
            c: false,
        }
    );
}

#[test]
fn test_34_half_carry() {
    let mut machine = run_test(
        &[0x34, 0xed, 0x0f], // INC (HL)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x10);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: false,
            n: false,
            h: true,
            c: false,
        }
    );
}

#[test]
fn test_35() {
    let mut machine = run_test(
        &[0x35, 0xed, 0x42], // DEC (HL)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x41);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: false,
            n: true,
            h: false,
            c: false,
        }
    );
}

#[test]
fn test_35_zero() {
    let mut machine = run_test(
        &[0x35, 0xed, 0x01], // DEC (HL)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0x00);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: true,
            n: true,
            h: false,
            c: false,
        }
    );
}

#[test]
fn test_35_half_carry() {
    let mut machine = run_test(
        &[0x35, 0xed, 0x00], // DEC (HL)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.get_interface().memory[0x02], 0xff);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: false,
            n: true,
            h: true,
            c: false,
        }
    );
}

#[test]
fn test_36() {
    let mut machine = run_test(
        &[0x36, 0x42, 0xed, 0x00], // LD (HL), n
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x03;
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.get_interface().memory[0x03], 0x42);
}

#[test]
fn test_37() {
    let machine = run_test(
        &[0x37], // SCF
        |machine| {
            machine.cpu.registers.flags = Flags {
                z: false,
                n: true,
                h: true,
                c: false,
            };
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: false,
            n: false,
            h: false,
            c: true,
        }
    );
}

#[test]
fn test_38() {
    let machine = run_test(
        &[0x38, 0x01, 0xed, 0xed], // JR C, e
        |machine| {
            machine.cpu.registers.flags = Flags {
                z: false,
                n: false,
                h: false,
                c: true,
            };
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.registers.pc, 0x04);
}

#[test]
fn test_38_negative() {
    let machine = run_test(
        &[0x00, 0xed, 0x38, -3i8 as u8], // JR C, e
        |machine| {
            machine.cpu.registers.flags = Flags {
                z: false,
                n: false,
                h: false,
                c: true,
            };
            machine.cpu.registers.pc = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 12);
    assert_eq!(machine.cpu.registers.pc, 0x02);
}

#[test]
fn test_38_nojump() {
    let machine = run_test(
        &[0x38, 0x01, 0xed, 0x00], // JR C, e
        |_| {},
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.pc, 0x03);
}

#[test]
fn test_3a() {
    let machine = run_test(
        &[0x3a, 0xed, 0x42], // LD A, (HL-)
        |machine| {
            machine.cpu.registers.h = 0x00;
            machine.cpu.registers.l = 0x02;
        },
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.a, 0x42);
    assert_eq!(machine.cpu.registers.h, 0x00);
    assert_eq!(machine.cpu.registers.l, 0x01);
}

#[test]
fn test_3c() {
    let machine = run_test(
        &[0x3c], // INC A
        |machine| {
            machine.cpu.registers.a = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x43);
    assert_eq!(machine.cpu.registers.flags, Flags::default());
}

#[test]
fn test_3c_zero() {
    let machine = run_test(
        &[0x3c], // INC A
        |machine| {
            machine.cpu.registers.a = 0xff;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x00);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: true,
            n: false,
            h: true,
            c: false,
        }
    );
}

#[test]
fn test_3c_half_carry() {
    let machine = run_test(
        &[0x3c], // INC A
        |machine| {
            machine.cpu.registers.a = 0x0f;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x10);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: false,
            n: false,
            h: true,
            c: false,
        }
    );
}

#[test]
fn test_3d() {
    let machine = run_test(
        &[0x3d], // DEC A
        |machine| {
            machine.cpu.registers.a = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x41);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: false,
            n: true,
            h: false,
            c: false,
        }
    );
}

#[test]
fn test_3d_zero() {
    let machine = run_test(
        &[0x3d], // DEC A
        |machine| {
            machine.cpu.registers.a = 0x01;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0x00);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: true,
            n: true,
            h: false,
            c: false,
        }
    );
}

#[test]
fn test_3d_half_carry() {
    let machine = run_test(
        &[0x3d], // DEC A
        |machine| {
            machine.cpu.registers.a = 0x00;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0xff);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: false,
            n: true,
            h: true,
            c: false,
        }
    );
}

#[test]
fn test_3e() {
    let machine = run_test(
        &[0x3e, 0x42], // LD A, n
        |_| {},
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.a, 0x42);
}

#[test]
fn test_3f() {
    let machine = run_test(
        &[0x3f], // CCF
        |machine| {
            machine.cpu.registers.flags = Flags {
                z: false,
                n: true,
                h: true,
                c: false,
            };
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(
        machine.cpu.registers.flags,
        Flags {
            z: false,
            n: false,
            h: false,
            c: true,
        }
    );
}

#[test]
fn test_3f_carry() {
    let machine = run_test(
        &[0x3f], // CCF
        |machine| {
            machine.cpu.registers.flags = Flags {
                z: false,
                n: true,
                h: true,
                c: true,
            };
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.flags, Flags::default());
}
