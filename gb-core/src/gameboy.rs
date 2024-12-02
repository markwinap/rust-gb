use crate::cpu::address::Cpu;
use crate::cpu::opcodes::DecodeStep;
use crate::cpu::{CpuState, Interface, Step};
use crate::hardware::boot_rom::Bootrom;
use crate::hardware::cartridge::Cartridge;
use crate::hardware::input::Button;
use crate::hardware::ppu::PPuState;
use crate::hardware::{Hardware, HardwareState, Screen};
use crate::is_log_enabled;

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
    pub state: Step,
}
pub struct GameBoy<'a, S: Screen> {
    pub cpu: Cpu<Hardware<'a, S>>,
    pub state: Step,
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

        GameBoy {
            cpu,
            state: Step::Run,
        }
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

        GameBoy {
            cpu,
            state: state.state,
        }
    }
}

impl<'a, S: Screen> GameBoy<'a, S> {
    pub fn tick(&mut self) -> u8 {
        let (cycles, decode_step) = self.cpu.step(self.state);
        if cycles != 0 {
            let interrupts = &mut self.cpu.interface.interrupt_handler;
            interrupts.step();
            self.cpu.interface.input_controller.update_state(interrupts);
            self.cpu.interface.timer.do_cycle(cycles as u32, interrupts);
            self.cpu.interface.gpu.step(cycles as isize, interrupts);
            self.cpu.interface.sound.do_cycle(cycles as u32);
            self.cpu.interface.cartridge.step();
        }
        let next_state = match decode_step {
            DecodeStep::Run => {
                if self.cpu.interface.interrupt_master_enabled() && self.cpu.interface.any_enabled()
                {
                    Step::Interrupt
                } else {
                    Step::Run
                }
            }
            DecodeStep::Halt => {
                if self.cpu.interface.any_enabled() {
                    if self.cpu.interface.interrupt_master_enabled() {
                        Step::Interrupt
                    } else {
                        Step::HaltBug
                    }
                } else {
                    Step::Halt
                }
            }
            DecodeStep::Stopped => Step::Stopped,
        };
        self.state = next_state;
        cycles
    }

    pub fn create_state(&self) -> GameBoyState {
        GameBoyState {
            cpu_state: self.cpu.create_state(),
            ppu_state: self.cpu.interface.gpu.create_state(),
            hard_ware_state: self.cpu.interface.create_state(),
            state: self.state,
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
