

pub trait Memory {
    fn set_byte(&mut self, address: u16, value: u8);
    fn get_byte(&self, address: u16) -> Option<u8>;
}