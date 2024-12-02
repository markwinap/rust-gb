use crate::cpu::Interface;
use crate::hardware::boot_rom::Bootrom;
use crate::hardware::cartridge::Cartridge;
use crate::hardware::color_palette::Color;
use crate::hardware::input::InputController;
use crate::hardware::interrupt_handler::{InterruptHandler, InterruptLine};
use crate::hardware::ppu::Ppu;
use crate::hardware::timer::Timer;
use crate::hardware::work_ram::WorkRam;
use crate::memory::Memory;

use ppu::{Control, PPuState};
use sound::Sound;

pub mod boot_rom;
pub mod cartridge;
pub mod color_palette;
pub mod input;
pub mod interrupt_handler;
pub mod ppu;
pub mod rom;
pub mod sound;
pub mod timer;
pub mod work_ram;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

pub const HIRAM_SIZE: usize = 0x80;

pub type HiramData = [u8; HIRAM_SIZE];

pub const CPU_FREQ_HZ: usize = 4_194_304;

pub trait Screen {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn set_pixel(&mut self, x: u8, y: u8, color: Color);
    fn scanline_complete(&mut self, _y: u8, _skip: bool) {}
    fn draw(&mut self, skip_next: bool);
    fn frame_rate(&self) -> u8;
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy)]
pub struct Dma {
    source: u8,
}

pub struct Hardware<'a, T: Screen> {
    pub interrupt_handler: InterruptHandler,
    work_ram: WorkRam,
    hiram: HiramData,
    pub timer: Timer,
    pub cartridge: Box<dyn Cartridge + 'a>,
    pub gpu: Ppu<T>,
    pub bootrom: Bootrom,
    dma: Dma,
    pub sound: Sound,
    pub input_controller: InputController,
}

impl<'a, T: Screen> Hardware<'a, T> {
    fn transfer_dma(&mut self, offset: u8) {
        for i in 0..0xFE9F - 0xFE00 + 1 {
            let source = ((offset as u16) << 8) + i;
            let target = 0xFE00 + i;
            let byte = self.get_byte(source);
            self.gpu.write_oam(target as u8, byte);
        }
    }

