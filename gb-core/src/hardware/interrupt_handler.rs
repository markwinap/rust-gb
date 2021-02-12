use bitflags::bitflags;
use crate::util::int::IntExt;
use core::fmt::Display;
use bitflags::_core::fmt::Formatter;

bitflags!(
  pub struct InterruptLine: u8 {
    const VBLANK = 1 << 0;
    const STAT = 1 << 1;
    const TIMER = 1 << 2;
    const SERIAL = 1 << 3;
    const JOYPAD = 1 << 4;
  }
);


impl InterruptLine {
    pub fn highest_priority(&self) -> InterruptLine {
        InterruptLine::from_bits_truncate(self.bits().isolate_rightmost_one())
    }
}

pub struct InterruptHandler {
    pub interrupt_master_enabled: bool,
    pub enable_delay: u8,
    pub enabled_interrupts: InterruptLine,
    pub requested_interrupts: InterruptLine,

}

impl InterruptHandler {
    pub fn new() -> Self {
        Self {
            interrupt_master_enabled: true,
            enable_delay: 0,
            enabled_interrupts: InterruptLine::empty(),
            requested_interrupts: InterruptLine::empty(),
        }
    }
    pub fn step(&mut self) {
        match self.enable_delay {
            2 => self.enable_delay -= 1,
            1 => {
                self.interrupt_master_enabled = true;
                self.enable_delay -= 1
            }
            _ => {}
        }
    }

    pub fn set_interrupt_disabled(&mut self, disabled: bool) {
        if !disabled {
            self.enable_delay = 2;
        } else {
            self.interrupt_master_enabled = false;
        }
    }

    pub fn enable(&mut self, interrupt: InterruptLine, enable: bool) {
        if enable {
            self.enabled_interrupts |= interrupt;
        } else {
            self.enabled_interrupts -= interrupt;
        }
    }

    pub fn request(&mut self, interrupt: InterruptLine, requested: bool) {
        if requested {
            self.requested_interrupts.insert(interrupt);
        } else {
            self.requested_interrupts.remove(interrupt);
        }
    }

    pub fn acknowledge(&mut self, interrupt: InterruptLine) {
        self.request(interrupt, false);
    }

    pub fn is_enabled(&self, interrupt: InterruptLine) -> bool {
        self.enabled_interrupts.contains(interrupt)
    }

    pub fn is_requested(&self, interrupt: InterruptLine) -> bool {
        self.requested_interrupts.contains(interrupt)
    }

    pub fn any_enabled(&self) -> bool {
        !(self.enabled_interrupts & self.requested_interrupts).is_empty()
    }

    pub fn get_interrupt_flag(&self) -> u8 {
        const IF_UNUSED_MASK: u8 = (1 << 5) | (1 << 6) | (1 << 7);

        self.requested_interrupts.bits() | IF_UNUSED_MASK
    }


    pub fn set_interrupt_flag(&mut self, value: u8)  {
        self.requested_interrupts = InterruptLine::from_bits_truncate(value);
    }

    pub fn get_enabled_interrupts_flag(&self) -> u8 {
        const IF_UNUSED_MASK: u8 = (1 << 5) | (1 << 6) | (1 << 7);

        self.enabled_interrupts.bits() | IF_UNUSED_MASK
    }

    pub fn set_enabled_interrupts_flag(&mut self, value: u8)  {
        self.enabled_interrupts = InterruptLine::from_bits_truncate(value);
    }

}