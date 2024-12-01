use crate::cpu::registers::{Reg16, Reg8, Registers};
use crate::cpu::{Interface, Step};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cpu<T: Interface> {
    pub registers: Registers,
    //   pub op_code: u8,
    pub interface: T,
    //pub state: Step,
    // pub prev_opcode: PrevExec,
    pub tick_count: usize,
    pub current_screen_state: bool,
}

#[derive(Default)]
pub struct PrevExec {
    pub prev_opcode: u8,
    pub prev_pc: u16,
}

impl<T: Interface> Cpu<T> {
    pub fn get_interface(&mut self) -> &mut T {
        &mut self.interface
    }

    pub fn read_next_byte(&mut self) -> u8 {
        let addr = self.registers.get_pc();
        self.registers.increment_pc();
        let result = self.interface.get_byte(addr);
        // if result == u8::MAX {
        //     println!("WER: {}", result);
        // }
        result
    }

    pub fn read_next_word(&mut self) -> u16 {
        let lo = self.read_next_byte();
        let hi = self.read_next_byte();
        let word = u16::from_le_bytes([lo, hi]);
        //  println!("Word read: {:X?}", word);
        word
    }

    pub fn push_u16(&mut self, value: u16) {
        let [lo, hi] = u16::to_le_bytes(value);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.interface.set_byte(self.registers.sp, hi);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.interface.set_byte(self.registers.sp, lo);
    }

    pub fn pop_u16(&mut self) -> u16 {
        let lo = self.interface.get_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let hi = self.interface.get_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        u16::from_le_bytes([lo, hi])
    }
}

impl<T: Interface> Read8<Reg8> for Cpu<T> {
    #[inline(always)]
    fn read_8(&mut self, src: Reg8) -> u8 {
        match src {
            Reg8::A => self.registers.a,
            Reg8::B => self.registers.b,
            Reg8::C => self.registers.c,
            Reg8::D => self.registers.d,
            Reg8::E => self.registers.e,
            Reg8::H => self.registers.h,
            Reg8::L => self.registers.l,
        }
    }
}

impl<T: Interface> Write8<Reg8> for Cpu<T> {
    #[inline(always)]
    fn write_8(&mut self, dst: Reg8, val: u8) {
        match dst {
            Reg8::A => self.registers.set_a(val),
            Reg8::B => self.registers.set_b(val),
            Reg8::C => self.registers.set_c(val),
            Reg8::D => self.registers.set_d(val),
            Reg8::E => self.registers.set_e(val),
            Reg8::H => self.registers.set_h(val),
            Reg8::L => self.registers.set_l(val),
        }
    }
}

impl<T: Interface> Read16<Reg16> for Cpu<T> {
    #[inline(always)]
    fn read_16(&mut self, src: Reg16) -> u16 {
        match src {
            Reg16::AF => self.registers.get_af(),
            Reg16::BC => self.registers.get_bc(),
            Reg16::DE => self.registers.get_de(),
            Reg16::HL => self.registers.get_hl(),
            Reg16::SP => self.registers.sp,
        }
    }
}

