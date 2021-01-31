pub mod gl_screen;
mod fb_screen;

extern crate glium;
extern crate gb_core;



use crate::gl_screen::{GlScreen, render};
use gb_core::hardware::Screen;
use gb_core::gameboy::{GameBoy, SCREEN_PIXELS, SCREEN_WIDTH, GbEvents};
use std::sync::{Arc, RwLock, mpsc};
use std::fs::File;
use std::io::Read;
use gb_core::hardware::rom::Rom;
use gb_core::hardware::boot_rom::{BootromData, Bootrom};
use std::ops::{Deref, DerefMut};
use gb_core::hardware::color_palette::Color;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver, TryRecvError};
use std::cell::RefCell;
use std::time::Duration;
use crate::fb_screen::FbScreen;
use gb_core::hardware::input::{Controller, Button};
use std::collections::HashMap;

fn main() {
    construct_cpu()
}


pub fn construct_cpu() {
    let boot_rom = std::path::PathBuf::from("C:\\gbrom\\dmg_boot.bin");
    let rom = std::path::PathBuf::from("C:\\gbrom\\tetris.gb");
    let (sender2, receiver2) = mpsc::sync_channel::<Box<[u8; SCREEN_PIXELS]>>(1);

    let (controlSender, controlReceiver) = mpsc::channel::<GbEvents>();

    let mut eventloop = glium::glutin::event_loop::EventLoop::new();
    let gl_screen = GlScreen::init("foo".to_string(), &mut eventloop, receiver2);
    //let fb_screen = FbScreen::init("".to_string(), receiver2);

    let sync_screen = SynScreen { sender: sender2, off_screen_buffer: RefCell::new(Box::new([0; SCREEN_PIXELS])) };
    let rom_bytes = std::path::PathBuf::from(rom);
    let mut data: Vec<u8> = vec![];
    File::open(&rom_bytes).and_then(|mut f| f.read_to_end(&mut data)).map_err(|_| "Could not read ROM").unwrap();

    let mut file = File::open(boot_rom).unwrap();
    let mut data2 = Box::new(BootromData::new());
    file.read_exact(&mut (data2.deref_mut()).0).unwrap();
    let boot_room_stuff = Bootrom::new(Some(Arc::new(*data2)));


    let cputhread = std::thread::spawn(move || {
        let periodic = timer_periodic(16);
        let mut limit_speed = true;

        let waitticks = (4194304f64 / 1000.0 * 16.0).round() as u32;
        let mut ticks = 0;

        let rom = Rom::from_bytes(Arc::new(data).clone());
        let rom_type = rom.rom_type;
        let cart = rom_type.to_cartridge(&rom);
        let mut gameboy = GameBoy::create(sync_screen, DummyController::new(), cart, boot_room_stuff);


        'outer: loop {
            while ticks < waitticks {
                ticks += gameboy.tick() as u32
            }

            ticks -= waitticks;

            'recv: loop {
                match controlReceiver.try_recv() {
                    Ok(event) => {
                       // println!("KEY DETECTED");
                        match event {
                            GbEvents::KeyUp(key) => gameboy.cpu.interface.input_controller.controller.key_released(key),
                            GbEvents::KeyDown(key) =>  gameboy.cpu.interface.input_controller.controller.key_pressed(key),

                        }
                    },
                    Err(TryRecvError::Empty) => break 'recv,
                    Err(TryRecvError::Disconnected) => break 'outer,
                }
            }
            if limit_speed { let _ = periodic.recv(); }
        }
    });

    render(gl_screen, controlSender);
    //FbScreen::render(fb_screen);
    cputhread.join();
}



fn timer_periodic(ms: u64) -> Receiver<()> {
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(ms));
            if tx.send(()).is_err() {
                break;
            }
        }
    });
    rx
}

pub struct SynScreen {
    sender: SyncSender<Box<[u8; SCREEN_PIXELS]>>,
    off_screen_buffer: RefCell<Box<[u8; SCREEN_PIXELS]>>,
}


impl Screen for SynScreen {
    fn turn_on(&mut self) {
        //  unimplemented!()
    }

    fn turn_off(&mut self) {}

    fn set_pixel(&mut self, x: u8, y: u8, color: Color) {
        self.off_screen_buffer.get_mut()[y as usize * SCREEN_WIDTH * 3 + x as usize * 3 + 0] = color.red;
        self.off_screen_buffer.get_mut()[y as usize * SCREEN_WIDTH * 3 + x as usize * 3 + 1] = color.green;
        self.off_screen_buffer.get_mut()[y as usize * SCREEN_WIDTH * 3 + x as usize * 3 + 2] = color.blue;
    }

    fn draw(&mut self) {
        let stuff = self.off_screen_buffer.replace(Box::new([0; SCREEN_PIXELS]));
        self.sender.send(stuff);
    }
}

struct DummyController {
    state: HashMap<Button, bool>
}

impl DummyController {
    pub fn new() -> Self {
        Self {
            state: HashMap::new()
        }
    }

    pub fn key_pressed(&mut self, button: Button) {
      //  println!("Key press!!");
        self.state.insert(button, true);
    }

    pub fn key_released(&mut self, button: Button) {
        self.state.insert(button, false);
    }
}
impl Controller for DummyController {

    fn is_pressed(&self, button: Button) -> bool {
       let result = match self.state.get(&button) {
            Some(value) => *value,
            None => false
        };
        // if result {
        //     println!("{} : {}", button, result);
        // }

        result
    }

    fn tick(&self) {}
}