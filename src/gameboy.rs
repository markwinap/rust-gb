use crate::cpu::address::Cpu;
use crate::hardware::{Screen, Hardware};
use crate::hardware::color_palette::Color;

pub struct GameBoy {
    cpu: Cpu<Hardware<DummyScreen>>,
    elapsed_cycles: usize,
}

impl GameBoy {
    pub fn create() -> Self {
        let hardware = Hardware::create()
    }
}

pub struct DummyScreen {}

impl Screen for DummyScreen {
    fn turn_on(&mut self) {
      //  unimplemented!()
    }

    fn turn_off(&mut self) {
      //  unimplemented!()
    }

    fn set_pixel(&mut self, x: u8, y: u8, color: Color) {
       // unimplemented!()
    }

    fn draw(&mut self) {
        //unimplemented!()
    }
}