use core::ops::{Index, IndexMut};

use crate::memory::Memory;
use alloc::boxed::Box;
use log::info;

use super::rom::RomManager;

pub trait Cartridge {
    fn step(&mut self) {}

    fn read_rom(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);

    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);
}

pub struct ReadOnlyMemoryCartridge<RM: RomManager> {
    bytes: RM,
}

impl<RM: RomManager> Cartridge for ReadOnlyMemoryCartridge<RM> {
    fn read_rom(&self, address: u16) -> u8 {
        self.get_byte(address)
    }

    fn write_rom(&mut self, _: u16, _: u8) {}

    fn read_ram(&self, address: u16) -> u8 {
        self.get_byte(address)
    }

    fn write_ram(&mut self, _: u16, _: u8) {}
}

impl<RM: RomManager> ReadOnlyMemoryCartridge<RM> {
    pub fn from_bytes(bytes: RM) -> Self {
        Self { bytes }
    }
    #[inline(always)]
    pub fn compare(value: u16, from: u16, to: u16) -> isize {
        if value < from {
            return value as isize - from as isize;
        } else if value > to {
            return value as isize - to as isize;
        }
        return 0;
    }
}

impl<RM: RomManager> Memory for ReadOnlyMemoryCartridge<RM> {
    fn set_byte(&mut self, _: u16, _: u8) {}

    fn get_byte(&self, address: u16) -> u8 {
        if Self::compare(address, 0x4000, 0x7FFF) == 0 {
            return self
                .bytes
                .read_from_offset(0x4000 as usize, (address - 0x4000) as usize);
        }
        return self
            .bytes
            .read_from_offset(0x0000 as usize, address as usize);
    }
}

#[derive(Eq, PartialEq)]
enum MemoryMode {
    MBit16Rom8KByteRam,
    MBit4Rom32KByteRam,
}

pub struct Mbc1Cartridge<RM: RomManager> {
    rom_manager: RM,
    bank_ram: BankableRam,
    current_rom_bank: u8,
    mode: MemoryMode,
}

impl<RM: RomManager> Cartridge for Mbc1Cartridge<RM> {
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

impl<RM: RomManager> Memory for Mbc1Cartridge<RM> {
    fn set_byte(&mut self, address: u16, mut data: u8) {
        if address < 0x2000 {
            self.bank_ram.enable((data & 0b0000_1010) != 0);
        } else if address < 0x4000 {
            self.current_rom_bank = data & 0b0001_1111;
            if self.current_rom_bank == 0
                || self.current_rom_bank == 0x20
                || self.current_rom_bank == 0x40
                || self.current_rom_bank == 0x60
            {
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
        } else if Self::compare(address, 0xA000, 0xBFFF) == 0 {
            self.bank_ram[address - 0xA000] = data;
        }
    }

    fn get_byte(&self, address: u16) -> u8 {
        if Self::compare(address, 0x4000, 0x7FFF) == 0 {
            let bank_offset =
                ((self.current_rom_bank as u16 - 1) * (0x7FFF - 0x4000 + 1)) as usize + 0x4000;
            let result = self
                .rom_manager
                .read_from_offset(bank_offset, (address - 0x4000) as usize);
            return result;
        } else if Self::compare(address, 0xA000, 0xBFFF) == 0 {
            return self.bank_ram.get_byte(address - 0xA000);
        }
        self.rom_manager.read_from_offset(0x0000, address as usize)
    }
}

impl<RM: RomManager> Mbc1Cartridge<RM> {
    #[inline(always)]
    pub fn compare(value: u16, from: u16, to: u16) -> isize {
        if value < from {
            return value as isize - from as isize;
        } else if value > to {
            return value as isize - to as isize;
        }
        return 0;
    }

    pub fn new(rom_manager: RM, bank_ram: BankableRam) -> Self {
        Self {
            rom_manager,
            bank_ram,
            current_rom_bank: 1,
            mode: MemoryMode::MBit16Rom8KByteRam,
        }
    }
}

pub struct BankableRam {
    current_bank: u8,
    enabled: bool,
    banks: Box<[[u8; 0xBFFF - 0xA000 + 1]]>,
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

pub struct Mbc3Cartridge<RM: RomManager> {
    rom_manager: RM,
    current_bank_or_rtc: u8,
    ram_rtc_enabled: bool,
    ram_banks: Box<[[u8; 0xBFFF - 0xA000 + 1]]>,
    current_rom_bank: u8,

    //
    rtc_secs: u8,
    rtc_mins: u8,
    rtc_hours: u8,
    rtc_day_low: u8,
    rtc_day_high: u8,
    epoch: u64,
    prelatch: bool,
}

impl<RM: RomManager> Mbc3Cartridge<RM> {
    #[inline(always)]
    pub fn compare(value: u16, from: u16, to: u16) -> isize {
        if value < from {
            return value as isize - from as isize;
        } else if value > to {
            return value as isize - to as isize;
        }
        return 0;
    }

