use crate::memory::nmmu::Memory;
use std::sync::Arc;
use bitflags::_core::ops::{Index, IndexMut};

pub trait Cartridge: Memory {
    fn step(&mut self) {}
}

pub struct ReadOnlyMemoryCartridge {
    bytes: Arc<Vec<u8>>,
}

impl Cartridge for ReadOnlyMemoryCartridge {}

impl ReadOnlyMemoryCartridge {
    pub fn from_bytes(bytes: Arc<Vec<u8>>) -> Self {
        Self {
            bytes
        }
    }
}

impl Memory for ReadOnlyMemoryCartridge {
    fn set_byte(&mut self, _: u16, _: u8) {
        // panic!("Write not allowed for read only rom")
    }

    fn get_byte(&self, address: u16) -> Option<u8> {
        let result = self.bytes[address as usize];
        Some(result)
    }
}
/////////////////////

#[derive(Eq, PartialEq)]
enum MemoryMode {
    MBIT_16_ROM_8KBYTE_RAM,
    MBIT_4_ROM_32KBYTE_RAM,
}

pub struct Mbc1Cartridge {
    bytes: Arc<Vec<u8>>,
    bank_ram: BankableRam,
    current_rom_bank: u8,
    mode: MemoryMode,

}

impl Cartridge for Mbc1Cartridge {}

impl Memory for Mbc1Cartridge {
    fn set_byte(&mut self, address: u16, mut data: u8) {
        if address < 0x2000 {
            self.bank_ram.enable((data & 0b0000_1010) != 0);
        } else if address < 0x4000 {
            self.current_rom_bank = (data & 0b0001_1111);
            if self.current_rom_bank == 0 || self.current_rom_bank == 0x20 || self.current_rom_bank == 0x40 || self.current_rom_bank == 0x60 {
                self.current_rom_bank += 1;
            }
        } else if address < 0x6000 {
            if self.mode == MemoryMode::MBIT_4_ROM_32KBYTE_RAM {
                self.bank_ram.select_bank(data & 0b0000_0011);
            } else {
                data = (data & 0b0000_0011) << 5;
                self.current_rom_bank = (self.current_rom_bank & 0b0001_1111) + data;
            }
        } else if address < 0x8000 {
            data = data & 0b0000_0001;
            if data == 1 {
                self.mode = MemoryMode::MBIT_4_ROM_32KBYTE_RAM;
            } else {
                self.mode = MemoryMode::MBIT_16_ROM_8KBYTE_RAM
            }
        } else if Mbc1Cartridge::compare(address, 0xA000, 0xBFFF) == 0 {
            //self.bank_ram.set_byte(address - 0xA000, data);
            self.bank_ram[address - 0xA000] = data;
        }
    }

    fn get_byte(&self, address: u16) -> Option<u8> {
        if Mbc1Cartridge::compare(address, 0x4000, 0x7FFF) == 0 {
            return Some(self.bytes[(address + ((self.current_rom_bank as u16 - 1) * (0x7FFF - 0x4000 + 1))) as usize]);
        } else if Mbc1Cartridge::compare(address, 0xA000, 0xBFFF) == 0 {
            //return Some( self.bank_ram[address - 0xA000]);
            return self.bank_ram.get_byte(address - 0xA000);
        }
        Some(self.bytes[address as usize])
    }


    //      public int compareTo(int value) {
    //             if (value < from) {
    //                 return value - from;
    //             } else if (value > to) {
    //                 return value - to;
    //             }
    //             return 0;
    //         }
}

impl Mbc1Cartridge {
    pub fn compare(value: u16, from: u16, to: u16) -> isize {
        if value < from {
            return value as isize - from as isize;
        } else if value > to {
            return value as isize - to as isize;
        }
        return 0;
    }

    pub fn new(bytes: Arc<Vec<u8>>, bank_ram: BankableRam) -> Self {
        Self {
            bytes,
            bank_ram,
            current_rom_bank: 1,
            mode: MemoryMode::MBIT_16_ROM_8KBYTE_RAM,
        }
    }
}

//0xA000, 0xBFFF
pub struct BankableRam {
    banks: Vec<[u8; 0xBFFF - 0xA000 + 1]>,
    current_bank: u8,
    enabled: bool,
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

    fn get_byte(&self, address: u16) -> Option<u8> {
        Some(self.banks[self.current_bank as usize][address as usize])
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