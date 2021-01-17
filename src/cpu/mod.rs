use crate::cpu::address::Cpu;
use crate::cpu::interrupt_handler::InterruptLine;
use crate::cpu::registers::Registers;

pub mod flags;

pub mod interrupt_handler;
pub mod registers;
pub mod address;
pub mod alu;
mod opcodes;

pub enum Step {
    Run,
    Interrupt,
    Halt,
    HaltBug,
    Stopped,
}

pub trait Interface {
    fn set_interrupt_disabled(&mut self, disabled: bool);
    fn enable(&mut self, interrupt: InterruptLine, enable: bool);
    fn request(&mut self, interrupt: InterruptLine, requested: bool);
    fn acknowledge(&mut self, interrupt: InterruptLine);
    fn interrupt_master_enabled(&self) -> bool;
    fn requested_interrupts(&self) -> InterruptLine;
    fn change_interrupt_master_enabled(&mut self, boolean: bool);
    // fn is_enabled(&self, interrupt: InterruptLine) -> bool;
    // fn is_requested(&self, interrupt: InterruptLine) -> bool;
    fn any_enabled(&self) -> bool;

    fn set_byte(&mut self, address: u16, value: u8);
    fn get_byte(&self, address: u16) -> Option<u8>;

    fn step(&mut self) {}
}


impl<T: Interface> Cpu<T> {
    pub fn new(interface: T) -> Self {
        Cpu {
            registers: Registers::default(),
            op_code: 0x00,
            interface,
            state: Step::Run,
        }
    }
}

impl<T: Interface> Cpu<T> {
    pub fn step(&mut self) -> u8 {
        let (cycles, step) = match self.state {
            Step::Run => {
                let ((step, _), cycles) = self.decode();
                (cycles, step)
            }
            Step::Interrupt => {
                self.interface.change_interrupt_master_enabled(false);
                let interrupt = self.interface.requested_interrupts().highest_priority();
                self.interface.acknowledge(interrupt);
                self.registers.pc = match interrupt {
                    InterruptLine::VBLANK => 0x0040,
                    InterruptLine::STAT => 0x0048,
                    InterruptLine::TIMER => 0x0050,
                    InterruptLine::SERIAL => 0x0058,
                    InterruptLine::JOYPAD => 0x0060,
                    _ => 0x0000,
                };
                self.op_code = self.interface.get_byte(self.registers.pc).unwrap();
                self.interface.step();
                (0, Step::Run)
            }
            Step::Halt => {
                if self.interface.any_enabled() {
                    let (step, _) = self.handle_return(self.registers.pc);
                    (4, step)
                } else {
                    (4, Step::Halt)
                }
            }
            Step::HaltBug => {
                let current_pc = self.registers.pc;
                let ((step, _), cycles) = self.decode();
                self.registers.pc = current_pc;
                (cycles, step)
            }
            Step::Stopped => {
                panic!()
            }
        };
        cycles
    }
}
