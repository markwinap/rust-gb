use std::sync::Arc;

pub enum RomType {
    ROM_ONLY = 0x00,
}

impl RomType {
    pub fn battery(&self) -> bool {
        match self { RomType::ROM_ONLY => { false } }
    }
}

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

pub struct Cartridge {
    pub data: Arc<[u8]>,
    pub rom_type: RomType,
    pub rom_size: RomSize,
    pub ram_size: RamSize,
    pub model: Model,
    pub region: Region,
    pub title: String,

}