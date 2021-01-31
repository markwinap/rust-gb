use crate::cpu::address::Cpu;
use crate::hardware::{Screen, Hardware};
use crate::hardware::cartridge::Cartridge;
use crate::hardware::boot_rom::Bootrom;
use crate::hardware::input::{Controller, Button};

pub const SCREEN_HEIGHT: usize = 144;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

pub struct GameBoy<S: Screen, C: Controller> {
    pub cpu: Cpu<Hardware<S, C>>,
    elapsed_cycles: usize,
}

impl<S: Screen, C: Controller> GameBoy<S, C> {
    pub fn create(screen: S, controller: C, cartridge: Box<dyn Cartridge>, boot_rom: Bootrom) -> GameBoy<S, C> {
        let run_reset =  !boot_rom.is_active();
        let hardware = Hardware::create(screen,controller,  cartridge, boot_rom);
        let mut cpu = Cpu::new(hardware);

        if run_reset{
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


impl<S: Screen, C: Controller> GameBoy<S, C> {
    pub fn tick(&mut self) -> u8{
        if !self.cpu.interface.bootrom.is_active() && self.cpu.registers.pc > 634{
     //       println!("current opcode: {:#04X?}, current pc: {}", self.cpu.op_code, self.cpu.registers.pc);
        }
        let cycles = self.cpu.step();
       // println!("Current PC: {}", self.cpu.registers.pc);
        let interrupts = &mut self.cpu.interface.interrupt_handler;
        self.cpu.interface.input_controller.update_state(interrupts);
        self.cpu.interface.timer.do_cycle(cycles as u32, interrupts);
        self.cpu.interface.gpu.step(cycles as isize, interrupts);
        self.cpu.interface.cartridge.step();

        cycles
    }
}

pub enum GbEvents {
    KeyUp(Button),
    KeyDown(Button)
}