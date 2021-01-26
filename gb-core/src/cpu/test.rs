use crate::cpu::address::Cpu;
use crate::hardware::interrupt_handler::{InterruptHandler, InterruptLine};
use crate::cpu::{Interface, Step};


mod test_0x;
mod test_1x;
mod test_2x;
mod test_3x;
mod test_4x;
mod test_5x;
mod test_6x;
mod test_7x;
mod test_add16;
mod test_add16_sp_e;
mod test_ax;
mod test_cx;
mod test_dec16;
mod test_ex;
mod test_fx;
mod test_inc16;
mod test_load16;
mod test_load16_hl_sp_e;
mod test_load16_sp_hl;
mod test_pop16;
mod test_push16;
mod cb_test;

pub struct TestMachine {
    pub cpu: Cpu<TestHardware>,
    t_cycles: usize,
}


impl TestMachine {
    fn from_memory(input: &[u8]) -> TestMachine<> {
        TestMachine {
            cpu: Cpu::new(TestHardware::from_memory(input)),
            t_cycles: 0,
        }
    }
}

pub struct TestHardware {
    memory: Vec<u8>,
    interrupt_handler: InterruptHandler,
}

impl TestHardware {
    fn from_memory(input: &[u8]) -> TestHardware {
        let mut memory = vec![0x00; 0x10000];
        memory[0..input.len()].copy_from_slice(input);
        TestHardware {
            memory,
            interrupt_handler: InterruptHandler::new(),
        }
    }
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
}

impl Interface for TestHardware {
    fn set_interrupt_disabled(&mut self, disabled: bool) {
        self.interrupt_handler.set_interrupt_disabled(disabled);
    }

    // fn enable(&mut self, interrupt: InterruptLine, enable: bool) {
    //     self.interrupt_handler.enable(interrupt, enable);
    // }

    fn request(&mut self, interrupt: InterruptLine, requested: bool) {
        self.interrupt_handler.request(interrupt, requested);
    }

    fn acknowledge(&mut self, interrupt: InterruptLine) {
        self.interrupt_handler.acknowledge(interrupt);
    }

    fn interrupt_master_enabled(&self) -> bool {
        self.interrupt_handler.interrupt_master_enabled
    }

    fn requested_interrupts(&self) -> InterruptLine {
        self.interrupt_handler.requested_interrupts
    }

    fn change_interrupt_master_enabled(&mut self, boolean: bool) {
        self.interrupt_handler.interrupt_master_enabled = boolean;
    }


    fn any_enabled(&self) -> bool {
        self.interrupt_handler.any_enabled()
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    fn get_byte(&self, address: u16) -> Option<u8> {
        Some(self.read(address))
    }

    fn step(&mut self) {
        self.interrupt_handler.step();
    }
}

pub fn run_test(instructions: &[u8], init: impl Fn(&mut TestMachine)) -> TestMachine {
    let mut memory = instructions.to_vec();
    memory.push(0xed);

    let mut machine = TestMachine::from_memory(&memory);
    init(&mut machine);
    machine.cpu.step();
    machine.t_cycles = 0;
    while machine.cpu.op_code != 0xed && machine.cpu.state != Step::Halt {
        machine.t_cycles +=  machine.cpu.step() as usize;
    }
    machine
}