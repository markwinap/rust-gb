use crate::cpu::cpu::Cpu;
use crate::cpu::interrupt_handler::InterruptLine;
use crate::memory::AddressSpace;

pub mod flags;
pub mod error;

pub mod interrupt_manager;
pub mod interrupt_handler;
pub mod registers;
pub mod op;
pub mod cpu;
pub mod alu;

pub enum Step {
    Run,
    Interrupt,
    Halt,
    HaltBug,
    Stopped,
}

pub struct StateHandler {}

impl Cpu {
    pub fn step(&mut self) {
        let (cycles, step) = match self.state {
            Step::Run => {
                let ((step, _), cycles) = self.decode();
                (cycles, step)
            }
            Step::Interrupt => {
                self.interrupt_handler.interrupt_master_enabled = false;
                let interrupt = self.interrupt_handler.requested_interrupts.highest_priority();
                self.interrupt_handler.acknowledge(interrupt);
                self.registers.pc = match interrupt {
                    InterruptLine::VBLANK => 0x0040,
                    InterruptLine::STAT => 0x0048,
                    InterruptLine::TIMER => 0x0050,
                    InterruptLine::SERIAL => 0x0058,
                    InterruptLine::JOYPAD => 0x0060,
                    _ => 0x0000,
                };
                self.op_code = self.bus.get_byte(self.registers.pc).unwrap();
                self.interrupt_handler.step();
                (0, Step::Run)
            }
            Step::Halt => {
                if self.interrupt_handler.any_enabled() {
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
    }
}