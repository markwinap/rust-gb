use crate::memory::nmmu::Memory;
use crate::hardware::interrupt_handler::{InterruptHandler, InterruptLine};
bitflags!(
  struct TacReg: u8 {
    const ENABLE = 0b100;
    const MASK_1 = 0b010;
    const MASK_0 = 0b001;
  }
);

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
            0b11 => 256,
            0b10 => 64,
            0b01 => 16,
            _ => 1024,
        }
    }
}

pub struct Timer {
    enabled: bool,
    divider: u8,
    counter: u8,
    modulo: u8,
    tac: TacReg,
    internalcnt: u32,
    internaldiv: u32,
}

impl Timer {
    pub fn do_cycle(&mut self, ticks: u32, interrupts: &mut InterruptHandler) {
        self.internaldiv += ticks;
        while self.internaldiv >= 256 {
            self.divider = self.divider.wrapping_add(1);
            self.internaldiv -= 256;
        }

        if self.enabled {
            self.internalcnt += ticks;

            while self.internalcnt >= self.tac.to_frequency() {
                self.counter = self.counter.wrapping_add(1);
                if self.counter == 0 {
                    self.counter = self.modulo;
                    interrupts.request(InterruptLine::TIMER, true);
                }
                self.internalcnt -= self.tac.to_frequency();
            }
        }
    }
}

impl Memory for Timer {
    fn set_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => { self.divider = 0; }
            0xFF05 => { self.counter = value; }
            0xFF06 => { self.modulo = value; }
            0xFF07 => {
                self.tac = TacReg::from_bits_truncate(value);
                self.enabled = self.tac.contains(TacReg::ENABLE);
            }
            _ => panic!("Timer does not handler write {:4X}", address),
        };
    }

    fn get_byte(&self, address: u16) -> Option<u8> {
        let value = match address {
            0xFF04 => self.divider,
            0xFF05 => self.counter,
            0xFF06 => self.modulo,
            0xFF07 => {
                const TAC_UNUSED: u8 = 0b1111_1000;
                TAC_UNUSED | self.tac.bits()
            }
            _ => panic!("Timer does not handler read {:4X}", address),
        };
        Some(value)
    }
}