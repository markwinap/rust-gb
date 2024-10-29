#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy)]
pub struct WorkRam(
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))] [u8; 0x2000],
);

impl WorkRam {
    pub fn new() -> WorkRam {
        WorkRam([0; 0x2000])
    }
    #[inline(always)] //IMPORTANT
    pub fn write(&mut self, addr: u16, value: u8) {
        self.0[(addr as usize) & 0x1fff] = value;
    }
    #[inline(always)] //IMPORTANT
    pub fn read(&self, addr: u16) -> u8 {
        self.0[(addr as usize) & 0x1fff]
    }
}
