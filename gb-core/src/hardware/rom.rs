use alloc::boxed::Box;
use num_traits::FromPrimitive;
use crate::hardware::cartridge::{Cartridge, ReadOnlyMemoryCartridge, Mbc1Cartridge, BankableRam};
use alloc::vec::Vec;
use alloc::string::{String, ToString};

#[derive(FromPrimitive, Clone, Copy)]
pub enum RomType {
    RomOnly = 0x00,
    MBC1 = 0x01,
}

impl RomType {
    pub fn battery(&self) -> bool {
        match self {
            RomType::RomOnly => { false }
            RomType::MBC1 => { false }
            _ => { false }
        }
    }

    pub fn to_cartridge<'a>(self, rom: &Rom<'a>) -> Box<dyn Cartridge + 'a > {
        match self {
            RomType::RomOnly => Box::new(ReadOnlyMemoryCartridge::from_bytes(rom.data)),
            RomType::MBC1 => Box::new(Mbc1Cartridge::new(rom.data, BankableRam::new(rom.ram_size.banks()))),
            _ => panic!()
          //  RomType::MBC1 => Box::new(Mbc1Cartridge::new(rom.data, BankableRam::new(rom.ram_size.banks())))
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

pub struct Rom<'a> {
    pub data: &'a [u8],
    pub rom_type: RomType,
    pub rom_size: RomSize,
    pub ram_size: RamSize,
    pub model: Model,
    pub region: Region,
    pub title: String,

}

impl <'a> Rom<'a> {

    pub fn into_cartridge(&self) -> Box<dyn Cartridge + 'a> {
        let rom_type = self.rom_type.clone();
        rom_type.to_cartridge(self)
    }

    pub fn from_bytes(bytes: &'static [u8]) -> Self {
        let rom_t = bytes[0x147];
       
        let rom_size = RomSize::from_u8(bytes[0x148]).unwrap();
        let ram_size = RamSize::from_u8(bytes[0x149]).unwrap();
        //let ram_size = RamSize::_32KB;
        let model = Model::from_value(bytes[0x143]);
        let region = Region::from_value(bytes[0x14A]);
        let title = Rom::resolve_name(bytes);
        let rom_type = RomType::from_u8(bytes[0x147]).unwrap();



        Self {
            data: bytes,
            rom_type: rom_type,
            rom_size: rom_size,
            ram_size: ram_size,
            model: model,
            
            region: region,
            title: "sds".to_string(),
        }
    }

    fn resolve_name(data: &'a [u8]) -> String {
        let new_cartridge = data[0x14b] == 0x33;
        {
            let slice = if new_cartridge {
                &data[0x134..0x13f]
            } else {
                &data[0x134..0x143]
            };
            let utf8 = alloc::str::from_utf8(slice).unwrap();

            utf8.trim_end_matches('\0').to_string()
        }
    }
}