

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
