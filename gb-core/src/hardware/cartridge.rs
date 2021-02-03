use crate::memory::nmmu::Memory;
use std::sync::Arc;
use bitflags::_core::ops::{Index, IndexMut};

pub trait Cartridge {
    fn step(&mut self) {}

    fn read_rom(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);

    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);
}

pub struct ReadOnlyMemoryCartridge {
    bytes: Arc<Vec<u8>>,
}

impl Cartridge for ReadOnlyMemoryCartridge {
    fn read_rom(&self, address: u16) -> u8 {
        self.get_byte(address).unwrap()
    }

    fn write_rom(&mut self, address: u16, value: u8) {}

    fn read_ram(&self, address: u16) -> u8 {
        self.get_byte(address).unwrap()
    }

    fn write_ram(&mut self, address: u16, value: u8) {}
}

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
    ram_mode: bool,

}

impl Cartridge for Mbc1Cartridge {
    fn read_rom(&self, address: u16) -> u8 {
        // self.get_byte(address).unwrap()
        //  let idx = if address < 0x4000 { address as usize } else { self.current_rom_bank * 0x4000 | ((address as usize) & 0x3FFF) };
        //*self.bytes.get(idx).unwrap_or(&0)
        0
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        self.set_byte(address, value)
    }

    fn read_ram(&self, address: u16) -> u8 {
        if self.bank_ram.enabled {
            return 0;
        }
        let rambank = if self.ram_mode { self.current_rom_bank } else { 0 };
        //   self.bank_ram.
        0
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.set_byte(address, value)
    }
}

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
            ram_mode: false,
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




pub struct MBC1 {
    rom: Arc<Vec<u8>>,
    ram: Vec<u8>,
    ram_on: bool,
    ram_mode: bool,
    rombank: usize,
    rambank: usize,
}

impl MBC1 {
    pub fn new(data: Arc<Vec<u8>>) -> MBC1 {
        let (ramsize) = match data[0x147] {
            0x02 => (ram_size(data[0x149])),
            0x03 => (ram_size(data[0x149])),
            _ => (0),
        };

        let mut res = MBC1 {
            rom: data,
            ram: ::std::iter::repeat(0u8).take(ramsize).collect(),
            ram_on: false,
            ram_mode: false,
            rombank: 1,
            rambank: 0,

        };
        res
    }
}

fn ram_size(v: u8) -> usize {
    match v {
        1 => 0x800,
        2 => 0x2000,
        3 => 0x8000,
        4 => 0x20000,
        _ => 0,
    }
}

impl Cartridge for MBC1 {
    fn read_rom(&self, a: u16) -> u8 {
        let idx = if a < 0x4000 { a as usize } else { self.rombank * 0x4000 | ((a as usize) & 0x3FFF) };
        *self.rom.get(idx).unwrap_or(&0)
    }
    fn read_ram(&self, a: u16) -> u8 {
        if !self.ram_on { return 0; }
        let rambank = if self.ram_mode { self.rambank } else { 0 };
        self.ram[(rambank * 0x2000) | ((a & 0x1FFF) as usize)]
    }

    fn write_rom(&mut self, a: u16, v: u8) {
        match a {
            0x0000..=0x1FFF => { self.ram_on = v == 0x0A; }
            0x2000..=0x3FFF => {
                self.rombank = (self.rombank & 0x60) | match (v as usize) & 0x1F {
                    0 => 1,
                    n => n
                }
            }
            0x4000..=0x5FFF => {
                if !self.ram_mode {
                    self.rombank = self.rombank & 0x1F | (((v as usize) & 0x03) << 5)
                } else {
                    self.rambank = (v as usize) & 0x03;
                }
            }
            0x6000..=0x7FFF => { self.ram_mode = (v & 0x01) == 0x01; }
            _ => panic!("Could not write to {:04X} (MBC1)", a),
        }
    }

    fn write_ram(&mut self, a: u16, v: u8) {
        if !self.ram_on { return; }
        let rambank = if self.ram_mode { self.rambank } else { 0 };
        self.ram[(rambank * 0x2000) | ((a & 0x1FFF) as usize)] = v;
    }
}
