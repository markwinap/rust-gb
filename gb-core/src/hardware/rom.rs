use core::ops::Index;
use core::str::FromStr;

use crate::hardware::cartridge::{Cartridge, Mbc1Cartridge, ReadOnlyMemoryCartridge};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use super::cartridge::Mbc3Cartridge;

#[cfg(feature = "std")]
use std as core;

#[cfg(not(feature = "std"))]
use alloc::string::String;

#[cfg(not(feature = "std"))]
use alloc::format;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

#[derive(FromPrimitive, Clone, Copy)]
pub enum RomType {
    RomOnly = 0x00,
    MBC1 = 0x01,
    MBC1RamBattery = 0x03,
    MBC3 = 0x13,
}

fn rom_banks(v: u8) -> u8 {
    if v <= 8 {
        2 << v
    } else {
        0
    }
}

impl RomType {
    pub fn battery(&self) -> bool {
        match self {
            RomType::RomOnly => false,
            RomType::MBC1 => false,
            RomType::MBC3 => true,
            RomType::MBC1RamBattery => true,
        }
    }

    pub fn to_cartridge<'a, RM: RomManager + 'a>(self, rom: Rom<RM>) -> Box<dyn Cartridge + 'a> {
        match self {
            RomType::RomOnly => Box::new(ReadOnlyMemoryCartridge::from_bytes(rom.data)),
            RomType::MBC1 => Box::new(Mbc1Cartridge::new(
                rom.data,
                rom.ram_size.banks(),
                rom_banks(rom.rom_size as u8),
            )),
            RomType::MBC1RamBattery => Box::new(Mbc1Cartridge::new(
                rom.data,
                rom.ram_size.banks(),
                rom_banks(rom.rom_size as u8),
            )),
            RomType::MBC3 => Box::new(Mbc3Cartridge::new(rom)),
        }
    }
}

pub trait RomManager:
    Index<usize, Output = u8> + Index<core::ops::Range<usize>, Output = [u8]>
{
    fn read_from_offset(&self, seek_offset: usize, index: usize, bank_number: u8) -> u8;

    fn clock(&self) -> u64;

    fn save(&mut self, game_title: &str, bank_index: u8, bank: &[u8]);

    fn load_to_bank(&mut self, game_title: &str, bank_index: u8, bank: &mut [u8]);
}

#[derive(FromPrimitive)]
pub enum RomSize {
    _32KB = 0,
    _64KB = 1,
    _128KB = 2,
    _256KB = 3,
    _512KB = 4,
    _1MB = 5,
    _2MB = 6,
}

impl RomSize {
    pub fn expected_size(&mut self) -> u32 {
        match self {
            RomSize::_32KB => 32 * 1024,
            RomSize::_64KB => 64 * 1024,
            RomSize::_128KB => 128 * 1024,
            RomSize::_256KB => 256 * 1024,
            RomSize::_512KB => 512 * 1024,
            RomSize::_1MB => 1024 * 1024,
            RomSize::_2MB => 2048 * 1024,
        }
    }
}

#[derive(FromPrimitive)]
pub enum RamSize {
    NoRam = 0,
    _2KB = 0x01,
    _8KB = 0x02,
    _32KB = 0x03,
    _128KB = 0x04,
}

impl RamSize {
    pub fn ram_size(&self) -> u32 {
        match self {
            RamSize::NoRam => 0 * 1024,
            RamSize::_2KB => 2 * 1024,
            RamSize::_8KB => 8 * 1024,
            RamSize::_32KB => 32 * 1024,
            RamSize::_128KB => 128 * 1024,
        }
    }

    pub fn banks(&self) -> u8 {
        match self {
            RamSize::NoRam => 0,
            RamSize::_2KB => 1,
            RamSize::_8KB => 1,
            RamSize::_32KB => 4,
            RamSize::_128KB => 16,
        }
    }
}

pub enum Model {
    GameBoy,
    GameBoyColor,
}

impl Model {
    pub fn from_value(value: u8) -> Model {
        match value {
            0x80 => Model::GameBoyColor,
            _ => Model::GameBoy,
        }
    }
}

pub enum Region {
    JAPAN,
    INTERNATIONAL,
}

impl Region {
    pub fn from_value(value: u8) -> Region {
        match value {
            0 => Region::JAPAN,
            _ => Region::INTERNATIONAL,
        }
    }
}

pub struct Rom<RM: RomManager> {
    pub data: RM,
    pub rom_type: RomType,
    pub rom_size: RomSize,
    pub ram_size: RamSize,
    pub model: Model,
    pub region: Region,
    pub title: String,
}

impl<'a, RM: RomManager + 'a> Rom<RM> {
    pub fn into_cartridge(self) -> Box<dyn Cartridge + 'a> {
        let rom_type = self.rom_type.clone();
        rom_type.to_cartridge(self)
    }

    pub fn from_bytes(bytes: RM) -> Self {
        let rom_size = RomSize::from_u8(bytes[0x148]).unwrap();
        let ram_size = RamSize::from_u8(bytes[0x149]).unwrap();
        let model = Model::from_value(bytes[0x143]);
        let region = Region::from_value(bytes[0x14A]);
        let title = Rom::resolve_name(&bytes);
        let rom_type = RomType::from_u8(bytes[0x147])
            .expect(&format!("Rom type not matched: {}", bytes[0x147]));
        Self {
            data: bytes,
            rom_type: rom_type,
            rom_size: rom_size,
            ram_size: ram_size,
            model: model,
            region: region,
            title: title,
        }
    }

    fn resolve_name(data: &RM) -> String {
        let new_cartridge = data[0x14b] == 0x33;
        {
            let slice = if new_cartridge {
                &data[0x134..0x13f]
            } else {
                &data[0x134..0x143]
            };
            let utf8 = core::str::from_utf8(slice).unwrap();

            String::from_str(utf8.trim_end_matches('\0')).unwrap()
        }
    }
}
