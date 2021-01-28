use crate::cpu::address::Cpu;
use crate::hardware::{Screen, Hardware};
use crate::hardware::cartridge::Cartridge;
use crate::hardware::boot_rom::Bootrom;

pub const SCREEN_HEIGHT: usize = 144;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

pub struct GameBoy<S: Screen> {
    cpu: Cpu<Hardware<S>>,
    elapsed_cycles: usize,
}

impl<S: Screen> GameBoy<S> {
    pub fn create(screen: S, cartridge: Box<dyn Cartridge>, boot_rom: Bootrom) -> GameBoy<S> {
        let hardware = Hardware::create(screen, cartridge, boot_rom);
        let cpu = Cpu::new(hardware);
        GameBoy {
            cpu,
            elapsed_cycles: 0,
        }
    }
}


impl<S: Screen> GameBoy<S> {
    pub fn tick(&mut self) -> u8{
        let cycles = self.cpu.step();
       // println!("Current PC: {}", self.cpu.registers.pc);
        let interrupts = &mut self.cpu.interface.interrupt_handler;
        self.cpu.interface.timer.do_cycle(cycles as u32, interrupts);
        self.cpu.interface.gpu.step(cycles as isize, interrupts);
        self.cpu.interface.cartridge.step();
        cycles
    }
}