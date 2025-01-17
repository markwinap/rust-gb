use crate::memory::Memory;

#[cfg(not(feature = "std"))]
use alloc::vec::{self, Vec};

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

use super::rom::{Rom, RomManager};

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
                .read_from_offset(0x4000 as usize, (address - 0x4000) as usize, 1);
        }
        return self
            .bytes
            .read_from_offset(0x0000 as usize, address as usize, 0);
    }
}

#[derive(Eq, PartialEq)]
enum MemoryMode {
    _16MBitRom8KByteRam,
    _4MBitRom32KByteRam,
}

pub struct Mbc1Cartridge<RM: RomManager> {
    rom_manager: RM,
    current_ram_bank: u8,
    number_of_bank_rams: u8,
    ram_enabled: bool,
    ram_banks: Box<[[u8; 0xBFFF - 0xA000 + 1]]>,
    current_rom_bank: u8,
    rom_banks: u8,
    mode: MemoryMode,
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

    pub fn new(rom_manager: RM, ram_banks: u8, rom_banks: u8) -> Self {
        Self {
            rom_manager,
            ram_banks: (0..ram_banks).map(|_| [0; 0xBFFF - 0xA000 + 1]).collect(),
            ram_enabled: false,
            current_ram_bank: 0,
            number_of_bank_rams: ram_banks,
            current_rom_bank: 1,
            mode: MemoryMode::_16MBitRom8KByteRam,
            rom_banks: rom_banks,
        }
    }
}

impl<RM: RomManager> Cartridge for Mbc1Cartridge<RM> {
    fn read_rom(&self, address: u16) -> u8 {
        if Self::compare(address, 0x4000, 0x7FFF) == 0 {
            let bank = if address < 0x4000 {
                if self.mode == MemoryMode::_16MBitRom8KByteRam {
                    0
                } else {
                    self.current_rom_bank & 0xE0
                }
            } else {
                self.current_rom_bank
            };

            let bank_offset = ((bank as usize - 1) * (0x7FFF - 0x4000 + 1)) as usize + 0x4000;
            let result = self.rom_manager.read_from_offset(
                bank_offset,
                (address - 0x4000) as usize,
                self.current_rom_bank,
            );
            return result;
        }
        let result = self
            .rom_manager
            .read_from_offset(0x0000, address as usize, 0);
        result
    }

    fn write_rom(&mut self, address: u16, mut data: u8) {
        if address < 0x2000 {
            self.ram_enabled = data & 0xF == 0xA;
        } else if address < 0x4000 {
            let lower_bits = match (data as usize) & 0x1F {
                0 => 1,
                n => n,
            };
            self.current_rom_bank = (((self.current_rom_bank as usize & 0x60) | lower_bits)
                % self.rom_banks as usize) as u8;
        } else if address < 0x6000 {
            if self.current_rom_bank > 0x20 {
                let upper_bits = (data as u8 & 0x03) % (self.rom_banks >> 5);
                self.current_rom_bank = self.current_rom_bank & 0x1F | (upper_bits << 5)
            }
            if self.number_of_bank_rams > 1 {
                self.current_ram_bank = (data as u8) & 0x03;
            }
        } else if address < 0x8000 {
            data = data & 0b0000_0001;
            if data == 1 {
                self.mode = MemoryMode::_4MBitRom32KByteRam;
            } else {
                self.mode = MemoryMode::_16MBitRom8KByteRam
            }
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        let rambank = if self.mode == MemoryMode::_4MBitRom32KByteRam {
            self.current_ram_bank
        } else {
            0
        };

        self.ram_banks[rambank as usize][address as usize - 0xA000 as usize]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled || self.ram_banks.len() == 0 {
            return;
        }
        let rambank = if self.mode == MemoryMode::_4MBitRom32KByteRam {
            self.current_ram_bank
        } else {
            0
        };
        self.ram_banks[rambank as usize][address as usize - 0xA000 as usize] = value;
    }
}

pub struct Mbc3Cartridge<RM: RomManager> {
    rom_manager: Rom<RM>,
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

    pub fn new(rom_manager: Rom<RM>) -> Self {
        let epoch = rom_manager.data.clock() / 1_000_000;
        let banks = rom_manager.ram_size.banks();
        let mut cartridge = Self {
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
        };
        for (index, bank) in &mut cartridge.ram_banks.iter_mut().enumerate() {
            cartridge.rom_manager.data.load_to_bank(
                &cartridge.rom_manager.title,
                index as u8,
                bank,
            );
        }
        cartridge
    }

    fn update_epoch(&mut self) {
        self.epoch = self.epoch();
    }

    fn epoch(&self) -> u64 {
        self.rom_manager.data.clock() / 1_000_000
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
            let current_state = self.ram_rtc_enabled;
            self.ram_rtc_enabled = (data & 0b0000_1010) != 0;
            if !self.ram_rtc_enabled
                && current_state
                && (self.current_bank_or_rtc == 0x00
                    || self.current_bank_or_rtc == 0x01
                    || self.current_bank_or_rtc == 0x02
                    || self.current_bank_or_rtc == 0x03)
            {
                //SAVE ROM
                self.rom_manager.data.save(
                    &self.rom_manager.title,
                    self.current_bank_or_rtc,
                    &self.ram_banks[self.current_bank_or_rtc as usize],
                )
            }
        } else if address < 0x4000 {
            self.current_rom_bank = (data & 0x7f).max(1);
        } else if address < 0x6000 {
            self.current_bank_or_rtc = data;
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
            let result = self.rom_manager.data.read_from_offset(
                bank_offset,
                (address - 0x4000) as usize,
                self.current_rom_bank,
            );
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
            self.rom_manager
                .data
                .read_from_offset(0x0000, address as usize, 0)
        }
    }
}