impl<T: Interface> Write16<Reg16> for Cpu<T> {
    #[inline(always)]
    fn write_16(&mut self, dst: Reg16, val: u16) {
        //  println!("Inside write reg: {} with value: {}", dst, val);
        match dst {
            Reg16::AF => self.registers.set_af(val),
            Reg16::BC => self.registers.set_bc(val),
            Reg16::DE => self.registers.set_de(val),
            Reg16::HL => self.registers.set_hl(val),
            Reg16::SP => self.registers.sp = val,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Immediate8;

#[derive(Clone, Copy)]
pub struct Immediate16;

impl<T: Interface> Read8<Immediate8> for Cpu<T> {
    fn read_8(&mut self, _: Immediate8) -> u8 {
        let result = self.read_next_byte();
        // if result == u8::MAX {
        //     println!("WEIRD: {}", result);
        // }
        result
    }
}

impl<T: Interface> Read16<Immediate16> for Cpu<T> {
    fn read_16(&mut self, _: Immediate16) -> u16 {
        self.read_next_word()
    }
}

pub trait Read8<T: Copy> {
    fn read_8(&mut self, src: T) -> u8;
}

pub trait Write8<T: Copy> {
    fn write_8(&mut self, dst: T, val: u8);
}

pub trait Read16<T: Copy> {
    fn read_16(&mut self, src: T) -> u16;
}

pub trait Write16<T: Copy> {
    fn write_16(&mut self, dst: T, val: u16);
}

#[derive(Clone, Copy)]
pub enum Addr {
    BC,
    DE,
    HL,
    HLD,
    HLI,
    Direct,
    ReadOffset(ReadOffType),
}

#[derive(Clone, Copy)]
pub enum Addr16 {
    Direct,
}

#[derive(Clone, Copy)]
pub enum ReadOffType {
    Register(Reg8),
    Immediate8,
}

impl<T: Interface> Read16<Addr16> for Cpu<T> {
    #[inline(always)]
    fn read_16(&mut self, src: Addr16) -> u16 {
        match src {
            Addr16::Direct => {
                let addr = self.read_next_word();
                let lo = self.interface.get_byte(addr);
                let hi = self.interface.get_byte(addr.wrapping_add(1));
                u16::from_le_bytes([lo, hi])
            }
        }
    }
}

impl<T: Interface> Write16<Addr16> for Cpu<T> {
    #[inline(always)] //IMPORTANT
    fn write_16(&mut self, dst: Addr16, val: u16) {
        match dst {
            Addr16::Direct => {
                let addr = self.read_next_word();
                self.interface.set_byte(addr, val as u8);
                self.interface
                    .set_byte(addr.wrapping_add(1), (val >> 8) as u8)
            }
        }
    }
}

impl<T: Interface> Write8<Addr> for Cpu<T> {
    #[inline(always)] //IMPORTANT
    fn write_8(&mut self, dst: Addr, val: u8) {
        match dst {
            Addr::BC => {
                let addr = self.registers.get_bc();
                self.interface.set_byte(addr, val);
            }
            Addr::DE => {
                let addr = self.registers.get_de();
                self.interface.set_byte(addr, val);
            }
            Addr::HL => {
                let addr = self.registers.get_hl();
                self.interface.set_byte(addr, val);
            }
            Addr::HLD => {
                let addr = self.registers.get_hl();
                self.registers.set_hl(addr.wrapping_sub(1));
                self.interface.set_byte(addr, val);
            }
            Addr::HLI => {
                let addr = self.registers.get_hl();
                self.registers.set_hl(addr.wrapping_add(1));
                self.interface.set_byte(addr, val);
            }
            Addr::Direct => {
                let addr = self.read_next_word();
                self.interface.set_byte(addr, val);
            }
            Addr::ReadOffset(so) => {
                let offset = match so {
                    ReadOffType::Register(r) => self.read_8(r),
                    ReadOffType::Immediate8 => self.read_8(Immediate8),
                };
                self.interface.set_byte(0xFF00 | offset as u16, val);
            }
        }
    }
}

impl<T: Interface> Read8<Addr> for Cpu<T> {
    #[inline(always)] //IMPORTANT
    fn read_8(&mut self, src: Addr) -> u8 {
        match src {
            Addr::BC => self.interface.get_byte(self.registers.get_bc()),
            Addr::DE => self.interface.get_byte(self.registers.get_de()),
            Addr::HL => self.interface.get_byte(self.registers.get_hl()),
            Addr::HLD => {
                let addr = self.registers.get_hl();
                self.registers.set_hl(addr.wrapping_sub(1));
                self.interface.get_byte(addr)
            }
            Addr::HLI => {
                let addr = self.registers.get_hl();
                self.registers.set_hl(addr.wrapping_add(1));
                self.interface.get_byte(addr)
            }
            Addr::Direct => {
                let addr = self.read_next_word();
                self.interface.get_byte(addr)
            }
            Addr::ReadOffset(so) => {
                let offset = match so {
                    ReadOffType::Register(r) => self.read_8(r),
                    ReadOffType::Immediate8 => self.read_8(Immediate8),
                };

                let value = self.interface.get_byte(0xFF00 | offset as u16);

                value
            }
        }
    }
}
