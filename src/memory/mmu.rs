use crate::memory::AddressSpace;

pub struct Mmu {
    spaces: Vec<Box<dyn AddressSpace>>
}

impl AddressSpace for Mmu {
    fn accepts(&self, _: u16) -> bool {
        true
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        if let Some(address_space) = self.get_space_mut(address) {
            address_space.set_byte(address, value)
        }
    }

    fn get_byte(&self, address: u16) -> Option<u8> {
        match self.get_space(address) {
            Some(address_space) => address_space.get_byte(address),
            None => None
        }
    }
}

impl Mmu {

    pub fn new() -> Self {
        Mmu {
            spaces: vec![]
        }
    }

    fn get_space(&self, address: u16) -> Option<&dyn AddressSpace> {
        for i in &self.spaces {
            if (*i).accepts(address) {
                return Some(&**i);
            }
        }
        None
    }
    fn get_space_mut(&mut self, address: u16) -> Option<&mut dyn AddressSpace> {
        for i in &mut self.spaces {
            if (*i).accepts(address) {
                return Some(&mut **i);
            }
        }
        None
    }
}