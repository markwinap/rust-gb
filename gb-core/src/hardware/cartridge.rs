use crate::memory::nmmu::Memory;
use std::sync::Arc;

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
        panic!("Write not allowed for read only rom")
    }

    fn get_byte(&self, address: u16) -> Option<u8> {
        let result = self.bytes[address as usize];
        Some(result)
    }
}