    pub fn create(
        screen: T,
        cartridge: Box<dyn Cartridge + 'a>,
        boot_rom: Bootrom,
        player: Box<dyn sound::AudioPlayer>,
    ) -> Hardware<'a, T> {
        let ppu: Ppu<T> = Ppu::new(screen);
        Hardware {
            interrupt_handler: InterruptHandler::new(),
            work_ram: WorkRam::new(),
            hiram: [0; HIRAM_SIZE],
            timer: Timer::new(),
            cartridge,
            gpu: ppu,
            bootrom: boot_rom,
            dma: Dma { source: 0 },
            sound: Sound::new_dmg(player),
            input_controller: InputController::new(),
        }
    }

    pub fn create_state(&self) -> HardwareState {
        HardwareState {
            interrupt_handler: self.interrupt_handler,
            work_ram: self.work_ram,
            hiram: self.hiram,
            timer: self.timer,
            dma: self.dma,
        }
    }

    pub fn create_from_state(
        screen: T,
        cartridge: Box<dyn Cartridge + 'a>,
        boot_rom: Bootrom,
        player: Box<dyn sound::AudioPlayer>,
        hardware_state: HardwareState,
        ppu_state: PPuState,
    ) -> Hardware<'a, T> {
        let ppu: Ppu<T> = Ppu::new_from_state(screen, ppu_state);
        Hardware {
            interrupt_handler: hardware_state.interrupt_handler,
            work_ram: hardware_state.work_ram,
            hiram: hardware_state.hiram,
            timer: hardware_state.timer,
            cartridge,
            gpu: ppu,
            bootrom: boot_rom,
            dma: hardware_state.dma,
            sound: Sound::new_dmg(player),
            input_controller: InputController::new(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HardwareState {
    pub interrupt_handler: InterruptHandler,
    pub work_ram: WorkRam,
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub hiram: HiramData,
    pub timer: Timer,
    pub dma: Dma,
}

impl<'a, T: Screen> Interface for Hardware<'a, T> {
    fn gpu_screen_on(&self) -> bool {
        self.gpu.control.contains(Control::LCD_ON)
    }
    fn scan_line(&self) -> u8 {
        self.gpu.scanline
    }
    fn set_interrupt_disabled(&mut self, disabled: bool) {
        self.interrupt_handler.set_interrupt_disabled(disabled);
    }

    fn reset(&mut self) {
        self.interrupt_handler.requested_interrupts = InterruptLine::empty();
        self.interrupt_handler.enabled_interrupts = InterruptLine::empty();
    }

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
        self.interrupt_handler.requested_interrupts
    }

    fn change_interrupt_master_enabled(&mut self, boolean: bool) {
        self.interrupt_handler.interrupt_master_enabled = boolean;
    }

    fn any_enabled(&self) -> bool {
        self.interrupt_handler.any_enabled()
    }
    #[inline(always)] //IMPORTANT
    fn set_byte(&mut self, address: u16, value: u8) {
        match (address >> 8) as u8 {
            0x00 if self.bootrom.is_active() => {}
            0x00..=0x7f => self.cartridge.write_rom(address, value),
            0x80..=0x9f => self.gpu.get_memory_as_mut().set_byte(address, value),
            0xa0..=0xbf => self.cartridge.write_ram(address, value),
            0xc0..=0xfd => self.work_ram.write(address, value),

            0xfe => match address & 0xff {
                0x00..=0x9f => {
                    self.gpu.write_oam(address as u8, value);
                }
                _ => (),
            },
            0xff => match address as u8 {
                0x00 => self.input_controller.write_register(value), //Joypad
                0x01 => (),                                          //Serial
                0x02 => (),                                          //Serial
                0x04..=0x07 => self.timer.set_byte(address, value),
                0x0f => self.interrupt_handler.set_interrupt_flag(value),
                0x10..=0x3f => self.sound.wb(address, value), //APU
                0x40 => self.gpu.set_control(value),
                0x41 => self.gpu.set_stat(value),
                0x42 => self.gpu.set_scroll_y(value),
                0x43 => self.gpu.set_scroll_x(value),
                0x44 => self.gpu.reset_current_line(),
                0x45 => self.gpu.set_compare_line(value),
                0x46 => self.transfer_dma(value),
                0x47 => self.gpu.set_bg_palette(value),
                0x48 => self.gpu.set_obj_palette0(value),
                0x49 => self.gpu.set_obj_palette1(value),
                0x4a => self.gpu.set_window_y(value),
                0x4b => self.gpu.set_window_x(value),
                0x50 => {
                    if self.bootrom.is_active() && value & 0b1 != 0 {
                        self.bootrom.deactivate();
                    }
                }
                0x80..=0xfe => self.hiram[(address as usize) & 0x7f] = value,
                0xff => self.interrupt_handler.set_enabled_interrupts_flag(value),
                _ => (),
            },
        }
    }
    #[inline(always)] //IMPORTANT
    fn get_byte(&mut self, address: u16) -> u8 {
        match (address >> 8) as u8 {
            0x00 if self.bootrom.is_active() => self.bootrom[address],
            0x00..=0x7f => self.cartridge.read_rom(address),

            0x80..=0x9f => self.gpu.read_memory(address),

            0xa0..=0xbf => self.cartridge.read_ram(address),
            0xc0..=0xfd => self.work_ram.read(address),

            0xfe => match address & 0xff {
                0x00..=0x9f => self.gpu.read_oam(address as u8),
                _ => 0,
            },
            0xff => {
                match address as u8 {
                    0x00 => self.input_controller.read_register(), //Joypad
                    0x01 => 0b0,                                   //Serial
                    0x02 => 0b0,                                   //Serial
                    0x04..=0x07 => self.timer.get_byte(address),

                    0x0f => self.interrupt_handler.get_interrupt_flag(),

                    0x10..=0x3f => self.sound.rb(address),
                    0x40 => self.gpu.get_control(),
                    0x41 => self.gpu.get_stat(),
                    0x42 => self.gpu.get_scroll_y(),
                    0x43 => self.gpu.get_scroll_x(),
                    0x44 => self.gpu.get_current_line(),
                    0x45 => self.gpu.get_compare_line(),
                    0x46 => self.dma.source,
                    0x47 => self.gpu.get_bg_palette(),
                    0x48 => self.gpu.get_obj_palette0(),
                    0x49 => self.gpu.get_obj_palette1(),

                    0x4a => self.gpu.get_window_y(),
                    0x4b => self.gpu.get_window_x(),
                    0x80..=0xfe => self.hiram[(address as usize) & 0x7f],
                    0xff => self.interrupt_handler.get_enabled_interrupts_flag(),
                    _ => 0xff,
                }
            }
        }
    }
}
