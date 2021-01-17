
use crate::cpu::address::Cpu;
use crate::cpu::interrupt_handler::{InterruptHandler, InterruptLine};
use crate::cpu::Interface;

pub struct TestMachine<> {
    pub cpu: Cpu<TestHardware>,
    t_cycles: usize,
}
// pub struct TestMachineTwo {
//     pub test_hardware: TestHardware,
//     pub cpu: Cpu<'static, TestHardware>,
//     t_cycles: usize,
// }
impl TestMachine<> {
    fn from_memory(input: &[u8]) -> TestMachine<> {
        TestMachine {
            cpu: Cpu::new(TestHardware::from_memory(input)),
            t_cycles: 0
        }
    }
}
pub struct TestHardware {
    memory: Vec<u8>,
    t_cycles: usize,
    interrupt_handler: InterruptHandler
}

impl TestHardware {
    fn from_memory(input: &[u8]) -> TestHardware {
        let mut memory = vec![0x00; 0x10000];
        memory[0..input.len()].copy_from_slice(input);
        TestHardware {
            memory,
            t_cycles: 0,
            interrupt_handler: InterruptHandler::new()
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

    fn enable(&mut self, interrupt: InterruptLine, enable: bool) {
       self.interrupt_handler.enable(interrupt, enable);
    }

    fn request(&mut self, interrupt: InterruptLine, requested: bool) {
        self.request(interrupt, requested);
    }

    fn acknowledge(&mut self, interrupt: InterruptLine) {
        self.acknowledge(interrupt);
    }

    fn interrupt_master_enabled(&self) -> bool {
        self.interrupt_master_enabled()
    }

    fn requested_interrupts(&self) -> InterruptLine {
        self.interrupt_handler.requested_interrupts
    }

    fn change_interrupt_master_enabled(&mut self, boolean: bool) {
        self.interrupt_handler.interrupt_master_enabled = boolean;
    }


    fn any_enabled(&self) -> bool {
        self.any_enabled()
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

// pub trait Interface {
//     fn set_interrupt_disabled(&mut self, disabled: bool);
//     fn enable(&mut self, interrupt: InterruptLine, enable: bool);
//     fn request(&mut self, interrupt: InterruptLine, requested: bool);
//     fn acknowledge(&mut self, interrupt: InterruptLine);
//     fn interrupt_master_enabled(&self) -> bool;
//     fn requested_interrupts(&self) -> InterruptLine;
//     // fn is_enabled(&self, interrupt: InterruptLine) -> bool;
//     // fn is_requested(&self, interrupt: InterruptLine) -> bool;
//     fn any_enabled(&self) -> bool;
//
//     fn set_byte(&mut self, address: u16, value: u8);
//     fn get_byte(&self, address: u16) -> Option<u8>;
//
//     fn step(&mut self) {}
// }

pub trait FakeHardware {
    fn get_byte(&self, address: u16) -> Option<u8>;
}

pub struct TestStruct<'a, T: FakeHardware> {
    pub hardware: &'a T
}

pub struct HardwareImp<'a, T: FakeHardware> {
    pub the_struct: TestStruct<'a, T>
}
impl <'a, T: FakeHardware> FakeHardware for HardwareImp<'a, T> {
    fn get_byte(&self, address: u16) -> Option<u8> {
        unimplemented!()
    }
}
//
// impl <'a, T: FakeHardware>  HardwareImp<'a, T> where Self : FakeHardware {
//     pub fn new() -> HardwareImp<'a, T> {
//         HardwareImp {
//             the_struct: TestStruct { hardware: &() }
//         }
//     }
// }

// impl <T> FakeHardware for T where T: HardwareImp {}

impl<'a, T: FakeHardware> TestStruct<'a, T> {
    pub fn new(hardware: &'a  T) -> Self {
        TestStruct {
            hardware: hardware
        }
    }
}