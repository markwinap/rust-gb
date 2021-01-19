use crate::cpu::test::run_test;
use crate::cpu::flags::Flags;

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


#[test]
fn test_04() {
    let machine = run_test(
        &[0x04], // INC B
        |machine| {
            machine.cpu.registers.b = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.b, 0x43);
    assert_eq!(machine.cpu.registers.flags, Flags::default());
}

#[test]
fn test_04_zero() {
    let machine = run_test(
        &[0x04], // INC B
        |machine| {
            machine.cpu.registers.b = 0xff;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.b, 0x00);
    assert_eq!(machine.cpu.registers.flags, Flags {
        z: true,
        n: false,
        h: true,
        c: false
    });
}

#[test]
fn test_04_half_carry() {
    let machine = run_test(
        &[0x04], // INC B
        |machine| {
            machine.cpu.registers.b = 0x0f;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.b, 0x10);
    assert_eq!(machine.cpu.registers.flags,  Flags {
        z: false,
        n: false,
        h: true,
        c: false
    });
}

#[test]
fn test_05() {
    let machine = run_test(
        &[0x05], // DEC B
        |machine| {
            machine.cpu.registers.b = 0x42;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.b, 0x41);
    assert_eq!(machine.cpu.registers.flags,  Flags {
        z: false,
        n: true,
        h: false,
        c: false
    });
}

#[test]
fn test_05_half_carry() {
    let machine = run_test(
        &[0x05], // DEC B
        |machine| {
            machine.cpu.registers.b = 0x00;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.b, 0xff);
    assert_eq!(machine.cpu.registers.flags,  Flags {
        z: false,
        n: true,
        h: true,
        c: false
    });
}

#[test]
fn test_06() {
    let machine = run_test(
        &[0x06, 0x42], // LD B, n
        |_| {},
    );
    assert_eq!(machine.t_cycles, 8);
    assert_eq!(machine.cpu.registers.b, 0x42);
}

#[test]
fn test_07() {
    let machine = run_test(
        &[0x07], // RLCA
        |machine| {
            machine.cpu.registers.a = 0x77;
        },
    );
    assert_eq!(machine.t_cycles, 4);
    assert_eq!(machine.cpu.registers.a, 0xee);
    assert_eq!(machine.cpu.registers.flags, Flags::default());
}
