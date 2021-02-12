use bitflags::_core::fmt::{Display, Formatter};

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Default, Eq, PartialEq, Debug)]
pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

impl Display for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "z: {}, n: {}, h: {}, c: {}", self.z, self.n, self.h, self.c)
    }
}

impl Flags {
    pub fn empty() -> Flags {
        Flags::default()
    }
    pub fn set_value(&mut self, v: u8) {
        self.z = (v & (1 << ZERO_FLAG_BYTE_POSITION)) != 0;
        self.n = (v & (1 << SUBTRACT_FLAG_BYTE_POSITION)) != 0;
        self.h = (v & (1 << HALF_CARRY_FLAG_BYTE_POSITION)) != 0;
        self.c = (v & (1 << CARRY_FLAG_BYTE_POSITION)) != 0;
    }

    pub fn read_value(&self) -> u8 {
        let z = self.z as u8;
        let n = self.n as u8;
        let h = self.h as u8;
        let c = self.c as u8;
        (z << ZERO_FLAG_BYTE_POSITION) | (n << SUBTRACT_FLAG_BYTE_POSITION) | (h << HALF_CARRY_FLAG_BYTE_POSITION) | (c << CARRY_FLAG_BYTE_POSITION)
    }
}

impl core::convert::From<Flags> for u8 {
    fn from(flag: Flags) -> u8 {
        flag.read_value()
    }
}

impl core::convert::From<u8> for Flags {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        Flags {
            z: zero,
            n: subtract,
            h: half_carry,
            c: carry,
        }
    }
}
