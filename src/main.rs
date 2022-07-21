pub mod gl_screen;
mod fb_screen;

extern crate glium;
extern crate gb_core;

use crate::gl_screen::{GlScreen, render};
use gb_core::hardware::Screen;
use gb_core::gameboy::{GameBoy, SCREEN_PIXELS, SCREEN_WIDTH, GbEvents};
use std::sync::{Arc, mpsc};
use std::fs::File;
use std::io::Read;
use gb_core::hardware::rom::Rom;
use gb_core::hardware::boot_rom::{BootromData, Bootrom};
use std::ops::{DerefMut};
use gb_core::hardware::color_palette::Color;
use std::sync::mpsc::{SyncSender, Receiver, TryRecvError};
use std::cell::RefCell;
use std::fs;
use std::path::{Path};

fn main() {
    construct_cpu()
}

pub fn load_rom(zip_file: &str, rom_name: &str) -> Rom<'static> {
    println!("sd {}", 3);

    let foo = format!("sd {}", 3);
    let file = fs::File::open(&zip_file).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    let bytes = match archive.by_name(rom_name) {
        Ok(rom_file) => {
            rom_file.bytes()
        }
        Err(_) => { panic!() }
    };
    let data: Result<Vec<_>, _> = bytes.collect();
    Rom::from_bytes(Box::leak(Box::new(data.unwrap())))
}

pub fn load_rom_from_path(path: &Path) -> Rom<'static> {
    let mut gb_rom: Vec<u8> = vec![];
    File::open(path).and_then(|mut f| f.read_to_end(&mut gb_rom)).map_err(|_| "Could not read ROM").unwrap();
    Rom::from_bytes(Box::leak(Box::new(gb_rom)))
}

pub fn construct_cpu() {
    let gb_rom = load_rom_from_path(&std::path::PathBuf::from("/home/plozano/gbrom/sml.gb"));
    // let gb_rom = load_rom("test-roms/cpu_instrs.zip", "cpu_instrs/cpu_instrs.gb");
    let boot_rom = std::path::PathBuf::from("/home/plozano/gbrom/dmg_boot.bin");


    let (sender2, receiver2) = mpsc::sync_channel::<Box<[u8; SCREEN_PIXELS]>>(1);
    let (control_sender, control_receiver) = mpsc::channel::<GbEvents>();
    let gl_screen = GlScreen::init("foo".to_string(), receiver2);

    let sync_screen = SynScreen { sender: sender2, off_screen_buffer: RefCell::new(Box::new([0; SCREEN_PIXELS])) };

    let boot_room_stuff = Bootrom::new(Some(BootromData::from_bytes(include_bytes!("/home/plozano/gbrom/dmg_boot.bin"))));

    let cputhread = std::thread::spawn(move || {
        let periodic = timer_periodic(16);
        let mut limit_speed = true;

        let waitticks = (4194304f64 / 1000.0 * 16.0).round() as u32;
        let mut ticks = 0;

        let cart = gb_rom.into_cartridge();
        let mut gameboy = GameBoy::create(sync_screen, cart, boot_room_stuff);


        'outer: loop {
            while ticks < waitticks {
                ticks += gameboy.tick() as u32
            }

            ticks -= waitticks;

            'recv: loop {
                match control_receiver.try_recv() {
                    Ok(event) => {
                        match event {
                            GbEvents::KeyUp(key) => gameboy.key_released(key),
                            GbEvents::KeyDown(key) => gameboy.key_pressed(key),
                        }
                    }
                    Err(TryRecvError::Empty) => break 'recv,
                    Err(TryRecvError::Disconnected) => break 'outer,
                }
            }
            if limit_speed { let _ = periodic.recv(); }
        }
    });

    render(gl_screen, control_sender);
    cputhread.join().unwrap();
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
    fn turn_on(&mut self) {}

    fn turn_off(&mut self) {}

    fn set_pixel(&mut self, x: u8, y: u8, color: Color) {
        self.off_screen_buffer.get_mut()[y as usize * SCREEN_WIDTH * 3 + x as usize * 3 + 0] = color.red;
        self.off_screen_buffer.get_mut()[y as usize * SCREEN_WIDTH * 3 + x as usize * 3 + 1] = color.green;
        self.off_screen_buffer.get_mut()[y as usize * SCREEN_WIDTH * 3 + x as usize * 3 + 2] = color.blue;
    }

    fn draw(&mut self, skip: bool) {
        let stuff = self.off_screen_buffer.replace(Box::new([0; SCREEN_PIXELS]));
        self.sender.send(stuff).unwrap();
    }
}

