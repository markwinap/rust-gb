use crate::cpu::cpu::Cpu;
use crate::cpu::interrupt_handler::InterruptLine;

pub mod flags;
pub mod error;

pub mod interrupt_manager;
pub mod interrupt_handler;
pub mod registers;
pub mod op;
pub mod cpu;
pub mod alu;
pub mod toycpu;

pub enum Step {
    Run,
    Interrupt,
    Halt,
    HaltBug,
    Stopped,
}

pub struct StateHandler {}

impl Cpu {
    pub fn execute_step(&mut self, step: Step) {
        let foo = match step {
            Step::Run => {
                let ((step, address), cycles) = self.decode();
                if let Step::Run = step {
                 //   self.registers.pc = address;
                }
            }
            Step::Interrupt => {
                //  self.ime =  false;
                //In this case it seems it is immediate compared to the case when enable done on EI opcode
                self.interrupt_handler.interrupt_master_enabled = false;
                self.push_u16(self.registers.pc);

                let interrupt = self.interrupt_handler.requested_interrupts.highest_priority();
                self.interrupt_handler.acknowledge(interrupt);
                //TODO Get highest interrupt
                //TODO acknowledge interrupt (thus removing its flag)
                //TODO set PC to interrupt location

                self.registers.pc = match interrupt {
                    InterruptLine::VBLANK => 0x0040,
                    InterruptLine::STAT => 0x0048,
                    InterruptLine::TIMER => 0x0050,
                    InterruptLine::SERIAL => 0x0058,
                    InterruptLine::JOYPAD => 0x0060,
                    _ => 0x0000,
                };
                //MAYBE SET OPCODE // handled by handle_return() in opcodes.rs

                //SWITCH TO RUNNING STATE
            }
            Step::Halt => {
                if self.interrupt_handler.any_enabled() {
                    // Step:: InterruptDispatch?
                } else {
                    //Step:: Halt
                }
            }
            Step::HaltBug => {}
            Step::Stopped => {}
        };
    }
}