    pub fn new(rom_manager: RM, banks: u8) -> Self {
        let epoch = rom_manager.clock() / 1_000_000;
        Self {
            rom_manager,
            ram_banks: (0..banks).map(|_| [0; 0xBFFF - 0xA000 + 1]).collect(),
            current_bank_or_rtc: 0,
            ram_rtc_enabled: false,
            current_rom_bank: 1,
            rtc_secs: 0,
            rtc_mins: 0,
            rtc_hours: 0,
            rtc_day_low: 0,
            rtc_day_high: 0,
            epoch: epoch,
            prelatch: false,
        }
    }

    fn update_epoch(&mut self) {
        self.epoch = self.epoch();
    }

    fn epoch(&self) -> u64 {
        self.rom_manager.clock() / 1_000_000
    }

    fn day(&self) -> u64 {
        ((self.rtc_day_high as u64 & 1) << 8) & self.rtc_day_low as u64
    }

    fn dhms_to_secs(&self) -> u64 {
        let d = self.day();
        let s = self.rtc_secs as u64;
        let m = self.rtc_mins as u64;
        let h = self.rtc_hours as u64;
        (d * 24 + h) * 3600 + m * 60 + s
    }

    fn secs_to_dhms(&mut self, secs: u64) {
        let s = secs % 60;
        let m = (secs / 60) % 60;
        let h = (secs / 3600) % 24;
        let d = secs / (3600 * 24);
        self.rtc_secs = s as u8;
        self.rtc_mins = m as u8;
        self.rtc_hours = h as u8;
        self.rtc_day_low = d as u8;
        self.rtc_day_high = (self.rtc_day_high & !1) | ((d >> 8) & 1) as u8;
    }

    fn latch(&mut self) {
        let new_epoch = if self.rtc_day_high & 0x40 == 0 {
            self.epoch()
        } else {
            // Halt
            self.epoch
        };
        let elapsed = new_epoch - self.epoch;

        let last_day = self.day();
        let last_secs = self.dhms_to_secs();
        self.secs_to_dhms(last_secs + elapsed);
        let new_day = self.day();

        // Overflow
        if new_day < last_day {
            self.rtc_day_high |= 0x80;
        }

        self.epoch = new_epoch;
    }
}

impl<RM: RomManager> Cartridge for Mbc3Cartridge<RM> {
    fn read_rom(&self, address: u16) -> u8 {
        self.get_byte(address)
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        self.set_byte(address, value)
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_rtc_enabled {
            return 0;
        }
        self.get_byte(address)
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.set_byte(address, value)
    }
}
impl<RM: RomManager> Memory for Mbc3Cartridge<RM> {
    fn set_byte(&mut self, address: u16, data: u8) {
        if address < 0x2000 {
            self.ram_rtc_enabled = (data & 0b0000_1010) != 0;
        } else if address < 0x4000 {
            self.current_rom_bank = data & 0x7f;
            info!("Changing bank: {}", self.current_rom_bank);
        } else if address < 0x6000 {
            self.current_bank_or_rtc = data;
            //TODO SAVE
        } else if address < 0x8000 {
            if self.prelatch {
                if data == 0x01 {
                    self.latch()
                }
                self.prelatch = false;
            } else if data == 0x00 {
                self.prelatch = true;
            }
        } else if Self::compare(address, 0xa000, 0xbfff) == 0 {
            match self.current_bank_or_rtc {
                x if x == 0x00 || x == 0x01 || x == 0x02 || x == 0x03 => {
                    self.ram_banks[x as usize][address as usize - 0xA000] = data
                }
                0x08 => {
                    self.rtc_secs = data;
                    self.update_epoch();
                }
                0x09 => {
                    self.rtc_mins = data;
                    self.update_epoch();
                }
                0x0a => {
                    self.rtc_hours = data;
                    self.update_epoch();
                }
                0x0b => {
                    self.rtc_day_low = data;
                    self.update_epoch();
                }
                0x0c => {
                    self.rtc_day_high = data;
                    self.update_epoch();
                }
                s => unimplemented!("Unknown selector: {:02x}", s),
            };
        }
    }

    fn get_byte(&self, address: u16) -> u8 {
        if Self::compare(address, 0x4000, 0x7FFF) == 0 {
            let bank_offset =
                ((self.current_rom_bank as usize - 1) * (0x7FFF - 0x4000 + 1)) as usize + 0x4000;
            let result = self
                .rom_manager
                .read_from_offset(bank_offset, (address - 0x4000) as usize);
            return result;
        } else if Self::compare(address, 0xa000, 0xbfff) == 0 {
            match self.current_bank_or_rtc {
                x if x == 0x00 || x == 0x01 || x == 0x02 || x == 0x03 => {
                    self.ram_banks[x as usize][address as usize - 0xA000]
                }
                0x08 => self.rtc_secs,
                0x09 => self.rtc_mins,
                0x0a => self.rtc_hours,
                0x0b => self.rtc_day_low,
                0x0c => self.rtc_day_high,
                s => unimplemented!("Unknown selector: {:02x}", s),
            }
        } else {
            self.rom_manager.read_from_offset(0x0000, address as usize)
        }
    }
}
