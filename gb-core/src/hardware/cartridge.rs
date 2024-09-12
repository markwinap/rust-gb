use crate::memory::Memory;
use bitflags::_core::ops::{Index, IndexMut};
use alloc::boxed::Box;

pub trait Cartridge {
    fn step(&mut self) {}

    fn read_rom(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);

    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);
}

pub struct ReadOnlyMemoryCartridge<'a> {
    bytes: &'a [u8],
}

impl<'a> Cartridge for ReadOnlyMemoryCartridge<'a> {
    fn read_rom(&self, address: u16) -> u8 {
        self.get_byte(address)
    }

    fn write_rom(&mut self, _: u16, _: u8) {}

    fn read_ram(&self, address: u16) -> u8 {
        self.get_byte(address)
    }

    fn write_ram(&mut self, _: u16, _: u8) {}
}

impl<'a> ReadOnlyMemoryCartridge<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self {
            bytes
        }
    }
}

impl<'a> Memory for ReadOnlyMemoryCartridge<'a> {
    fn set_byte(&mut self, _: u16, _: u8) {}

    fn get_byte(&self, address: u16) -> u8 {
        let result = self.bytes[address as usize];
        result
    }
}


#[derive(Eq, PartialEq)]
enum MemoryMode {
    MBit16Rom8KByteRam,
    MBit4Rom32KByteRam,
}

pub struct Mbc1Cartridge<'a> {
    bytes: &'a [u8],
    bank_ram: BankableRam,
    current_rom_bank: u8,
    mode: MemoryMode,
    ram_mode: bool,

}

impl<'a> Cartridge for Mbc1Cartridge<'a> {
    fn read_rom(&self, address: u16) -> u8 {
        self.get_byte(address)
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        self.set_byte(address, value)
    }

    fn read_ram(&self, address: u16) -> u8 {
        if self.bank_ram.enabled {
            return 0;
        }
        self.get_byte(address)
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.set_byte(address, value)
    }
}

impl<'a> Memory for Mbc1Cartridge<'a> {
    fn set_byte(&mut self, address: u16, mut data: u8) {
        if address < 0x2000 {
            self.bank_ram.enable((data & 0b0000_1010) != 0);
        } else if address < 0x4000 {
            self.current_rom_bank = (data & 0b0001_1111);
            if self.current_rom_bank == 0 || self.current_rom_bank == 0x20 || self.current_rom_bank == 0x40 || self.current_rom_bank == 0x60 {
                self.current_rom_bank += 1;
            }
        } else if address < 0x6000 {
            if self.mode == MemoryMode::MBit4Rom32KByteRam {
                self.bank_ram.select_bank(data & 0b0000_0011);
            } else {
                data = (data & 0b0000_0011) << 5;
                self.current_rom_bank = (self.current_rom_bank & 0b0001_1111) + data;
            }
        } else if address < 0x8000 {
            data = data & 0b0000_0001;
            if data == 1 {
                self.mode = MemoryMode::MBit4Rom32KByteRam;
            } else {
                self.mode = MemoryMode::MBit16Rom8KByteRam
            }
        } else if Mbc1Cartridge::compare(address, 0xA000, 0xBFFF) == 0 {
            self.bank_ram[address - 0xA000] = data;
        }
    }

    fn get_byte(&self, address: u16) -> u8 {
        if Mbc1Cartridge::compare(address, 0x4000, 0x7FFF) == 0 {
            return self.bytes[(address + ((self.current_rom_bank as u16 - 1) * (0x7FFF - 0x4000 + 1))) as usize];
        } else if Mbc1Cartridge::compare(address, 0xA000, 0xBFFF) == 0 {
            return self.bank_ram.get_byte(address - 0xA000);
        }
        self.bytes[address as usize]
    }
}

impl<'a> Mbc1Cartridge<'a> {
    pub fn compare(value: u16, from: u16, to: u16) -> isize {
        if value < from {
            return value as isize - from as isize;
        } else if value > to {
            return value as isize - to as isize;
        }
        return 0;
    }

    pub fn new(bytes: &'a [u8], bank_ram: BankableRam) -> Self {
        Self {
            bytes,
            bank_ram,
            current_rom_bank: 1,
            mode: MemoryMode::MBit16Rom8KByteRam,
            ram_mode: false,
        }
    }
}

pub struct BankableRam {
    current_bank: u8,
    enabled: bool,
    banks: Box<[[u8; 0xBFFF - 0xA000 + 1]]>
}

impl BankableRam {
    pub fn new(banks: u8) -> Self {
        Self {
            banks: (0..banks).map(|_| [0; 0xBFFF - 0xA000 + 1]).collect(),
            current_bank: 0,
            enabled: false,
        }
    }

    pub fn enable(&mut self, enable: bool) {
        self.enabled = enable;
    }
    pub fn select_bank(&mut self, bank: u8) {
        self.current_bank = bank;
    }
}

impl Memory for BankableRam {
    fn set_byte(&mut self, address: u16, value: u8) {
        self.banks[self.current_bank as usize][address as usize] = value;
    }

    fn get_byte(&self, address: u16) -> u8 {
        self.banks[self.current_bank as usize][address as usize]
    }
}

impl Index<u16> for BankableRam {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        if !self.enabled {
            return &0;
        }
        &(self.banks[self.current_bank as usize][index as usize])
    }
}

impl IndexMut<u16> for BankableRam {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut (self.banks[self.current_bank as usize][index as usize])
    }
}
