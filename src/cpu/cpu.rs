use crate::cpu::registers::{Registers, Reg8, Reg16};
use crate::memory::mmu::Mmu;
use crate::memory::AddressSpace;
use crate::cpu::interrupt_handler::InterruptHandler;
use crate::cpu::Step;

pub trait CpuContext {
    fn read_cycle(&mut self, addr: u16) -> u8;
    fn read_cycle_high(&mut self, addr: u8) -> u8 {
        self.read_cycle(0xff00 | (addr as u16))
    }
    fn write_cycle(&mut self, addr: u16, data: u8);
    fn write_cycle_high(&mut self, addr: u8, data: u8) {
        self.write_cycle(0xff00 | (addr as u16), data);
    }
}

// pub enum State{
//     Normal,
//     HaltState,
//     Stopped,
//     HaltBug,
// }

pub struct Cpu {
    pub registers: Registers,
    pub op_code: u8,
    pub bus: Mmu,
    pub interrupt_handler: InterruptHandler,
    pub state: Step
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::default(),
            op_code: 0x00,
            bus: Mmu::new(),
            interrupt_handler: InterruptHandler::new(),
            state: Step::Run
        }
    }

    pub fn read_next_byte(&mut self) -> u8 {
        let addr = self.registers.get_pc();
        self.registers.increment_pc();
        self.bus.get_byte(addr).unwrap()
    }

    pub fn read_next_word(&mut self) -> u16 {
        let lo = self.read_next_byte();
        let hi = self.read_next_byte();
        u16::from_le_bytes([lo, hi])
    }

    pub fn push_u16(&mut self, value: u16) {
        let [lo, hi] = u16::to_le_bytes(value);
        self.registers.sp =  self.registers.sp.wrapping_sub(1);
        self.bus.set_byte(self.registers.sp, hi);
        self.registers.sp =  self.registers.sp.wrapping_sub(1);
        self.bus.set_byte(self.registers.sp, lo);
    }

    pub fn pop_u16(&mut self) -> u16 {
        let lo = self.bus.get_byte(self.registers.sp).unwrap();
        self.registers.sp =  self.registers.sp.wrapping_add(1);
        let hi = self.bus.get_byte(self.registers.sp).unwrap();
        self.registers.sp =  self.registers.sp.wrapping_add(1);
        u16::from_le_bytes([lo, hi])
    }
}

impl In8<Reg8> for Cpu {
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

impl Out8<Reg8> for Cpu {
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

impl In16<Reg16> for Cpu {
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

impl Out16<Reg16> for Cpu {
    fn write_16(&mut self, dst: Reg16, val: u16) {
        match dst {
            Reg16::AF => self.registers.set_af(val),
            Reg16::BC => self.registers.set_bc(val),
            Reg16::DE => self.registers.set_de(val),
            Reg16::HL => self.registers.set_hl(val),
            Reg16::SP => self.registers.sp =  val,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Immediate8;

#[derive(Clone, Copy, Debug)]
pub struct Immediate16;

impl In8<Immediate8> for Cpu {
    fn read_8(&mut self, _: Immediate8) -> u8 {
        self.read_next_byte()
    }
}

impl In16<Immediate16> for Cpu {
    fn read_16(&mut self, _: Immediate16) -> u16 {
        self.read_next_word()
    }
}

pub trait In8<T: Copy> {
    fn read_8(&mut self, src: T) -> u8;
}

pub trait Out8<T: Copy> {
    fn write_8(&mut self, dst: T, val: u8);
}

pub trait In16<T: Copy> {
    fn read_16(&mut self, src: T) -> u16;
}

pub trait Out16<T: Copy> {
    fn write_16(&mut self, dst: T, val: u16);
}

#[derive(Clone, Copy, Debug)]
pub enum Addr where {
    BC,
    DE,
    HL,
    HLD,
    HLI,
    Direct,
    ReadOffset(ReadOffType)
}


#[derive(Clone, Copy, Debug)]
pub enum Addr16 {
    //(nn)
    Direct,
}

#[derive(Clone, Copy, Debug)]
pub enum ReadOffType {
    Register(Reg8),
    Immediate8,
}

impl In16<Addr16> for Cpu {
    fn read_16(&mut self, src: Addr16) -> u16 {
        match src {
            Addr16::Direct => {
                let addr = self.read_next_word();
                let lo = self.bus.get_byte(addr).unwrap();
                let hi = self.bus.get_byte(addr.wrapping_add(1)).unwrap();
                u16::from_le_bytes([lo, hi])
            }
        }
    }
}

impl Out16<Addr16> for Cpu {
    fn write_16(&mut self, dst: Addr16, val: u16) {
        match dst {
            Addr16::Direct => {
                let addr = self.read_next_word();
                self.bus.set_byte(addr, val as u8);
                self.bus.set_byte(addr.wrapping_add(1), (val >> 8) as u8)
            }
        }
    }
}

impl Out8<Addr> for Cpu {
    fn write_8(&mut self, dst: Addr, val: u8) {
        match dst {
            Addr::BC => {
                let addr = self.registers.get_bc();
                self.bus.set_byte(addr, val);
            }
            Addr::DE => {
                let addr = self.registers.get_de();
                self.bus.set_byte(addr, val);
            }
            Addr::HL => {
                let addr = self.registers.get_hl();
                self.bus.set_byte(addr, val);
            }
            Addr::HLD => {
                let addr = self.registers.get_hl();
                self.registers.set_hl(addr.wrapping_sub(1));
                self.bus.set_byte(addr, val);
            }
            Addr::HLI => {
                let addr = self.registers.get_hl();
                self.registers.set_hl(addr.wrapping_add(1));
                self.bus.set_byte(addr, val);
            }
            Addr::Direct => {
                let addr = self.read_next_word();
                self.bus.set_byte(addr, val);
            }
            Addr::ReadOffset(so) => {
                let offset = match so {
                    ReadOffType::Register(r) => self.read_8(r),
                    ReadOffType::Immediate8 => self.read_8(Immediate8),
                };
                self.bus.set_byte(0xFF00 | offset as u16, val);
            }
        }
    }
}

impl In8<Addr> for Cpu {
    fn read_8(&mut self, src: Addr) -> u8 {
        match src {
            Addr::BC => {
                self.bus.get_byte(self.registers.get_bc()).unwrap()
            }
            Addr::DE => {
                self.bus.get_byte(self.registers.get_de()).unwrap()
            }
            Addr::HL => {
                self.bus.get_byte(self.registers.get_hl()).unwrap()
            }
            Addr::HLD => {
                let addr = self.registers.get_hl();
                self.registers.set_hl(addr.wrapping_sub(1));
                self.bus.get_byte(addr).unwrap()
            }
            Addr::HLI => {
                let addr = self.registers.get_hl();
                self.registers.set_hl(addr.wrapping_add(1));
                self.bus.get_byte(addr).unwrap()
            }
            Addr::Direct => {
                let addr = self.read_next_word();
                self.bus.get_byte(addr).unwrap()
            }
            Addr::ReadOffset(so) => {
                let offset = match so {
                    ReadOffType::Register(r) => self.read_8(r),
                    ReadOffType::Immediate8 => self.read_8(Immediate8),
                };
                self.bus.get_byte(0xFF00 | offset as u16).unwrap()
            }
        }
    }
}
