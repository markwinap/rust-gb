use crate::cpu::address::Cpu;
use crate::cpu::registers::Registers;
use crate::hardware::interrupt_handler::InterruptLine;
use crate::is_log_enabled;

pub mod flags;

pub mod address;
pub mod alu;
mod opcodes;
pub mod registers;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Step {
    Run,
    Interrupt,
    Halt,
    HaltBug,
    Stopped,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy)]
pub struct CpuState {
    pub registers: Registers,
    pub op_code: u8,
    pub state: Step,
}
pub trait Interface {
    fn set_interrupt_disabled(&mut self, disabled: bool);
    fn request(&mut self, interrupt: InterruptLine, requested: bool);
    fn acknowledge(&mut self, interrupt: InterruptLine);
    fn interrupt_master_enabled(&self) -> bool;
    fn requested_interrupts(&self) -> InterruptLine;
    fn change_interrupt_master_enabled(&mut self, boolean: bool);
    fn reset(&mut self);
    fn any_enabled(&self) -> bool;
    fn set_byte(&mut self, address: u16, value: u8);
    fn get_byte(&mut self, address: u16) -> u8;
    fn interface_step(&mut self) {}
    fn gpu_screen_on(&self) -> bool;
    fn scan_line(&self) -> u8;
}

impl<T: Interface> Cpu<T> {
    pub fn new(interface: T) -> Self {
        Cpu {
            registers: Registers::default(),
            op_code: 0x00,
            interface,
            state: Step::Run,
            tick_count: 0,
            current_screen_state: false,
            active_print: false,
        }
    }

    pub fn new_from_state(interface: T, state: CpuState) -> Self {
        Cpu {
            registers: state.registers,
            op_code: state.op_code,
            interface,
            state: state.state,
            tick_count: 0,
            current_screen_state: false,
            active_print: false,
        }
    }

    pub fn create_state(&self) -> CpuState {
        CpuState {
            registers: self.registers,
            op_code: self.op_code,
            state: self.state,
        }
    }
}

impl<T: Interface> Cpu<T> {
    pub fn reset(&mut self) {
        self.registers.pc = 0x100;
        self.push_u16(0xFFFE);
        self.registers.set_af(0x0180);
        self.registers.set_bc(0x0013);
        self.registers.set_de(0x00D8);
        self.registers.set_hl(0x014D);

        self.interface.reset();
    }

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
                self.push_u16(self.registers.pc);

                let interrupt_address = match interrupt {
                    InterruptLine::VBLANK => 0x0040,
                    InterruptLine::STAT => 0x0048,
                    InterruptLine::TIMER => 0x0050,
                    InterruptLine::SERIAL => 0x0058,
                    InterruptLine::JOYPAD => 0x0060,
                    _ => 0x0000,
                };
                self.op_code = self.interface.get_byte(interrupt_address);
                self.registers.pc = interrupt_address.wrapping_add(1);
                self.interface.interface_step();
                (20, Step::Run)
            }
            Step::Halt => {
                if self.interface.any_enabled() {
                    if is_log_enabled() {
                        // println!("OUT OF HALT");
                    }
                    let (step, _) = self.handle_return(self.registers.pc);
                    (0, step)
                } else {
                    (4, Step::Halt)
                }
            }
            Step::HaltBug => {
                let current_pc = self.registers.pc;
                println!("Current HALT PC: {}", self.registers.pc);
                let ((step, _), cycles) = self.decode();
                self.registers.pc = current_pc;
                (cycles, step)
            }
            Step::Stopped => {
                panic!()
            }
        };
        self.state = step;
        cycles
    }
}
