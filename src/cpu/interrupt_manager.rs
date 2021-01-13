use crate::memory::AddressSpace;

pub enum InterruptType {
    VBlank,
    LCDC,
    Timer,
    Serial,
    P10_13
}

impl InterruptType {
    pub fn get_handler(&self) -> u16 {
        match self {
            Self::VBlank => 0x0040,
            Self::LCDC => 0x0048,
            Self::Timer => 0x0050,
            Self::Serial => 0x0058,
            Self::P10_13 => 0x0060
        }
    }

    pub fn get_ordinal(&self) -> u8 {
        match self {
            Self::VBlank => 0,
            Self::LCDC => 1,
            Self::Timer => 2,
            Self::Serial => 3,
            Self::P10_13 => 4
        }
    }
}

pub struct InterruptManager {
    gbc: bool,
    ime: bool,
    interrupt_flag: u8,
    //0xe1
    interrupt_enabled: u8,
    pending_enable_interrupts: i32,
    //?
    pending_disable_interrupts: i32, //?
}

impl InterruptManager {
    pub fn new(gbc: bool) -> Self {
        Self {
            gbc,
            ime: false,
            interrupt_flag: 0xe1,
            interrupt_enabled: 0,
            pending_enable_interrupts: -1,
            pending_disable_interrupts: -1,
        }
    }

    pub fn enable_interrupts(&mut self, with_delay: bool) {
        self.pending_disable_interrupts = -1;
        if with_delay {
            if self.pending_enable_interrupts == -1 {
                self.pending_enable_interrupts = 1;
            }
        } else {
            self.pending_enable_interrupts = -1;
            self.ime = true;
        }
    }

    pub fn disable_interrupts(&mut self, with_delay: bool) {
        self.pending_disable_interrupts = -1;
        if with_delay && self.gbc {
            if self.pending_disable_interrupts == -1 {
                self.pending_disable_interrupts = 1;
            }
        } else {
            self.pending_disable_interrupts = -1;
            self.ime = false;
        }
    }

    pub fn request_interrupt(&mut self, interrupt_type: InterruptType) {
        self.interrupt_flag |= 1 << interrupt_type.get_ordinal();
    }

    pub fn clear_interrupt(&mut self, interrupt_type: InterruptType) {
        self.interrupt_flag |= 0 << interrupt_type.get_ordinal();
    }

    pub fn on_instruction_finished(&mut self) {
        if self.pending_enable_interrupts != -1 {
            let current = self.pending_enable_interrupts;
            self.pending_enable_interrupts -= 1;
            if current == 0 {
                self.enable_interrupts(false);
            }
        }

        if self.pending_disable_interrupts != -1 {
            let current = self.pending_disable_interrupts;
            self.pending_disable_interrupts -= 1;
            if current == 0 {
                self.disable_interrupts(false);
            }
        }
    }

    pub fn is_interrupt_requested(&self) -> bool {
        (self.interrupt_flag & self.interrupt_enabled) != 0
    }

    pub fn is_halt_bug(&self) -> bool {
        (self.interrupt_flag & self.interrupt_enabled & 0x1f) != 0 && !self.ime
    }
}

impl AddressSpace for InterruptManager {
    fn accepts(&self, address: u16) -> bool {
        address == 0xff0f || address == 0xffff
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        match address {
            0xff0f => {
                self.interrupt_flag = value | 0xe0;
            }
            0xffff => {
                self.interrupt_enabled = value;
            }
            _ => {}
        };
    }

    fn get_byte(&self, address: u16) -> Option<u8> {
        match address {
            0xff0f => Some(self.interrupt_flag),
            0xffff => Some(self.interrupt_enabled),
            _ => Some(0xff)
        }
    }
}