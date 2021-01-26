use std::sync::Arc;
use num_traits::FromPrimitive;
use crate::hardware::cartridge::{Cartridge, ReadOnlyMemoryCartridge};

#[derive(FromPrimitive)]
pub enum RomType {
    ROM_ONLY = 0x00,
}

impl RomType {
    pub fn battery(&self) -> bool {
        match self { RomType::ROM_ONLY => { false } }
    }

    pub fn to_cartridge(&self, rom: &Rom) -> Box<dyn Cartridge> {
        match self {
            RomType::ROM_ONLY => Box::new(ReadOnlyMemoryCartridge::from_bytes(rom.data.clone()))
        }
    }
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
            RomSize::_32KB => { 32 * 1024 }
            RomSize::_64KB => { 64 * 1024 }
            RomSize::_128KB => { 128 * 1024 }
            RomSize::_256KB => { 256 * 1024 }
            RomSize::_512KB => { 512 * 1024 }
            RomSize::_1MB => { 1024 * 1024 }
            RomSize::_2MB => { 2048 * 1024 }
        }
    }
}

#[derive(FromPrimitive)]
pub enum RamSize {
    _2KB = 0,
    _8KB = 1,
    _32KB = 2,
    _128KB = 3,
}

impl RamSize {
    pub fn ram_size(&self) -> u32 {
        match self {
            RamSize::_2KB => { 2 * 1024 }
            RamSize::_8KB => { 8 * 1024 }
            RamSize::_32KB => { 32 * 1024 }
            RamSize::_128KB => { 128 * 1024 }
        }
    }

    pub fn banks(&self) -> u8 {
        match self {
            RamSize::_2KB => { 1 }
            RamSize::_8KB => { 1 }
            RamSize::_32KB => { 4 }
            RamSize::_128KB => { 16 }
        }
    }
}

pub enum Model {
    GAME_BOY,
    GAME_BOY_COLOR,
}


impl Model {
    pub fn from_value(value: u8) -> Model {
        match value {
            0x80 => Model::GAME_BOY_COLOR,
            _ => Model::GAME_BOY,
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

pub struct Rom {
    pub data: Arc<[u8]>,
    pub rom_type: RomType,
    pub rom_size: RomSize,
    pub ram_size: RamSize,
    pub model: Model,
    pub region: Region,
    pub title: String,

}

impl Rom {
    pub fn from_bytes(bytes: Arc<[u8]>) -> Self {
        Self {
            data: bytes.clone(),
            rom_type: RomType::from_u8(bytes[0x147]).unwrap(),
            rom_size: RomSize::from_u8(bytes[0x148]).unwrap(),
            ram_size: RamSize::from_u8(bytes[0x149]).unwrap(),
            model: Model::from_value(bytes[0x143]),
            region: Region::from_value(bytes[0x14A]),
            title: Rom::resolve_name(&bytes),
        }
    }

    fn resolve_name(data: &Arc<[u8]>) -> String {
        let new_cartridge = data[0x14b] == 0x33;
        {
            let slice = if new_cartridge {
                &data[0x134..0x13f]
            } else {
                &data[0x134..0x143]
            };
            let utf8 = std::str::from_utf8(slice).unwrap();

            utf8.trim_end_matches('\0').to_string()
        }
    }
}