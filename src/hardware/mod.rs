use crate::cpu::{Interface, Step};
use crate::cpu::address::Cpu;
use crate::hardware::color_palette::Color;
use crate::hardware::interrupt_handler::{InterruptLine, InterruptHandler};
use crate::hardware::work_ram::WorkRam;
use crate::hardware::timer::Timer;
use crate::hardware::cartridge::Cartridge;
use crate::hardware::boot_rom::{BootromData, Bootrom};
use crate::hardware::ppu::Ppu;
use crate::memory::nmmu::Memory;

mod ppu;
mod color_palette;
pub mod interrupt_handler;
mod work_ram;
mod boot_rom;
mod timer;
mod cartridge;
mod rom;

pub const HIRAM_SIZE: usize = 0x80;

pub type HiramData = [u8; HIRAM_SIZE];

pub trait Screen {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn set_pixel(&mut self, x: u8, y: u8, color: Color);
    fn draw(&mut self);
}


struct Hardware<T: Screen> {
    interrupt_handler: InterruptHandler,
    work_ram: WorkRam,
    hiram: HiramData,
    timer: Timer,
    cartridge: Box<dyn Cartridge>,
    gpu: Ppu<T>,
    bootrom: Bootrom,
}

impl <T: Screen> Interface for Hardware<T> {
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
        let result = match (address >> 8) as u8 {
            0x0000 if self.bootrom.is_active() => Some(self.bootrom[address]),
            0x00..=0x3f => self.cartridge.get_byte(address),
            0x40..=0x7f => self.cartridge.get_byte(address),
            0x80..=0x97 => self.gpu.read_memory(address),
            0x98..=0x9b => self.gpu.read_memory(address),
            0x9c..=0x9f => self.gpu.read_memory(address),
            0xa0..=0xbf => self.cartridge.get_byte(address),
            0xc0..=0xcf => Some(self.work_ram.read_lower(address)),
            0xd0..=0xdf => Some(self.work_ram.read_upper(address)),

            0xe0..=0xef => Some(self.work_ram.read_lower(address)),
            0xf0..=0xfd => Some(self.work_ram.read_upper(address)),
            0xff => {
                match address {
                    0x00 => Some(0b0), //Joypad
                    0x01 => Some(0b0), //Serial
                    0x02 => Some(0b0), //Serial
                    0x04 => self.timer.get_byte(address),
                    0x05 => self.timer.get_byte(address),
                    0x06 => self.timer.get_byte(address),
                    0x07 => self.timer.get_byte(address),
                    0x0f => Some(self.interrupt_handler.get_interrupt_flag()),
                    0x10..=0x3f => Some(0b0), //Audio

                    0x40 => Some(self.gpu.get_control()),
                    0x41 => Some(self.gpu.get_stat()),
                    0x42 => Some(self.gpu.get_scroll_y()),
                    0x43 => Some(self.gpu.get_scroll_x()),
                    0x44 => Some(self.gpu.get_current_line()),
                    0x45 => Some(self.gpu.get_compare_line()),
                    0x46 => Some(0b0), //TODO OAM
                    0x47 => Some(self.gpu.get_bg_palette()),
                    0x48 => Some(self.gpu.get_obj_palette0()),
                    0x49 => Some(self.gpu.get_obj_palette1()),

                    0x4a => Some(self.gpu.get_window_y()),
                    0x4b => Some(self.gpu.get_window_x()),
                    0x80..=0xfe => Some(self.hiram[(address as usize) & 0x7f]),
                    0xff => Some(self.interrupt_handler.get_enabled_interrupts_flag()),
                    _ => Some(0xff)
                }

            }

            _ => None
        };

        None
    }

    fn step(&mut self) {
        unimplemented!()
    }
}
