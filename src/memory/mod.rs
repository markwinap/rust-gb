
pub mod mmu;
pub trait AddressSpace {

    fn accepts(&self, address: u16) -> bool;
    fn set_byte(&mut self, address: u16, value: u8);
    fn get_byte(&self, address: u16) -> Option<u8>;
}
