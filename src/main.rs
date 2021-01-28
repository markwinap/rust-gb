pub mod gl_screen;

extern crate glium;
extern crate gb_core;

use crate::gl_screen::{GlScreen, render};
use gb_core::hardware::Screen;
use gb_core::gameboy::{GameBoy, SCREEN_PIXELS, SCREEN_WIDTH};
use std::sync::{Arc, RwLock, mpsc};
use std::fs::File;
use std::io::Read;
use gb_core::hardware::rom::Rom;
use gb_core::hardware::boot_rom::{BootromData, Bootrom};
use std::ops::{Deref, DerefMut};
use gb_core::hardware::color_palette::Color;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::cell::RefCell;
use std::time::Duration;

fn main() {
    construct_cpu()
}


pub fn construct_cpu() {
    let boot_rom = std::path::PathBuf::from("C:\\gbrom\\dmg_boot.bin");
    let rom = std::path::PathBuf::from("C:\\gbrom\\tetris.gb");
    let mut eventloop = glium::glutin::event_loop::EventLoop::new();


    let (sender2, receiver2) = mpsc::sync_channel::<Box<[u8; SCREEN_PIXELS]>>(1);
    let mut gl_screen = GlScreen::init("foo".to_string(), &mut eventloop, receiver2);
    let sync_screen = SynScreen { sender: sender2, off_screen_buffer: RefCell::new(Box::new([0; SCREEN_PIXELS])) };
    let rom_bytes = std::path::PathBuf::from(rom);
    let mut data: Vec<u8> = vec![];
    let file_rom = File::open(&rom_bytes).and_then(|mut f| f.read_to_end(&mut data)).map_err(|_| "Could not read ROM").unwrap();
    // let fjf = data.as_slice()

   // let gl_screenArc = Arc::new(RwLock::new(gl_screen));

    let mut file = File::open(boot_rom).unwrap();
    let mut data2 = Box::new(BootromData::new());
    file.read_exact(&mut (data2.deref_mut()).0).unwrap();
    let boot_room_stuff = Bootrom::new(Some(Arc::new(*data2)));

    // let thread_handler = render(gl_screen);
    //  thread_handler.join();

    let cputhread = std::thread::spawn(move|| {
        let rom = Rom::from_bytes(Arc::new(data).clone());
        let rom_type = rom.rom_type;
        let cart = rom_type.to_cartridge(&rom);
        let mut gameboy = GameBoy::create(sync_screen, cart, boot_room_stuff);
        while true {
         //   println!("In loop");
            //std::thread::sleep(Duration::from_millis(2));
            gameboy.tick()
        }
    });

    render(gl_screen);

    cputhread.join();

}

pub struct SynScreen {
    sender: SyncSender<Box<[u8; SCREEN_PIXELS]>>,
    off_screen_buffer: RefCell<Box<[u8; SCREEN_PIXELS]>>,
}

impl SynScreen {
    fn index(x: u8, y: u8) -> usize {
        3 * ((y as usize * SCREEN_WIDTH) + x as usize)
    }
}
impl Screen for SynScreen {
    fn turn_on(&mut self) {
        //  unimplemented!()
    }

    fn turn_off(&mut self) {
    }

    fn set_pixel(&mut self, x: u8, y: u8, color: Color) {

      //  let index = SynScreen::index(x, y);
        println!("Setting pixel! x: {}, y: {}", x, y);
        // self.off_screen_buffer.get_mut()[index] = color.red;
        // self.off_screen_buffer.get_mut()[index + 1] = color.green;
        // self.off_screen_buffer.get_mut()[index + 2] = color.blue;
        println!("Calculated location: {}", y as usize * SCREEN_WIDTH * 3 + x as usize * 3 + 0);
        self.off_screen_buffer.get_mut()[y as usize * SCREEN_WIDTH * 3 + x as usize * 3 + 0] = color.red;;
        self.off_screen_buffer.get_mut()[y as usize * SCREEN_WIDTH * 3 + x as usize * 3 + 1] = color.green;
        self.off_screen_buffer.get_mut()[y as usize * SCREEN_WIDTH * 3 + x as usize * 3 + 2] = color.blue;
    }

    fn draw(&mut self) {
        let stuff = self.off_screen_buffer.replace(Box::new([0; SCREEN_PIXELS]));
        self.sender.send(stuff);
    }
}