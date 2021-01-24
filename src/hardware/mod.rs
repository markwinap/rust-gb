use crate::cpu::{Interface, Step};
use crate::cpu::address::Cpu;
use crate::hardware::color_palette::Color;
use crate::hardware::interrupt_handler::{InterruptLine, InterruptHandler};
use crate::hardware::work_ram::WorkRam;
use crate::hardware::timer::Timer;

mod ppu;
mod color_palette;
pub mod interrupt_handler;
mod work_ram;
mod boot_rom;
mod timer;
mod cartridge;

pub const HIRAM_SIZE: usize = 0x80;
pub type HiramData = [u8; HIRAM_SIZE];

pub trait Screen {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn set_pixel(&mut self, x: u8, y: u8, color: Color);
    fn draw(&mut self);
}



struct Hardware {
    interrupt_handler: InterruptHandler,
    work_ram: WorkRam,
    hiram: HiramData,
    timer: Timer,
}

impl Interface for Hardware {

    fn set_interrupt_disabled(&mut self, disabled: bool) {
        self.interrupt_handler.set_interrupt_disabled(disabled);
    }

    // fn enable(&mut self, interrupt: InterruptLine, enable: bool) {
    //     self.interrupt_handler.enable(interrupt,enable);
    // }

    fn request(&mut self, interrupt: InterruptLine, requested: bool) {
        self.interrupt_handler.request(interrupt, requested);
    }

    fn acknowledge(&mut self, interrupt: InterruptLine) {
        self.interrupt_handler.acknowledge(interrupt);
    }

    fn interrupt_master_enabled(&self) -> bool {
        self.interrupt_handler.interrupt_master_enabled
    }

    fn requested_interrupts(&self) -> InterruptLine {
        self.requested_interrupts()
    }

    fn change_interrupt_master_enabled(&mut self, boolean: bool) {
        self.interrupt_handler.interrupt_master_enabled = boolean;
    }

    fn any_enabled(&self) -> bool {
        self.interrupt_handler.any_enabled()
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        unimplemented!()
    }

    fn get_byte(&self, address: u16) -> Option<u8> {
        unimplemented!()
    }

    fn step(&mut self) {
        unimplemented!()
    }
}
