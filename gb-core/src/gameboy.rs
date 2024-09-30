use crate::cpu::address::Cpu;
use crate::hardware::boot_rom::Bootrom;
use crate::hardware::cartridge::Cartridge;
use crate::hardware::input::Button;
use crate::hardware::{Hardware, Screen};
use alloc::boxed::Box;

pub const SCREEN_HEIGHT: usize = 144;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

pub struct GameBoy<'a, S: Screen> {
    pub cpu: Cpu<Hardware<'a, S>>,
    elapsed_cycles: usize,
}

impl<'a, S: Screen> GameBoy<'a, S> {
    pub fn create(
        screen: S,
        cartridge: Box<dyn Cartridge + 'a>,
        boot_rom: Bootrom,
        player: Box<dyn crate::hardware::sound::AudioPlayer>,
    ) -> GameBoy<S> {
        let run_reset = !boot_rom.is_active();
        let hardware = Hardware::create(screen, cartridge, boot_rom, player);
        let mut cpu = Cpu::new(hardware);

        if run_reset {
            cpu.reset();
            cpu.interface.gpu.reset();
        }
        cpu.handle_return(cpu.registers.pc);
        GameBoy {
            cpu,
            elapsed_cycles: 0,
        }
    }
}

impl<'a, S: Screen> GameBoy<'a, S> {
    pub fn tick(&mut self) -> u8 {
        let cycles = self.cpu.step();
        let interrupts = &mut self.cpu.interface.interrupt_handler;
        self.cpu.interface.input_controller.update_state(interrupts);
        self.cpu.interface.timer.do_cycle(cycles as u32, interrupts);
        self.cpu.interface.gpu.step(cycles as isize, interrupts);
        self.cpu.interface.sound.do_cycle(cycles as u32);
        self.cpu.interface.cartridge.step();
        cycles
    }

    pub fn get_screen(&mut self) -> &mut S {
        &mut self.cpu.interface.gpu.screen
    }

    #[cfg(feature = "debug")]
    pub fn tick(&mut self) -> u8 {
        let mut cycles = self.cpu.step();
        if self.cpu.state == Step::Interrupt {
            cycles += self.cpu.step();
        }
        let interrupts = &mut self.cpu.interface.interrupt_handler;
        self.cpu.interface.input_controller.update_state(interrupts);
        self.cpu.interface.timer.do_cycle(cycles as u32, interrupts);
        self.cpu.interface.gpu.step(cycles as isize, interrupts);
        self.cpu.interface.cartridge.step();
        cycles
    }

    pub fn key_pressed(&mut self, button: Button) {
        self.cpu.interface.input_controller.key_pressed(button);
    }

    pub fn key_released(&mut self, button: Button) {
        self.cpu.interface.input_controller.key_released(button);
    }
}

pub enum GbEvents {
    KeyUp(Button),
    KeyDown(Button),
}
