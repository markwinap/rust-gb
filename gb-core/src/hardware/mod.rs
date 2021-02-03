use crate::cpu::{Interface, Step};
use crate::cpu::address::Cpu;
use crate::hardware::color_palette::Color;
use crate::hardware::interrupt_handler::{InterruptLine, InterruptHandler};
use crate::hardware::work_ram::WorkRam;
use crate::hardware::timer::Timer;
use crate::hardware::cartridge::Cartridge;
use crate::hardware::boot_rom::{BootromData, Bootrom};
use crate::hardware::ppu::Ppu;
use crate::memory::Memory;
use std::time::Duration;
use crate::hardware::input::{InputController, Controller};

pub mod ppu;
pub mod color_palette;
pub mod interrupt_handler;
pub mod work_ram;
pub mod boot_rom;
pub mod timer;
pub mod cartridge;
pub mod rom;
pub mod input;

pub const HIRAM_SIZE: usize = 0x80;

pub type HiramData = [u8; HIRAM_SIZE];

pub const HIRAM_EMPTY: HiramData = [0; HIRAM_SIZE];

pub trait Screen {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn set_pixel(&mut self, x: u8, y: u8, color: Color);
    fn draw(&mut self);
}

struct Dma {
    source: u8,

}

pub struct Hardware<T: Screen> {
    pub interrupt_handler: InterruptHandler,
    work_ram: WorkRam,
    hiram: HiramData,
    pub timer: Timer,
    pub cartridge: Box<dyn Cartridge>,
    pub gpu: Ppu<T>,
    pub bootrom: Bootrom,
    dma: Dma,
    pub input_controller: InputController
}


impl<T: Screen> Hardware<T> {
    fn transfer_dma(&mut self, offset: u8) {
        for i in 0..0xFE9F - 0xFE00 + 1 {
            let source = ((offset as u16) << 8) + i;
            let target = 0xFE00 + i;
            self.gpu.write_oam(target as u8, self.get_byte(source).unwrap());
        }
    }

    fn do_step(&mut self) {}
    pub fn create(screen: T, cartridge: Box<dyn Cartridge>, boot_rom: Bootrom) -> Hardware<T> {
        let ppu: Ppu<T> = Ppu::new(screen);
        Hardware {
            interrupt_handler: InterruptHandler::new(),
            work_ram: WorkRam::new(),
            hiram: HIRAM_EMPTY,
            timer: Timer::new(),
            cartridge,
            gpu: ppu,
            bootrom: boot_rom,
            dma: Dma { source: 0 },
            input_controller: InputController::new(),
        }
    }
}


impl<T: Screen> Interface for Hardware<T> {
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

    fn set_byte(&mut self, address: u16, value: u8) {
        match (address >> 8) as u8 {
            0x00 if self.bootrom.is_active() => {}
            0x00..=0x7f => self.cartridge.write_rom(address, value),
            0x80..=0x97 => self.gpu.get_memory_as_mut().set_byte(address, value),
            0x98..=0x9b => self.gpu.get_memory_as_mut().set_byte(address, value),
            0x9c..=0x9f => self.gpu.get_memory_as_mut().set_byte(address, value),
            0xa0..=0xbf => self.cartridge.write_ram(address, value),
            0xc0..=0xcf => self.work_ram.write_lower(address, value),
            0xd0..=0xdf => self.work_ram.write_upper(address, value),

            0xe0..=0xef => self.work_ram.write_lower(address, value),
            0xf0..=0xfd => self.work_ram.write_upper(address, value),
            0xfe => match address & 0xff {
                0x00..=0x9f => { //TODO OAM
                    // self.generic_mem_cycle(|hw| {
                    //     if !hw.oam_dma.is_active() {
                    //         hw.gpu.write_oam(addr as u8, value)
                    //     }
                    // })
                    self.gpu.write_oam(address as u8, value);
                }
                _ => (),
            },
            0xff => match address as u8 {
                0x00 => self.input_controller.write_register(value), //Joypad
                0x01 => (), //Serial
                0x02 => (), //Serial
                0x04 => self.timer.set_byte(address, value),
                0x05 => self.timer.set_byte(address, value),
                0x06 => self.timer.set_byte(address, value),
                0x07 => self.timer.set_byte(address, value),
                0x0f => self.interrupt_handler.set_interrupt_flag(value),
                0x10..=0x3f => (), //APU
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
                 //   println!("DEACTIVATE BOOT");
                    if self.bootrom.is_active() && value & 0b1 != 0 {
               //         println!("DEACTIVATE BOOT");
                        std::thread::sleep(Duration::from_secs(5));
                        self.bootrom.deactivate();
                    }
                }
                0x80..=0xfe => {
                    // if value == 255 && (((address as usize) & 0x7f) == 0) {
                    //     println!("weird!!");
                    //     std::thread::sleep(Duration::from_secs(3));
                    //     println!("weird done!!");
                    // }
                    self.hiram[(address as usize) & 0x7f] = value
                },
                0xff => self.interrupt_handler.set_enabled_interrupts_flag(value),
                _ => ()
            }
            _ => {}
        }
    }


    fn get_byte(&self, address: u16) -> Option<u8> {
        match (address >> 8) as u8 {
            0x00 if self.bootrom.is_active() => {
                Some(self.bootrom[address])
            },
            0x00..=0x3f => Some(self.cartridge.read_rom(address)),
            0x40..=0x7f => Some(self.cartridge.read_rom(address)),
            0x80..=0x97 => self.gpu.read_memory(address),
            0x98..=0x9b => self.gpu.read_memory(address),
            0x9c..=0x9f => self.gpu.read_memory(address),
            0xa0..=0xbf => Some(self.cartridge.read_ram(address)),
            0xc0..=0xcf => Some(self.work_ram.read_lower(address)),
            0xd0..=0xdf => Some(self.work_ram.read_upper(address)),

            0xe0..=0xef => Some(self.work_ram.read_lower(address)),
            0xf0..=0xfd => Some(self.work_ram.read_upper(address)),
            0xfe => {
                match address & 0xff {
                    0x00..=0x9f => Some(self.gpu.read_oam(address as u8)),
                    _ => panic!("Unsupported read at ${:04x}", address),
                }
            }
            0xff => {
                match address as u8 {
                    0x00 => Some(self.input_controller.read_register()), //Joypad
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
                    0x46 => Some(self.dma.source),
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
        }
    }

    fn step(&mut self) {
        self.interrupt_handler.step();
    }
}
