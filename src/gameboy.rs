use crate::cpu::address::Cpu;
use crate::hardware::{Screen, Hardware};
use crate::hardware::cartridge::Cartridge;
use crate::hardware::boot_rom::Bootrom;

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
    pub fn tick(&mut self) {
        let cycles = self.cpu.step();
        let interrupts = &mut self.cpu.interface.interrupt_handler;
        self.cpu.interface.timer.do_cycle(cycles as u32, interrupts);
        self.cpu.interface.gpu.step(cycles as isize, interrupts);
        self.cpu.interface.cartridge.step();
    }
}