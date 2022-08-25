use bitflags::_core::fmt::{Display, Formatter};

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Default, Eq, PartialEq, Debug)]
pub struct Flags(u8);

// impl Display for Flags {
//     fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
//         write!(f, "z: {}, n: {}, h: {}, c: {}", self.z, self.n, self.h, self.c)
//     }
// }
pub enum CpuFlag
{
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}

impl Flags {
    pub fn empty() -> Flags {
        Flags::default()
    }
    pub fn set_value(&mut self, v: u8) {
        self.0 = v;
    }

    pub fn read_value(&self) -> u8 {
        self.0
    }

    
}

impl core::convert::From<Flags> for u8 {
    fn from(flag: Flags) -> u8 {
        flag.read_value()
    }
}

impl core::convert::From<u8> for Flags {
    fn from(byte: u8) -> Self {
        Flags(byte)
    }
}
