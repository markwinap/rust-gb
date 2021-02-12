use alloc::boxed::Box;

#[derive(Clone)]
pub struct WorkRam {
    ram: Box<[u8; 0x2000]>,
}

impl WorkRam {
    pub fn new() -> WorkRam {
        WorkRam {
            ram: Box::new([0; 0x2000]),
        }
    }

    pub fn read_lower(&self, addr: u16) -> u8 {
        self.ram[(addr as usize) & 0x1fff]
    }
    pub fn write_lower(&mut self, addr: u16, value: u8) {
        self.ram[(addr as usize) & 0x1fff] = value;
    }

    pub fn read_upper(&self, addr: u16) -> u8 {
        self.ram[(addr as usize) & 0x1fff]
    }
    pub fn write_upper(&mut self, addr: u16, value: u8) {
        self.ram[(addr as usize) & 0x1fff] = value;
    }
}
