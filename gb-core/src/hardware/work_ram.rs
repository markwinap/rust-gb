pub struct WorkRam([u8; 0x2000]);

impl WorkRam {
    pub fn new() -> WorkRam {
        WorkRam([0; 0x2000])
    }

    pub fn read_lower(&self, addr: u16) -> u8 {
        self.0[(addr as usize) & 0x1fff]
    }
    pub fn write_lower(&mut self, addr: u16, value: u8) {
        self.0[(addr as usize) & 0x1fff] = value;
    }

    pub fn read_upper(&self, addr: u16) -> u8 {
        self.0[(addr as usize) & 0x1fff]
    }
    pub fn write_upper(&mut self, addr: u16, value: u8) {
        self.0[(addr as usize) & 0x1fff] = value;
    }
}
