use crate::memory::Memory;
use crate::hardware::interrupt_handler::{InterruptHandler, InterruptLine};
use std::ops::Index;
use bitflags::_core::ops::IndexMut;
bitflags!(
  struct TacReg: u8 {
    const ENABLE = 0b100;
    const MASK_1 = 0b010;
    const MASK_0 = 0b001;
  }
);

pub const FREQUENCY: u32 = 4 * 1024 * 1024;

impl TacReg {
    fn counter_mask(&self) -> u16 {
        match self.bits() & 0b11 {
            0b11 => (1 << 5),
            0b10 => (1 << 3),
            0b01 => (1 << 1),
            _ => (1 << 7),
        }
    }
    fn to_frequency(&self) -> u32 {
        match self.bits() & 0b11 {
            0b11 => 16284,
            0b10 => 65536,
            0b01 => 262144,
            _ => 4096,
        }
    }
}

pub struct Timer {
    enabled: bool,
    tac: TacReg,
    divider_cycles: isize,
    divider_counter: u8,
    timer_cycles: i32,
    timer_counter: u8,
    timer_modulo: u8,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            enabled: false,
            tac: TacReg::empty(),
            divider_cycles: 0,
            divider_counter: 0,
            timer_cycles: 0,
            timer_counter: 0,
            timer_modulo: 0,
        }
    }
    pub fn do_cycle(&mut self, ticks: u32, interrupts: &mut InterruptHandler) {
        self.divider_cycles -= ticks as isize;
        while self.divider_cycles <= 0 {
            self.divider_cycles += 255;
            self.divider_counter = (self.divider_counter.wrapping_add(1)) & 0xFF;
        }

        if self.enabled {
            self.timer_cycles -= ticks as i32;
            while self.timer_cycles <= 0 {
                self.timer_cycles += self.tac.to_frequency() as i32;
                if self.timer_counter == 0xFF {
                    self.timer_counter = self.timer_modulo;
                    interrupts.request(InterruptLine::TIMER, true);
                } else {
                    self.timer_counter += 1;
                }
            }
        }
    }
}


impl Memory for Timer {
    fn set_byte(&mut self, address: u16, value: u8) {
        let current_tac = self.tac;
        match address {
            0xFF04 => { self.divider_counter = 0; }
            0xFF05 => { self.timer_counter = value; }
            0xFF06 => { self.timer_modulo = value; }
            0xFF07 => {
                self.tac = TacReg::from_bits_truncate(value);
                self.enabled = self.tac.contains(TacReg::ENABLE);
            }
            _ => panic!("Timer does not handler write {:4X}", address),
        };

        if current_tac.to_frequency() != self.tac.to_frequency() {
            self.timer_cycles = (FREQUENCY / self.tac.to_frequency()) as i32;
        }
    }

    fn get_byte(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.divider_counter,
            0xFF05 => self.timer_counter,
            0xFF06 => self.timer_modulo,
            0xFF07 => {
                const TAC_UNUSED: u8 = 0b1111_1000;
                TAC_UNUSED | self.tac.bits()
            }
            _ => panic!("Timer does not handler read {:4X}", address),
        }
    }
}