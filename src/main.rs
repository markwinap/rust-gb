pub mod gl_screen;

extern crate glium;
extern crate gb_core;

use crate::gl_screen::{GlScreen, render};
use gb_core::hardware::Screen;
use gb_core::gameboy::GameBoy;
use std::sync::{Arc, RwLock};
use std::fs::File;
use std::io::Read;
use gb_core::hardware::rom::Rom;
use gb_core::hardware::boot_rom::{BootromData, Bootrom};
use std::ops::{Deref, DerefMut};

fn main() {
    construct_cpu()
}


pub fn construct_cpu() {
    let boot_rom = std::path::PathBuf::from("C:\\gbrom\\dmg_boot.bin");
    let rom = std::path::PathBuf::from("C:\\gbrom\\tetris.gb");
    let mut eventloop = glium::glutin::event_loop::EventLoop::new();
    let mut gl_screen = GlScreen::init("foo".to_string(), &mut eventloop);

    let rom_bytes = std::path::PathBuf::from(rom);
    let mut data:Vec<u8> = vec![];
    let file_rom = File::open(&rom_bytes).and_then(|mut f| f.read_to_end(&mut data)).map_err(|_| "Could not read ROM").unwrap();
   // let fjf = data.as_slice()
    let cart =Rom::from_bytes(Arc::new(data).clone());

    let gl_screenArc = Arc::new(RwLock::new(gl_screen));

    let mut file = File::open(boot_rom).unwrap();
    let mut data = Box::new(BootromData::new());
    file.read_exact(&mut (data.deref_mut()).0).unwrap();
    let boot_room_stuff = Bootrom::new(Some(Arc::new(*data)));
    let gameboy = GameBoy::create(gl_screenArc.clone(), Box::new(cart),boot_room_stuff);
    // let thread_handler = render(gl_screen);
    //  thread_handler.join();
    render(gl_screen);
}
