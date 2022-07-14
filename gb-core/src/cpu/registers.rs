use crate::cpu::flags::Flags;


#[derive(Clone, Copy, Debug)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

use core::fmt::Display;
#[derive(Clone, Copy, Debug)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP
}

#[derive(Default, Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub flags: Flags,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | self.flags.read_value() as u16
    }

    pub fn set_af(&mut self, v: u16) {
        self.a = get_msb(v);
        self.flags.set_value(get_lsb(v));
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    pub fn set_bc(&mut self, v: u16) {
        self.b = get_msb(v);
        self.c = get_lsb(v);
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    pub fn set_de(&mut self, v: u16) {
        self.d = get_msb(v);
        self.e = get_lsb(v);
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    pub fn set_hl(&mut self, v: u16) {
        self.h = get_msb(v);
        self.l = get_lsb(v);
    }

    pub fn increment_sp(&mut self) {
        self.sp = (self.sp + 1) & 0xffff;
    }

    pub fn decrement_sp(&mut self) {
        self.sp = (self.sp - 1) & 0xffff;
    }

    pub fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }


    pub fn get_a(&mut self) -> &mut u8 {
        &mut self.a
    }
    pub fn set_a(&mut self, v: u8) {
        self.a = v;
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }
    pub fn set_b(&mut self, v: u8) {
        self.b = v;
    }

    pub fn get_c(&self) -> u8 {
        self.c
    }
    pub fn set_c(&mut self, v: u8) {
        self.c = v;
    }

    pub fn get_d(&self) -> u8 {
        self.d
    }
    pub fn set_d(&mut self, v: u8) {
        self.d = v;
    }

    pub fn get_e(&self) -> u8 {
        self.e
    }
    pub fn set_e(&mut self, v: u8) {
        self.e = v;
    }

    pub fn get_h(&self) -> u8 {
        self.h
    }
    pub fn set_h(&mut self, v: u8) {
        self.h = v;
    }
    pub fn get_l(&self) -> u8 {
        self.l
    }
    pub fn set_l(&mut self, v: u8) {
        self.l = v;
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }
    pub fn set_sp(&mut self, v: u16) {
        self.sp = v;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }
    pub fn set_pc(&mut self, v: u16) {
        self.pc = v;
    }

    pub fn write16(&mut self, register: Reg16, value: u16) {
        match register {
            Reg16::AF => { self.set_af(value);}
            Reg16::BC => { self.set_bc(value);}
            Reg16::DE => { self.set_de(value);}
            Reg16::HL => { self.set_hl(value);}
            Reg16::SP => { self.set_sp(value);}
        }
    }

    pub fn read16(&self, register: Reg16) -> u16 {
        match register {
            Reg16::AF => { self.get_af() }
            Reg16::BC => { self.get_bc() }
            Reg16::DE => { self.get_de() }
            Reg16::HL => { self.get_hl() }
            Reg16::SP => { self.get_sp() }
        }
    }
}

#[inline(always)]
pub fn get_msb(v: u16) -> u8 {
    (v >> 8) as u8
}
#[inline(always)]
pub fn get_lsb(v: u16) -> u8 {
    v as u8
}

