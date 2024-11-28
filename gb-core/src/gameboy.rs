use crate::cpu::address::Cpu;
use crate::cpu::CpuState;
use crate::hardware::boot_rom::Bootrom;
use crate::hardware::cartridge::Cartridge;
use crate::hardware::input::Button;
use crate::hardware::ppu::PPuState;
use crate::hardware::{Hardware, HardwareState, Screen};

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

pub const SCREEN_HEIGHT: usize = 144;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameBoyState {
    pub cpu_state: CpuState,
    pub ppu_state: PPuState,
    pub hard_ware_state: HardwareState,
}
pub struct GameBoy<'a, S: Screen> {
    pub cpu: Cpu<Hardware<'a, S>>,
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
            cpu.interface.interrupt_handler.reset();
        }
        cpu.handle_return(cpu.registers.pc);
        GameBoy { cpu }
    }

    pub fn create_from_state(
        screen: S,
        cartridge: Box<dyn Cartridge + 'a>,
        mut boot_rom: Bootrom,
        player: Box<dyn crate::hardware::sound::AudioPlayer>,
        state: GameBoyState,
    ) -> GameBoy<S> {
        boot_rom.deactivate();
        let hardware = Hardware::create_from_state(
            screen,
            cartridge,
            boot_rom,
            player,
            state.hard_ware_state,
            state.ppu_state,
        );
        let cpu = Cpu::new_from_state(hardware, state.cpu_state);

        GameBoy { cpu }
    }
}

impl<'a, S: Screen> GameBoy<'a, S> {
    pub fn tick(&mut self) -> u8 {
        let cycles = self.cpu.step();
        if cycles != 0 {
            let interrupts = &mut self.cpu.interface.interrupt_handler;
            self.cpu.interface.input_controller.update_state(interrupts);
            self.cpu.interface.timer.do_cycle(cycles as u32, interrupts);
            self.cpu.interface.gpu.step(cycles as isize, interrupts);
            self.cpu.interface.sound.do_cycle(cycles as u32);
            self.cpu.interface.cartridge.step();
        }

        cycles
    }

    pub fn create_state(&self) -> GameBoyState {
        GameBoyState {
            cpu_state: self.cpu.create_state(),
            ppu_state: self.cpu.interface.gpu.create_state(),
            hard_ware_state: self.cpu.interface.create_state(),
        }
    }

    pub fn get_screen(&mut self) -> &mut S {
        &mut self.cpu.interface.gpu.screen
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
